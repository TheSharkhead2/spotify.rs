use crate::spotify::{Spotify, SpotifyError};
use json::{self, JsonValue, Null};
use serde_json::Value;
use std::collections::HashMap;

/// Enum to store types of requests relevant to Spotify API
pub enum RequestMethod {
    Get,
    Post(HashMap<String, Value>),
    Put(HashMap<String, Value>),
    Delete(HashMap<String, Value>),
}

impl Spotify {
    /// New Spotify request codeflow using the surf request package.
    pub async fn spotify_request(
        &self,
        url_extension: &str,
        request_method: RequestMethod,
    ) -> Result<JsonValue, SpotifyError> {
        let access_token = self.access_token()?; // get access token

        // create request url
        let request_url = format!("https://api.spotify.com/v1/{}", url_extension);

        let request = match request_method {
            RequestMethod::Get => {
                surf::get(request_url).header("Authorization", format!("Bearer {}", access_token))
            }
            RequestMethod::Post(body) => {
                match surf::post(request_url)
                    .header("Authorization", format!("Bearer {}", access_token))
                    .header("Content-Type", "application/json")
                    .body_json(&body)
                {
                    Ok(req) => req,
                    Err(e) => return Err(SpotifyError::RequestError(e.to_string())),
                }
            }
            RequestMethod::Put(body) => {
                match surf::put(request_url)
                    .header("Authorization", format!("Bearer {}", access_token))
                    .header("Content-Type", "application/json")
                    .body_json(&body)
                {
                    Ok(req) => req,
                    Err(e) => return Err(SpotifyError::RequestError(e.to_string())),
                }
            }
            RequestMethod::Delete(body) => {
                match surf::delete(request_url)
                    .header("Authorization", format!("Bearer {}", access_token))
                    .header("Content-Type", "application/json")
                    .body_json(&body)
                {
                    Ok(req) => req,
                    Err(e) => return Err(SpotifyError::RequestError(e.to_string())),
                }
            }
        }
        .await;

        match request {
            Ok(mut res) => {
                let request_body = match res.body_string().await {
                    Ok(body) => body,
                    Err(e) => {
                        return Err(SpotifyError::RequestError(format!(
                            "JSON parsing error: {}",
                            e
                        )))
                    }
                };

                let response = match json::parse(&request_body) {
                    Ok(res) => res,
                    Err(e) => {
                        return Err(SpotifyError::RequestError(format!(
                            "JSON parsing error: {}",
                            e
                        )))
                    }
                };

                Ok(response)
            }
            Err(e) => Err(SpotifyError::RequestError(e.to_string())),
        }
    }

    /// General request to the spotify API. Returns JSON response
    ///
    /// # Arguments
    /// * `url_extension` - part of url past: `https://api.spotify.com/v1/`. Specific to each type of request
    /// * `request_method` - type of request (GET, POST, PUT, DELETE)
    ///
    /// # Panics
    /// On various parsing errors. Shouldn't happen? Probably.
    ///
    pub fn spotify_request_legacy(
        &self,
        url_extension: &str,
        request_method: RequestMethod,
    ) -> Result<JsonValue, SpotifyError> {
        let access_token = self.access_token()?; // get access token

        let client = reqwest::blocking::Client::new(); // create client

        let mut headers = reqwest::header::HeaderMap::new(); // create header map
        headers.insert(
            "Authorization",
            format!("Bearer {}", access_token).parse().unwrap(),
        ); // insert authorization header

        let request_url = format!("https://api.spotify.com/v1/{}", url_extension); // create request url

        // Send appropriate request for request method
        let response = match request_method {
            RequestMethod::Get => match client.get(&request_url).headers(headers).send() {
                Ok(response) => response,
                Err(e) => return Err(SpotifyError::RequestError(e.to_string())),
            },
            RequestMethod::Post(body) => {
                match client
                    .post(&request_url)
                    .headers(headers)
                    .header("Content-Type", "application/json")
                    .json(&body)
                    .send()
                {
                    Ok(response) => response,
                    Err(e) => return Err(SpotifyError::RequestError(e.to_string())),
                }
            }
            RequestMethod::Put(body) => {
                match client
                    .put(&request_url)
                    .headers(headers)
                    .header("Content-Type", "application/json")
                    .json(&body)
                    .send()
                {
                    Ok(response) => response,
                    Err(e) => return Err(SpotifyError::RequestError(e.to_string())),
                }
            }
            RequestMethod::Delete(body) => {
                match client
                    .delete(&request_url)
                    .headers(headers)
                    .header("Content-Type", "application/json")
                    .json(&body)
                    .send()
                {
                    Ok(response) => response,
                    Err(e) => return Err(SpotifyError::RequestError(e.to_string())),
                }
            }
        };

        let response_body = json::parse(&response.text().unwrap()); // parse response body
        match response_body {
            // check for errors
            Ok(response_body) => {
                if response_body["error"].is_null() {
                    // if no error field then assume no error
                    Ok(response_body)
                } else {
                    match response_body["error"]["status"].as_i32() {
                        // match various known errors
                        Some(401) => Err(SpotifyError::BadOrExpiredToken(
                            response_body["error"]["message"].to_string(),
                        )),
                        Some(403) => Err(SpotifyError::BadRequest(
                            response_body["error"]["message"].to_string(),
                        )),
                        Some(429) => Err(SpotifyError::RateLimitExceeded(
                            response_body["error"]["message"].to_string(),
                        )),
                        _ => Err(SpotifyError::RequestError(format!(
                            "Error code: {}, message: {}",
                            response_body["error"]["status"], response_body["error"]["message"]
                        ))), // unknown error/general error
                    }
                }
            }
            Err(_) => Ok(Null), // on json parsing error just return nothing (temp fix for a potential non-problem)
        }
    }
}
