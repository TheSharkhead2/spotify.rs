use crate::spotify::SpotifyError;
use crate::srequest::{general_request, RequestMethodHeaders};
use querystring::stringify;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use urlencoding::encode;

/// Struct for JSON from access token request
#[derive(Deserialize, Serialize, Debug)]
struct AccessTokenResponseJson {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub refresh_token: String,
    pub scope: String,
}

/// Generates the code verifier and code challenge for PKCE
///
/// # Panics
/// When random number generation fails. See [get random docs](https://docs.rs/getrandom/latest/getrandom/#functions)
///
fn generate_verifier() -> (String, String) {
    let mut buf = [0u8; 32]; // empty list of 32 bytes

    getrandom::getrandom(&mut buf).unwrap(); // generate random bytes - unwrap to panic on random failure

    let code_verifier = base64::encode_config(buf, base64::URL_SAFE).replace('=', ""); // encode bytes into url safe base64 string and remove tailing equal sign

    let mut code_challenge_hasher = Sha256::new(); // new hashing object to create code challenge
    code_challenge_hasher.update(&code_verifier); // add code verifier to hash
    let code_challenge_raw = code_challenge_hasher.finalize(); // finalize hash of code verifier

    let code_challenge =
        base64::encode_config(code_challenge_raw, base64::URL_SAFE).replace('=', ""); // encode bytes into url safe base64 string and remove tailing equal sign

    (code_verifier, code_challenge)
}

/// Creates and returns url to request the Spotify API for authorization code. Doesn't also grab it from a localhost port. Will return: (auth_url, state, code_verifier) where `state` is the state variable used
/// in the url for extra security and the `code_verifier` is needed for authentication.
///
pub fn requesturl_authorization_code(
    client_id: &str,
    redirect_uri: &str,
    scope: &str,
) -> (String, String, String) {
    let authorization_code_endpoint = String::from("https://accounts.spotify.com/authorize?"); // authorization code endpoint
    let character_set = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"; // character set for random string

    let (code_verifier, code_challenge) = generate_verifier(); // generate code verifier and challenge

    let state = random_string::generate(16, character_set); // generate random string for state variable

    let encoded_redirect_uri = encode(redirect_uri).into_owned(); // encode redirect uri for url

    // define parameters for authorization code request
    let parameters = vec![
        ("response_type", "code"),
        ("client_id", client_id),
        ("redirect_uri", &encoded_redirect_uri),
        ("scope", scope),
        ("show_dialog", "true"),
        ("state", &state),
        ("code_challenge", &code_challenge[..]),
        ("code_challenge_method", "S256"),
    ];

    let query_parameters = stringify(parameters); // stringify parameters

    let auth_url = authorization_code_endpoint + &query_parameters; // create authorization url

    (auth_url, state, code_verifier)
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
pub async fn get_access_token(
    authorization_code: &str,
    client_id: &str,
    code_verifier: &str,
    redirect_uri: &str,
) -> Result<(String, String, i64), SpotifyError> {
    let request_uri = "https://accounts.spotify.com/api/token?"; // token request uri

    let encoded_redirect_uri = encode(redirect_uri).into_owned(); // encode redirect uri for url

    let query_parameters = vec![
        ("grant_type", "authorization_code"),
        ("code", authorization_code),
        ("redirect_uri", &encoded_redirect_uri),
        ("client_id", client_id),
        ("code_verifier", code_verifier),
    ];

    let query_string = stringify(query_parameters); // stringify query parameters

    let request_headers = HashMap::from([
        (
            String::from("Content-Type"),
            String::from("application/x-www-form-urlencoded"),
        ),
        (String::from("Content-Length"), String::from("0")),
    ]);

    let response = general_request(
        String::from(request_uri) + &query_string,
        RequestMethodHeaders::Post(request_headers, HashMap::new()),
    )
    .await?;

    match response {
        Ok(mut res) => {
            if res.status() == 200 {
                let response_data: AccessTokenResponseJson = match res.body_json().await {
                    Ok(data) => data,
                    Err(e) => return Err(SpotifyError::AuthenticationError(e.to_string())),
                };

                Ok((
                    response_data.access_token,
                    response_data.refresh_token,
                    response_data.expires_in,
                ))
            } else {
                Err(SpotifyError::AuthenticationError(format!(
                    "Status code: {}",
                    res.status()
                )))
            }
        }
        Err(e) => Err(SpotifyError::AuthenticationError(e.to_string())),
    }
}

/// Requests new refresh token from Spotify API. Returns (access_token, refresh_token, expires_in)
///
/// # Arguments
/// * `refresh_token` - The refresh token used to request a new refresh token
/// * `client_id` - The client id of the application
///
pub async fn refresh_access_token(
    refresh_token: &str,
    client_id: &str,
) -> Result<(String, String, i64), SpotifyError> {
    let request_uri = "https://accounts.spotify.com/api/token?"; // token request uri

    let query_parameters = vec![
        ("grant_type", "refresh_token"),
        ("refresh_token", refresh_token),
        ("client_id", client_id),
    ];

    let query_string = stringify(query_parameters); // stringify query parameters

    let request_headers = HashMap::from([
        (
            String::from("Content-Type"),
            String::from("application/x-www-form-urlencoded"),
        ),
        (String::from("Content-Length"), String::from("0")),
    ]);

    let response = general_request(
        String::from(request_uri) + &query_string,
        RequestMethodHeaders::Post(request_headers, HashMap::new()),
    )
    .await?;

    match response {
        Ok(mut res) => {
            if res.status() == 200 {
                let response_data: AccessTokenResponseJson = match res.body_json().await {
                    Ok(data) => data,
                    Err(e) => return Err(SpotifyError::AuthenticationError(e.to_string())),
                };

                Ok((
                    response_data.access_token,
                    response_data.refresh_token,
                    response_data.expires_in,
                ))
            } else {
                Err(SpotifyError::AuthenticationError(format!(
                    "Status code: {}",
                    res.status()
                )))
            }
        }
        Err(e) => Err(SpotifyError::AuthenticationError(e.to_string())),
    }
    // if response.status().is_success() {
    //     // check if response is successful
    //     let response_body = json::parse(&response.text().unwrap()).unwrap(); // get response as json

    //     let access_token = response_body["access_token"].to_string(); // get access token from response
    //     let expires_in_str = response_body["expires_in"].to_string(); // get expires in from response
    //     let expires_in: i64 = expires_in_str.parse().unwrap(); // parse expires in to i64
    //     let new_refresh_token = match response_body["refresh_token"] {
    //         // get refresh token from response
    //         json::JsonValue::Null => refresh_token.to_string(),
    //         _ => response_body["refresh_token"].to_string(),
    //     };

    //     return Ok((access_token, expires_in, new_refresh_token)); // return access token and expires in and new refresh token
    // } else {
    //     let response_code = response.status().as_u16(); // get response code

    //     let response_body = json::parse(&response.text().unwrap()).unwrap(); // get response as json

    //     match response_code {
    //         400 => {
    //             return Err(SpotifyError::BadRequest(format!(
    //                 "Error {}: {}",
    //                 response_code, response_body["error_description"]
    //             )))
    //         }
    //         401 => {
    //             return Err(SpotifyError::Unauthorized(format!(
    //                 "Error {}: {}",
    //                 response_code, response_body["error_description"]
    //             )))
    //         }
    //         _ => {
    //             return Err(SpotifyError::GeneralError(format!(
    //                 "Error: {}",
    //                 response_code
    //             )))
    //         }
    //     }
    // }
}
