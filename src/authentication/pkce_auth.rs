use crate::authentication::auth_objects::{PkceAuth, PkcePreAuth};
use crate::authentication::AuthorizationJsonError;
use crate::requests::SpotifyStatus;
use crate::requests::{general_request, RequestMethod};
use crate::Error;

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use querystring::stringify;
use reqwest::header::{HeaderMap, CONTENT_LENGTH, CONTENT_TYPE};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use urlencoding::encode;

/// Struct representing JSON response for 200 OK status code response from requesting access token.
/// Response detailed at: https://developer.spotify.com/documentation/web-api/tutorials/code-pkce-flow
#[derive(Deserialize)]
struct PkceAccessTokenReponse {
    access_token: String,
    token_type: String,
    scope: String,
    expires_in: String,
    refresh_token: String,
}

/// Generates the code verifier and code challenge for PKCE
///
/// # Panics
/// When random number generation fails. See [get random docs](https://docs.rs/getrandom/latest/getrandom/#functions)
///
fn generate_verifier() -> (String, String) {
    let mut buf = [0u8; 32]; // empty list of 32 bytes

    getrandom::getrandom(&mut buf).unwrap(); // generate random bytes - unwrap to panic on random failure

    let code_verifier = URL_SAFE_NO_PAD.encode(buf); // encode bytes into url safe base64 string

    let mut code_challenge_hasher = Sha256::new(); // new hashing object to create code challenge
    code_challenge_hasher.update(&code_verifier); // add code verifier to hash
    let code_challenge_raw = code_challenge_hasher.finalize(); // finalize hash of code verifier

    let code_challenge = URL_SAFE_NO_PAD.encode(code_challenge_raw); // encode bytes into code_challenge base64 string

    (code_verifier, code_challenge)
}

/// Generates authentication url for PKCE authentication.
///
/// # Arguments
/// * `client_id` - The client id for the application (obtained through Spotify for the app you are building)
/// * `redirect_uri` - Where Spotify will redirect the user after they grant access for you to use the API
/// * `scope` - The scope your application is requesting from the Spotify API
///
/// # Returns
/// * `auth_url` - Generated authentication url for your application.
/// * `pkce_pre_auth` - Object storing data required for the next authentication step.
///
pub fn pkce_authentication_url(
    client_id: &'static str,
    redirect_uri: &'static str,
    scope: String,
) -> (String, PkcePreAuth) {
    let authorization_code_endpoint = String::from("https://accounts.spotify.com/authorize?"); // authorization code endpoint
    let character_set = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"; // character set for random string

    let (code_verifier, code_challenge) = generate_verifier(); // generate code verifier and challenge

    let state: String = random_string::generate(16, character_set); // generate random string for state variable

    let encoded_redirect_uri = encode(redirect_uri).into_owned(); // encode redirect uri for url

    // define parameters for authorization code request
    let parameters = vec![
        ("response_type", "code"),
        ("client_id", client_id),
        ("redirect_uri", &encoded_redirect_uri),
        ("scope", &scope[..]),
        ("show_dialog", "true"),
        ("state", &state[..]),
        ("code_challenge", &code_challenge[..]),
        ("code_challenge_method", "S256"),
    ];

    let query_parameters = stringify(parameters); // stringify parameters

    let auth_url = authorization_code_endpoint + &query_parameters; // create authorization url

    (
        auth_url,
        PkcePreAuth::new(client_id, redirect_uri, scope, state, code_verifier),
    )
}

/// Gets the access token required to access the API
///
/// # Arguments
/// * `request_client` - https client to perform request on
/// * `auth_code` - authorization code returned to the redirect_uri from user granting API access
/// * `pkce_pre_auth` - `PkcePreAuth` object returned by `pkce_authentication_url()`
///
async fn access_token(
    request_client: reqwest::Client,
    auth_code: &'static str,
    pkce_pre_auth: &PkcePreAuth,
) -> Result<PkceAccessTokenReponse, Error> {
    // pull out values needed for this function
    let (client_id, redirect_uri, code_verifier) = pkce_pre_auth.get_access_token_requirements();

    let request_uri = "https://accounts.spotify.com/api/token?"; // token request uri

    let encoded_redirect_uri = encode(redirect_uri).into_owned(); // encode redirect uri for url

    let query_parameters = vec![
        ("grant_type", "authorization_code"),
        ("code", auth_code),
        ("redirect_uri", &encoded_redirect_uri),
        ("client_id", client_id),
        ("code_verifier", code_verifier),
    ];

    let query_string = stringify(query_parameters); // stringify query parameters

    // build request headers
    let mut request_headers = HeaderMap::new();
    request_headers.insert(
        CONTENT_TYPE,
        "application/x-www-form-urlencoded".parse().unwrap(),
    );
    request_headers.insert(CONTENT_LENGTH, "0".parse().unwrap());

    // send request for access token
    let response = general_request(
        request_client,
        String::from(request_uri) + &query_string,
        RequestMethod::Post(request_headers, None),
    )
    .await;

    // handle errors
    match response {
        Err(e) => Err(e.into()),
        Ok(response) => {
            // unpack status code
            let status_code: Result<SpotifyStatus, Error> = response.status().try_into();

            match status_code {
                Ok(status_code) => {
                    // look for expected 200 response
                    match status_code {
                        // had an okay response, return access token result
                        SpotifyStatus::OK => Ok(response.json().await?),

                        // something unexpected happened
                        _ => {}
                    }
                }
                // unrecognized status code so who knows what happened
                Err(status_code) => Err(status_code),
            }
        }
    }
}

/// Completes the PKCE authentication codeflow, granting access to the Spotify API.
pub fn new_pkce(auth_code: &'static str, pkce_pre_auth: PkcePreAuth) {}
