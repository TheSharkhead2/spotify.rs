use crate::authentication::auth_errors::{process_auth_error, SpotifyAuthenticationError};
use crate::authentication::auth_objects::{PkceAuth, PkcePreAuth, Scope, SpotifyAuth};
use crate::requests::SpotifyStatus;
use crate::requests::{general_request, RequestMethod};
use crate::Error;

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use chrono::{Duration, Utc};
use querystring::stringify;
use reqwest::header::{HeaderMap, CONTENT_LENGTH, CONTENT_TYPE};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use urlencoding::encode;

#[cfg(feature = "local_auth")]
use {
    open,
    querystring::querify,
    std::collections::HashMap,
    tiny_http::{Response, Server},
};

/// Struct representing JSON response for 200 OK status code response from requesting access token.
/// Response detailed at: https://developer.spotify.com/documentation/web-api/tutorials/code-pkce-flow
#[derive(Deserialize)]
struct PkceAccessTokenReponse {
    access_token: String,
    token_type: String,
    scope: String,
    expires_in: i64,
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
    client_id: String,
    redirect_uri: &str,
    scope: Scope,
) -> (String, PkcePreAuth) {
    let authorization_code_endpoint = String::from("https://accounts.spotify.com/authorize?"); // authorization code endpoint
    let character_set = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"; // character set for random string

    let (code_verifier, code_challenge) = generate_verifier(); // generate code verifier and challenge

    let state: String = random_string::generate(16, character_set); // generate random string for state variable

    let encoded_redirect_uri = encode(redirect_uri).into_owned(); // encode redirect uri for url

    // convert Scope struct to string understandable by Spotify API
    let scope_string: String = scope.into();

    // define parameters for authorization code request
    let parameters = vec![
        ("response_type", "code"),
        ("client_id", &client_id[..]),
        ("redirect_uri", &encoded_redirect_uri),
        ("scope", &scope_string[..]),
        ("show_dialog", "true"),
        ("state", &state[..]),
        ("code_challenge", &code_challenge[..]),
        ("code_challenge_method", "S256"),
    ];

    let query_parameters = stringify(parameters); // stringify parameters

    let auth_url = authorization_code_endpoint + &query_parameters; // create authorization url

    (
        auth_url,
        PkcePreAuth::new(client_id, redirect_uri.into(), scope, state, code_verifier),
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
    auth_code: String,
    pkce_pre_auth: &PkcePreAuth,
) -> Result<PkceAccessTokenReponse, Error> {
    // pull out values needed for this function
    let (client_id, redirect_uri, code_verifier) = pkce_pre_auth.get_access_token_requirements();

    let request_uri = "https://accounts.spotify.com/api/token?"; // token request uri

    let encoded_redirect_uri = encode(&redirect_uri[..]).into_owned(); // encode redirect uri for url

    let query_parameters = vec![
        ("grant_type", "authorization_code"),
        ("code", &auth_code[..]),
        ("redirect_uri", &encoded_redirect_uri),
        ("client_id", &client_id[..]),
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
                        _ => {
                            // on error, should give status code 400: https://datatracker.ietf.org/doc/html/rfc6749#section-5.2
                            if let SpotifyStatus::BadRequest = status_code {
                                // convert into `spotifyrs::Error` object
                                Err(process_auth_error(response.json().await?))
                            } else {
                                // status code wasn't 400
                                Err(Error::UnexpectedStatusCode(status_code.into()))
                            }
                        }
                    }
                }
                // unrecognized status code so who knows what happened
                Err(status_code) => Err(status_code),
            }
        }
    }
}

/// Completes the PKCE authentication codeflow, granting access to the Spotify API.
pub async fn new_pkce(
    request_client: reqwest::Client,
    auth_code: String,
    state: String,
    pkce_pre_auth: PkcePreAuth,
) -> Result<SpotifyAuth, Error> {
    // if states don't match, there was some kind of problem
    if state != pkce_pre_auth.get_state() {
        return Err(Error::InvalidState);
    }

    match access_token(request_client, auth_code, &pkce_pre_auth).await {
        Ok(token) => {
            // get client id and scope
            let (client_id, scope) = pkce_pre_auth.get_auth_requirements();

            let expires_at = Utc::now() + Duration::seconds(token.expires_in); // get DateTime object for when token will expire

            Ok(SpotifyAuth::PKCE(PkceAuth::new(
                client_id,
                scope,
                token.access_token,
                token.refresh_token,
                expires_at,
            )))
        }
        // just pass on error
        Err(e) => Err(e),
    }
}

/// Fully automated PKCE authentication locally. Will generate auth url, open in the browser, listen for result, and then return the authetnicated Spotify object.
#[cfg(feature = "local_auth")]
pub async fn local_pkce(
    request_client: reqwest::Client,
    client_id: String,
    redirect_port: String,
    scope: Scope,
    timeout: u32,
) -> Result<SpotifyAuth, Error> {
    // format port into localhost url
    let redirect_uri = format!("http://localhost:{}/callback", redirect_port);

    // get auth url
    let (auth_url, pkce_pre_auth) = pkce_authentication_url(client_id, &redirect_uri, scope);

    // open auth url
    let open_success = open::that(auth_url);

    // error handling
    if let Err(err) = open_success {
        return Err(Error::BrowserFailure(err)); // failed to open browser
    }

    // localhost port
    let listening_ip = format!("127.0.0.1:{}", redirect_port);

    // open server
    let server = Server::http(listening_ip).unwrap();

    let mut code: Option<String> = None;
    let mut state: Option<String> = None;

    // wait for request with timeout
    match server.recv_timeout(core::time::Duration::from_secs(timeout.into())) {
        Ok(req) => {
            // check to see if we got a request
            if let Some(req) = req {
                let query: HashMap<String, String> = querify(&req.url()[10..]) // take everything but the "/callback?" in url and parse the query
                    .into_iter()
                    .map(|(x, y)| (String::from(x), String::from(y))) // convert from &str to String
                    .collect();

                // looking for both a code and a state
                if query.contains_key("code") && query.contains_key("state") {
                    // send message to browser saying it is okay to close
                    let response = Response::from_string("you can close the browser");
                    req.respond(response).unwrap();

                    // extract code and state
                    code = Some(query.get("code").unwrap().clone()); // can unwrap because already know key exists
                    state = Some(query.get("state").unwrap().clone());
                } else {
                    // didn't get both code and state, something is wrong
                    return Err(Error::UnexpectedAuthCode);
                }
            } else {
                // otherwise, timeout error
                return Err(Error::HttpServerTimeout);
            }
        }
        // io error
        Err(err) => return Err(Error::HttpServerError(err)),
    }

    // get authentication object
    let pkce = new_pkce(request_client, code.unwrap(), state.unwrap(), pkce_pre_auth).await?;

    Ok(pkce)
}
