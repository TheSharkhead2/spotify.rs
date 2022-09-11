use crate::srequest::{spotify_request, RequestMethod};
use crate::spotify::{Spotify, SpotifyError, Album};
use crate::object_formatting::{format_album};

impl Spotify {
    pub fn get_album(&self, album_id: &str, market: Option<&str>) -> Result<Album, SpotifyError> {
        let mut url_extension = format!("albums/{}", album_id); // base url 

        // if market parameter supplied, add to request as query parameter 
        if let Some(market) = market {
            url_extension.push_str(&format!("?market={}", market));
        }

        match self.access_token() { // get access token
            Ok(access_token) => {
                match spotify_request(&access_token, &url_extension, RequestMethod::Get){ // make request  
                    Ok(response) => {
                        return Ok(format_album(&response))
                    },
                    Err(e) => return Err(SpotifyError::RequestError(e.to_string())),
                }
            },
            Err(e) => return Err(e), // on error, return error
        } 

    }
}