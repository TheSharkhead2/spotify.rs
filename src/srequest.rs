use reqwest; 
use json::{self, JsonValue, Null};
use crate::spotify::{Spotify, SpotifyError};
use std::collections::HashMap;
use serde_json::Value;

/// Enum to store types of requests relevant to Spotify API
pub enum RequestMethod {
    Get,
    Post(HashMap<String, Value>),
    Put(HashMap<String, Value>),
    Delete(HashMap<String, Value>),
}
 
impl Spotify {
    /// General request to the spotify API. Returns JSON response 
    /// 
    /// # Arguments
    /// * `url_extension` - part of url past: https://api.spotify.com/v1/ . Specific to each type of request
    /// * `request_method` - type of request (GET, POST, PUT, DELETE)
    /// 
    /// # Panics 
    /// On various parsing errors. Shouldn't happen? Probably.
    /// 
    pub fn spotify_request(&mut self, url_extension: &str, request_method: RequestMethod) -> Result<JsonValue, SpotifyError> {
        let access_token = self.access_token(); // get access token

        let client = reqwest::blocking::Client::new(); // create client

        let mut headers = reqwest::header::HeaderMap::new(); // create header map
        headers.insert("Authorization", format!("Bearer {}", access_token).parse().unwrap()); // insert authorization header

        let request_url = format!("https://api.spotify.com/v1/{}", url_extension); // create request url

        // Send appropriate request for request method
        let response = match request_method {
            RequestMethod::Get => {match client.get(&request_url).headers(headers).send() {
                Ok(response) => response,
                Err(e) => return Err(SpotifyError::RequestError(e.to_string())),
            }},
            RequestMethod::Post(body) => {match client.post(&request_url).headers(headers).header("Content-Type", "application/json").json(&body).send() {
                Ok(response) => response,
                Err(e) => return Err(SpotifyError::RequestError(e.to_string())),
            }},
            RequestMethod::Put(body) => {match client.put(&request_url).headers(headers).header("Content-Type", "application/json").json(&body).send() {
                Ok(response) => response,
                Err(e) => return Err(SpotifyError::RequestError(e.to_string())),
            }},
            RequestMethod::Delete(body) => {match client.delete(&request_url).headers(headers).header("Content-Type", "application/json").json(&body).send(){
                Ok(response) => response,
                Err(e) => return Err(SpotifyError::RequestError(e.to_string())),
            }},
        };

        let response_body = json::parse(&response.text().unwrap()); // parse response body
        match response_body { // check for errors
            Ok(response_body) => {
                if response_body["error"].is_null() { // if no error field then assume no error
                    Ok(response_body)
                } else {
                    match response_body["error"]["status"].as_i32() { // match various known errors
                        Some(401) => Err(SpotifyError::BadOrExpiredToken(response_body["error"]["message"].to_string())),
                        Some(403) => Err(SpotifyError::BadRequest(response_body["error"]["message"].to_string())),
                        Some(429) => Err(SpotifyError::RateLimitExceeded(response_body["error"]["message"].to_string())),
                        _ => Err(SpotifyError::RequestError(format!("Error code: {}, message: {}", response_body["error"]["status"], response_body["error"]["message"]))), // unknown error/general error
                    }
                    
                }
            },
            Err(_) => Ok(Null), // on json parsing error just return nothing (temp fix for a potential non-problem)
        }
    }
}