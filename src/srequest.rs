use reqwest; 
use json::{self, JsonValue, Null};

/// Enum to store types of requests relevant to Spotify API
pub enum RequestMethod {
    Get,
    Post,
    Put,
    Delete,
}
 
/// General request to the spotify API. Returns JSON response 
/// 
/// # Arguments
/// * `access_token` - access token for Spotify API 
/// * `url_extension` - part of url past: https://api.spotify.com/v1/ . Specific to each type of request
/// * `request_method` - type of request (GET, POST, PUT, DELETE)
/// 
/// # Panics 
/// On various parsing errors. Shouldn't happen? Probably.
/// 
pub fn spotify_request(access_token: &str, url_extension: &str, request_method: RequestMethod) -> Result<JsonValue, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new(); // create client

    let mut headers = reqwest::header::HeaderMap::new(); // create header map
    headers.insert("Authorization", format!("Bearer {}", access_token).parse().unwrap()); // insert authorization header

    let request_url = format!("https://api.spotify.com/v1/{}", url_extension); // create request url
    
    // Send appropriate request for request method
    let response = match request_method {
        RequestMethod::Get => client.get(&request_url).headers(headers).send()?,
        RequestMethod::Post => client.post(&request_url).headers(headers).send()?,
        RequestMethod::Put => client.put(&request_url).headers(headers).header("Content-Length", "0").send()?,
        RequestMethod::Delete => client.delete(&request_url).headers(headers).send()?,
    };

    // if response is successful, read response 
    if response.status().is_success() {
        let response_body = json::parse(&response.text().unwrap());

        match response_body {
            Ok(response_body) => return Ok(response_body),
            Err(_) => Ok(Null), // on error just return nothing (temp fix probably)
        }

    } else {
        return Err(format!("{}",response.status()).into()) // should probably do some proper error handling based on response code
    }
}