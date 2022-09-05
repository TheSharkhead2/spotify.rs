use dotenv;
use reqwest::{self, redirect};
use base64;
use getrandom;
use sha2::{Sha256, Digest};
use random_string;
use querystring::stringify;

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

fn get_authorization_code(client_id: &str, redirect_uri: &str, scope: &str, code_challenge: &str) -> Result<(), Box<dyn std::error::Error>> {
    let authorization_code_endpoint = "https://accounts.spotify.com/authorize?".to_owned(); // authorization code endpoint
    let character_set = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"; // character set for random string

    let state = random_string::generate(16, character_set); // generate random string for state variable 

    // define parameters for authorization code request
    let parameters = vec![
        ("response_type", "code"),
        ("client_id", client_id),
        ("redirect_uri", redirect_uri),
        ("scope", scope),
        ("show_dialog", "true"),
        ("state", &state),
        ("code_challenge", code_challenge),
        ("code_challenge_method", "S256"),
    ];

    let query_parameters = stringify(parameters); // stringify parameters 

    let body = reqwest::blocking::get(authorization_code_endpoint + &query_parameters)?
        .text()?;

    println!("{}", body);

    Ok(())
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
    pub fn new(redirect_uri: String, scope: String) -> ApplicationDetails {
        let client_id = dotenv::var("CLIENT_ID").unwrap(); // grab client_id from .env

        let (code_verifier, code_challenge) = generate_verifier(); // generate code verifier and code challenge

        get_authorization_code(&client_id, &redirect_uri, &scope, &code_challenge);

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

