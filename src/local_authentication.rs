use chrono::{Duration, Utc};
use open;
use querystring::querify;
use std::fs;
use std::sync::RwLock;
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

use crate::authorization::{get_access_token, refresh_access_token, requesturl_authorization_code};
use crate::spotify::{Spotify, SpotifyError};

// html to show when authorization is successful
const AUTHORIZATION_SUCCESSFUL_HTML: &str = r###"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>Success</title>
  </head>
  <body>
    <h1>Success!</h1>
    <p>Thank you for authenticating with Spotify! You can close this page now.</p>
  </body>
</html>"###;

/// Full code flow for getting authorization code from Spotify to authenticate API use. Returns (auth_code_result, code_verifier).
///
/// # Arguments
/// * `client_id` - Spotify developer client id
/// * `localhost_port` - the port for localhost redirect. Redirect uri should be: http://localhost:{localhost_port}/callback
/// * `redirect_uri` - redirect_uri for request. Should be: http://localhost:{localhost_port}/callback
/// * `scope` - scope of permissions for the request. See [Spotify docs](https://developer.spotify.com/documentation/general/guides/scopes/) for more info
/// * `code_challenge` - code challenge for PKCE. See [Spotify docs](https://developer.spotify.com/documentation/general/guides/authorization-guide/#authorization-code-flow-with-proof-key-for-code-exchange-pkce) for more info
///
/// # Panics
/// When browser fails to open authentication url
///
fn get_authorization_code(
    client_id: &str,
    localhost_port: &str,
    redirect_uri: &str,
    scope: &str,
) -> (Result<String, Box<dyn std::error::Error>>, String) {
    // get request url for authorization code
    let (auth_url, state, code_verifier) =
        requesturl_authorization_code(client_id, redirect_uri, scope);

    // open authorization url in browser for user to authorize application
    match open::that(auth_url) {
        Ok(()) => println!("Opened authorization url in browser"),
        Err(e) => panic!("Failed to open authorization url in browser: {}", e), // panic on inability to open browser (can't authentiate)
    }

    // listen for authorization code from redirect uri and parse option result
    return (listen_for_auth_code(localhost_port, &state), code_verifier);
}

/// Listens on specified port for the authorization code utilizing `handle_connection()`. This is a modified version of code
/// from the [Rust handbook](https://doc.rust-lang.org/book/ch20-01-single-threaded.html). Returns Option<auth_code: String>.
///
/// # Arguments
///
/// * `port` - The port to listen on
/// * `state` - The state variable used in authorization request (used to authenticate authorization code)
///
/// # Panics
/// On any authorization error.
///
fn listen_for_auth_code(port: &str, state: &str) -> Result<String, Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(String::from("127.0.0.1:") + &port).unwrap(); // listen on specified port for localhost

    // on connection, process information for auth code
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        let auth_code = handle_connection(stream, &state); // handle connection and get auth code

        match auth_code {
            Some(result) => match result {
                Ok(code) => return Ok(code),
                Err(e) => return Err(e),
            },
            None => continue,
        }
    }
    Err("Failed to find authorization code.".into())
}

/// Handles connection to localhost port to do error handling/detection and state validation. Returns authorization code.
/// This code is a modified version of what appears in the [Rust handbook](https://doc.rust-lang.org/book/ch20-01-single-threaded.html).
///
/// # Arguments
///
/// * `stream` - TcpStream object to handle connection
/// * `state` - the state string used in the authorization request
///
/// # Panics
/// * When http request parsing is unsuccessful
/// * On error surrounding sending success webpage to user
///
fn handle_connection(
    mut stream: TcpStream,
    state: &str,
) -> Option<Result<String, Box<dyn std::error::Error>>> {
    let buf_reader = BufReader::new(&mut stream);

    // read information from HTTP request and break into lines
    let http_request = buf_reader.lines().next().unwrap().unwrap(); // Get request line from HTTP request

    let http_request_len = http_request.len(); // get length of http request

    // look for expected request
    if &http_request[0..13] == "GET /callback"
        && &http_request[(http_request_len - 9)..] == " HTTP/1.1"
    {
        let query = querify(&http_request[14..http_request_len - 9]); // get query parameters from request (from 14 to remove "GET /callback?" and to -9 to remove "HTTP/1.1")

        // check if state matches expected state
        if query[1].0 == "state" && query[1].1 == state {
            // check if authorization code is present
            if query[0].0 == "code" {
                let authorization_code = String::from(query[0].1); // get authorization code

                let status_line = "HTTP/1.1 200 OK"; // status line for success response
                let contents = AUTHORIZATION_SUCCESSFUL_HTML.to_string(); // read html file to display to user
                let content_length = contents.len();

                // create response
                let response =
                    format!("{status_line}\r\nContent-Length: {content_length}\r\n\r\n{contents}");

                stream.write_all(response.as_bytes()).unwrap(); // write response to stream

                return Some(Ok(authorization_code)); // return authorization code
            } else if query[0].0 == "error" {
                return Some(Err(format!("Authorization error: {}", query[0].1).into()));
            // return authorization error
            } else {
                return Some(Err("Authorization error".into())); // on no code or error present, just error
            }
        } else {
            return Some(Err(format!(
                "Invalid state. Expected {} got {}. Authorization failed",
                state, query[1].1
            )
            .into())); // on invalid state, invalidate authorization
        }
    } else {
        return None; // return None if request is not expected
    }
}

impl Spotify {
    /// Creates a new Spotify object by authenticating with the Spotify API using the PKCE codeflow.
    /// Grabs `client_id` from `.env` file.
    ///
    /// # Arguments
    /// * `localhost_port` - The localhost port fort the redirect uri. Note: currently there is only support for localhost redirect uris.
    /// * `scope` - The scope of the Spotify API. See <https://developer.spotify.com/documentation/general/guides/authorization/scopes/> for more information.
    ///
    pub fn authenticate(&self, localhost_port: String, scope: String) -> Result<(), SpotifyError> {
        let client_id = dotenv::var("CLIENT_ID").unwrap(); // grab client_id from .env

        let redirect_uri = format!("http://localhost:{}/callback", &localhost_port); // redirect uri for authorization code endpoint

        let (auth_code_result, code_verifier) =
            get_authorization_code(&client_id, &localhost_port, &redirect_uri, &scope);

        let (access_token, refresh_token, expires_in) = match auth_code_result {
            Ok(auth_code) => {
                get_access_token(&auth_code, &client_id, &code_verifier, &redirect_uri).unwrap()
                // get access token (be lazy with error handling and just panic if request is bad)
            }
            Err(e) => return Err(SpotifyError::AuthenticationError(e.to_string())),
        };

        let expires_at = Utc::now() + Duration::seconds(expires_in); // get time when access token expires

        // update all of the Spotify object's fields
        let mut self_client_id = self.client_id.write().unwrap();
        *self_client_id = Some(client_id);
        let mut self_scope = self.scope.write().unwrap();
        *self_scope = Some(scope);
        let mut self_access_token = self.access_token.write().unwrap();
        *self_access_token = Some(access_token);
        let mut self_refresh_token = self.refresh_token.write().unwrap();
        *self_refresh_token = Some(refresh_token);
        let mut self_expires_at = self.expires_at.write().unwrap();
        *self_expires_at = Some(expires_at);

        Ok(())
    }

    /// Saves necessary authorization information to file for later use
    ///
    /// # Arguments
    /// * `file_name` - The name of the file to save the authorization information to
    ///
    pub fn save_to_file(&self, file_name: &str) -> Result<(), SpotifyError> {
        let auth_data = self.auth_save_data()?;

        let data = format!("{}\n{}\n{}", auth_data.0, auth_data.1, auth_data.2,); // format data to be saved to file

        match fs::write(file_name, data) {
            // write data to file
            Ok(_) => Ok(()),
            Err(e) => Err(SpotifyError::FileError(e.to_string())),
        }
    }

    /// Creates a new autheticated object from file
    ///
    /// # Arguments
    /// * `file_name` - The name of the file to load the authorization information from
    ///
    /// # Panics
    /// Panics if the file doesn't contain the necessary information
    ///
    pub fn new_from_file(file_name: &str) -> Result<Spotify, SpotifyError> {
        let data = match fs::read_to_string(file_name) {
            Ok(data) => data,
            Err(_) => return Err(SpotifyError::NoFile), // assume no file
        };
        let mut lines = data.lines(); // get lines from data

        let client_id = lines.next().unwrap().to_string(); // get client id
        let scope = lines.next().unwrap().to_string(); // get scope
        let refresh_token = lines.next().unwrap().to_string(); // get refresh token

        let (access_token, expires_in, new_refresh_token) =
            refresh_access_token(&refresh_token, &client_id)?; // refresh access token. Panics if request is bad
        let expires_at = Utc::now() + Duration::seconds(expires_in); // get time when access token expires

        // return Spotify object
        Ok(Spotify {
            client_id: RwLock::new(Some(client_id)),
            scope: RwLock::new(Some(scope)),
            access_token: RwLock::new(Some(access_token)),
            refresh_token: RwLock::new(Some(new_refresh_token)),
            expires_at: RwLock::new(Some(expires_at)),
        })
    }

    /// Authorizes a blank Spotify object from a file
    ///
    /// # Arguments
    /// * `file_name` - The name of the file to load the authorization information from
    ///
    /// # Panics
    /// Panics if the file doesn't contain the necessary information
    ///
    pub fn authenticate_from_file(&self, file_name: &str) -> Result<(), SpotifyError> {
        let data = match fs::read_to_string(file_name) {
            Ok(data) => data,
            Err(_) => return Err(SpotifyError::NoFile), // assume no file
        };

        let mut lines = data.lines(); // get lines from data

        let client_id = lines.next().unwrap().to_string(); // get client id
        let scope = lines.next().unwrap().to_string(); // get scope
        let refresh_token = lines.next().unwrap().to_string(); // get refresh token

        let (access_token, expires_in, new_refresh_token) =
            refresh_access_token(&refresh_token, &client_id)?; // refresh access token. Panics if request is bad
        let expires_at = Utc::now() + Duration::seconds(expires_in); // get time when access token expires

        // set client id, scope, access token, refresh token, and expires at
        let mut self_client_id = self.client_id.write().unwrap();
        *self_client_id = Some(client_id);
        let mut self_scope = self.scope.write().unwrap();
        *self_scope = Some(scope);
        let mut self_access_token = self.access_token.write().unwrap();
        *self_access_token = Some(access_token);
        let mut self_refresh_token = self.refresh_token.write().unwrap();
        *self_refresh_token = Some(new_refresh_token);
        let mut self_expires_at = self.expires_at.write().unwrap();
        *self_expires_at = Some(expires_at);

        Ok(())
    }
}
