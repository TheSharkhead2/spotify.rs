use reqwest; 
use json::{self, JsonValue, Null};
use crate::spotify::{Spotify, SpotifyError};
use std::collections::HashMap;
use serde::ser::Serialize;

/// Enum to store types of requests relevant to Spotify API
pub enum RequestMethod<V: Serialize> {
    Get,
    Post,
    Put(HashMap<String, V>),
    Delete,
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
    pub fn spotify_request<V: Serialize>(&mut self, url_extension: &str, request_method: RequestMethod<V>) -> Result<JsonValue, SpotifyError> {
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
            RequestMethod::Post => {match client.post(&request_url).headers(headers).send() {
                Ok(response) => response,
                Err(e) => return Err(SpotifyError::RequestError(e.to_string())),
            }},
            RequestMethod::Put(body) => {match client.put(&request_url).headers(headers).header("Content-Type", "application/json").json(&body).send() {
                Ok(response) => response,
                Err(e) => return Err(SpotifyError::RequestError(e.to_string())),
            }},
            RequestMethod::Delete => {match client.delete(&request_url).headers(headers).send(){
                Ok(response) => response,
                Err(e) => return Err(SpotifyError::RequestError(e.to_string())),
            }},
        };
        // if response is successful, read response 
        if response.status().is_success() {
            let response_body = json::parse(&response.text().unwrap());
        
            match response_body {
                Ok(response_body) => return Ok(response_body),
                Err(_) => Ok(Null), // on error just return nothing (temp fix probably)
            }

        } else {
            return Err(SpotifyError::RequestError(format!("{}",&response.status()))) // should probably do some proper error handling based on response code, for now just return general request error
        }
    }
}