use dotenv;
use reqwest::{self, redirect};
use base64;
use getrandom;
use sha2::{Sha256, Digest};
use random_string;
use querystring::{stringify, querify};
use open;
use urlencoding::encode;
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

/// Generates the code verifier and code challenge for PKCE 
/// 
/// # Panics
/// When random number generation fails. See [get random docs](https://docs.rs/getrandom/latest/getrandom/#functions)
fn generate_verifier() -> (String, String) {
    let mut buf = [0u8; 32]; // empty list of 32 bytes 

    getrandom::getrandom(&mut buf).unwrap(); // generate random bytes - unwrap to panic on random failure 

    let code_verifier = base64::encode_config(buf, base64::URL_SAFE).replace("=",""); // encode bytes into url safe base64 string and remove tailing equal sign 

    let mut code_challenge_hasher = Sha256::new(); // new hashing object to create code challenge 
    code_challenge_hasher.update(&code_verifier); // add code verifier to hash 
    let code_challenge_raw = code_challenge_hasher.finalize(); // finalize hash of code verifier 

    let code_challenge = base64::encode_config(code_challenge_raw, base64::URL_SAFE).replace("=", ""); // encode bytes into url safe base64 string and remove tailing equal sign

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
fn get_authorization_code(client_id: &str, localhost_port: &str, redirect_uri: &str, scope: &str, code_challenge: &str) -> Result<String, Box<dyn std::error::Error>> {
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
    if let Some(auth_code) = listen_for_auth_code(localhost_port, &state) {
        println!("Received authorization code: {}", auth_code);
        Ok(auth_code) // return authorization code
    } else {
        Err("Failed to receive authorization code".into()) // if None is returned, unknown error has occured
    } 
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
fn listen_for_auth_code(port: &str, state: &str) -> Option<String> {
    let listener = TcpListener::bind(String::from("127.0.0.1:") + &port).unwrap(); // listen on specified port for localhost

    // on connection, process information for auth code
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        let auth_code = handle_connection(stream, &state); // handle connection and get auth code

        match auth_code {
            Some(result) => {
                match result {
                    Ok(code) => return Some(code),
                    Err(e) => panic!("Error: {}", e),
                }
            },
            None => continue,
        }

    }
    None
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
fn handle_connection(mut stream: TcpStream, state: &str) -> Option<Result<String, Box<dyn std::error::Error>>> {
    let buf_reader = BufReader::new(&mut stream);

    // read information from HTTP request and break into lines 
    let http_request = buf_reader.lines().next().unwrap().unwrap(); // Get request line from HTTP request

    let http_request_len = http_request.len(); // get length of http request

    // look for expected request
    if &http_request[0..13] == "GET /callback" && &http_request[(http_request_len-9)..] == " HTTP/1.1" {
        let query = querify(&http_request[14..http_request_len-9]); // get query parameters from request (from 14 to remove "GET /callback?" and to -9 to remove "HTTP/1.1")

        // check if state matches expected state
        if query[1].0 == "state" && query[1].1 == state {
            // check if authorization code is present
            if query[0].0 == "code" {
                let authorization_code = String::from(query[0].1); // get authorization code

                let status_line = "HTTP/1.1 200 OK"; // status line for success response
                let contents = fs::read_to_string("src/authorization_successful.html").unwrap(); // read html file to display to user
                let content_length = contents.len(); 

                // create response
                let response = format!(
                    "{status_line}\r\nContent-Length: {content_length}\r\n\r\n{contents}"
                );

                stream.write_all(response.as_bytes()).unwrap(); // write response to stream

                return Some(Ok(authorization_code)); // return authorization code
            } else if query[0].0 == "error" {
                return Some(Err(format!("Authorization error: {}", query[0].1).into())); // return authorization error
            } else {
                return Some(Err("Authorization error".into())) // on no code or error present, just error
            }
        } else {
            return Some(Err(format!("Invalid state. Expected {} got {}. Authorization failed", state, query[1].1).into())) // on invalid state, invalidate authorization
        }

    } else {
        return None; // return None if request is not expected
    }
}

/// Object that holds information relevant to PKCE authorization
pub struct ApplicationDetails {
    pub client_id: String,
    redirect_uri: String,
    scope: String,
    access_token: Option<String>,
    refresh_token: Option<String>,
    expires_at: Option<u32>,
    code_challenge: String,
    code_verifier: String,
}

impl ApplicationDetails {
    pub fn new(localhost_port: String, scope: String) -> ApplicationDetails {
        let client_id = dotenv::var("CLIENT_ID").unwrap(); // grab client_id from .env

        let (code_verifier, code_challenge) = generate_verifier(); // generate code verifier and code challenge

        let redirect_uri = format!("http://localhost:{}/callback", &localhost_port); // redirect uri for authorization code endpoint

        get_authorization_code(&client_id, &localhost_port, &redirect_uri, &scope, &code_challenge);

        ApplicationDetails {
            client_id: client_id,
            redirect_uri: redirect_uri,
            scope: scope,
            access_token: None,
            refresh_token: None,
            expires_at: None,
            code_challenge: code_challenge,
            code_verifier: code_verifier,
        }
    }
}

