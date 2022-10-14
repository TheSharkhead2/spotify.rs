use base64;
use getrandom;
use json;
use open;
use querystring::{querify, stringify};
use random_string;
use reqwest;
use sha2::{Digest, Sha256};
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
use urlencoding::encode;

/// Generates the code verifier and code challenge for PKCE
///
/// # Panics
/// When random number generation fails. See [get random docs](https://docs.rs/getrandom/latest/getrandom/#functions)
///
pub fn generate_verifier() -> (String, String) {
    let mut buf = [0u8; 32]; // empty list of 32 bytes

    getrandom::getrandom(&mut buf).unwrap(); // generate random bytes - unwrap to panic on random failure

    let code_verifier = base64::encode_config(buf, base64::URL_SAFE).replace("=", ""); // encode bytes into url safe base64 string and remove tailing equal sign

    let mut code_challenge_hasher = Sha256::new(); // new hashing object to create code challenge
    code_challenge_hasher.update(&code_verifier); // add code verifier to hash
    let code_challenge_raw = code_challenge_hasher.finalize(); // finalize hash of code verifier

    let code_challenge =
        base64::encode_config(code_challenge_raw, base64::URL_SAFE).replace("=", ""); // encode bytes into url safe base64 string and remove tailing equal sign

    (code_verifier, code_challenge)
}

/// Full code flow for getting authorization code from Spotify to authenticate API use.
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
pub fn get_authorization_code(
    client_id: &str,
    localhost_port: &str,
    redirect_uri: &str,
    scope: &str,
    code_challenge: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let authorization_code_endpoint = "https://accounts.spotify.com/authorize?".to_owned(); // authorization code endpoint
    let character_set = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"; // character set for random string

    let state = random_string::generate(16, character_set); // generate random string for state variable

    let encoded_redirect_uri = encode(&redirect_uri).into_owned(); // encode redirect uri for url

    // define parameters for authorization code request
    let parameters = vec![
        ("response_type", "code"),
        ("client_id", client_id),
        ("redirect_uri", &encoded_redirect_uri),
        ("scope", scope),
        ("show_dialog", "true"),
        ("state", &state),
        ("code_challenge", code_challenge),
        ("code_challenge_method", "S256"),
    ];

    let query_parameters = stringify(parameters); // stringify parameters

    let auth_url = authorization_code_endpoint + &query_parameters; // create authorization url

    // open authorization url in browser for user to authorize application
    match open::that(auth_url) {
        Ok(()) => println!("Opened authorization url in browser"),
        Err(e) => panic!("Failed to open authorization url in browser: {}", e), // panic on inability to open browser (can't authentiate)
    }

    // listen for authorization code from redirect uri and parse option result
    return listen_for_auth_code(localhost_port, &state);
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
                let contents = fs::read_to_string("src/authorization_successful.html").unwrap(); // read html file to display to user
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

/// Returns `(access_tokem, refresh_token, expires_in)` tuple. `access_token` is used to access API, `refresh_token` is used to
/// refresh `access_token` when it expires, and `expires_in` is the number of seconds until `access_token` expires.  
///
/// # Arguments
/// * `authorization_code` - The authorization code received from the authorization request
/// * `client_id` - The client id of the application
/// * `code_verifier` - The code verifier used in the authorization request
/// * `redirect_uri` - The redirect uri used in the authorization request
///
/// # Panics
/// * On request error (to Spotify API)
/// * On error parsing expires_in from response to int (shouldn't happen)
///
pub fn get_access_token(
    authorization_code: &str,
    client_id: &str,
    code_verifier: &str,
    redirect_uri: &str,
) -> Result<(String, String, i64), Box<dyn std::error::Error>> {
    let request_uri = "https://accounts.spotify.com/api/token?"; // token request uri

    let client = reqwest::blocking::Client::new();

    let encoded_redirect_uri = encode(&redirect_uri).into_owned(); // encode redirect uri for url

    let query_parameters = vec![
        ("grant_type", "authorization_code"),
        ("code", authorization_code),
        ("redirect_uri", &encoded_redirect_uri),
        ("client_id", client_id),
        ("code_verifier", code_verifier),
    ];

    let query_string = stringify(query_parameters); // stringify query parameters

    let response = client
        .post(String::from(request_uri) + &query_string)
        .header("Content-Type", "application/x-www-form-urlencoded") // set Content-Type header
        .header("Content-Length", "0") // set Content-Length header
        .send()?; // send request

    if response.status().is_success() {
        // check if response is successful
        let response_body = json::parse(&response.text().unwrap()).unwrap(); // get response as json

        let access_token = response_body["access_token"].to_string(); // get access token from response
        let refresh_token = response_body["refresh_token"].to_string(); // get refresh token from response
        let expires_in_str = response_body["expires_in"].to_string(); // get expires in from response
        let expires_in: i64 = expires_in_str.parse().unwrap(); // parse expires in to i64

        return Ok((access_token, refresh_token, expires_in)); // return access token, refresh token, and expires in
    } else {
        return Err(format!("Error: {}", response.status()).into()); // return error if response is not successful
    }
}

/// Requests new refresh token from Spotify API. Returns new refresh token and time until it expires
///
/// # Arguments
/// * `refresh_token` - The refresh token used to request a new refresh token
/// * `client_id` - The client id of the application
///
pub fn refresh_access_token(
    refresh_token: &str,
    client_id: &str,
) -> Result<(String, i64), Box<dyn std::error::Error>> {
    let request_uri = "https://accounts.spotify.com/api/token?"; // token request uri

    let client = reqwest::blocking::Client::new();

    let query_parameters = vec![
        ("grant_type", "refresh_token"),
        ("refresh_token", refresh_token),
        ("client_id", client_id),
    ];

    let query_string = stringify(query_parameters); // stringify query parameters

    let response = client
        .post(String::from(request_uri) + &query_string)
        .header("Content-Type", "application/x-www-form-urlencoded") // set Content-Type header
        .header("Content-Length", "0") // set Content-Length header
        .send()?; // send request

    if response.status().is_success() {
        // check if response is successful
        let response_body = json::parse(&response.text().unwrap()).unwrap(); // get response as json

        let access_token = response_body["access_token"].to_string(); // get access token from response
        let expires_in_str = response_body["expires_in"].to_string(); // get expires in from response
        let expires_in: i64 = expires_in_str.parse().unwrap(); // parse expires in to i64

        return Ok((access_token, expires_in)); // return access token and expires in
    } else {
        return Err(format!("Error: {}", response.status()).into()); // return error if response is not successful
    }
}
