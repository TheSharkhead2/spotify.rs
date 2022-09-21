use crate::srequest::{spotify_request, RequestMethod};
use crate::spotify::{Spotify, SpotifyError, Artist};
use crate::object_formatting::{format_artist};

impl Spotify {
    /// Get information on a single aritst: https://developer.spotify.com/documentation/web-api/reference/#/operations/get-an-artist 
    /// Required scope: none 
    pub fn get_artist(&self, artist_id: &str) -> Result<Artist, SpotifyError> {
        let url_extension = format!("artists/{}", artist_id);

        match self.access_token() {
            Ok(access_token) => {
                match spotify_request(&access_token, &url_extension, RequestMethod::Get) {
                    Ok(response) => {
                        return Ok(format_artist(&response))
                    },
                    Err(e) => return Err(SpotifyError::RequestError(e.to_string())),
                }
            },
            Err(e) => return Err(e), // On error with access token, return error
        }
    }
}