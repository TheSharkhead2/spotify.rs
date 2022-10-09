use crate::srequest::RequestMethod;
use crate::spotify::{Spotify, SpotifyError, Playback}; 

impl Spotify {
    /// Gets current playback state of current user: <https://developer.spotify.com/documentation/web-api/reference/#/operations/get-information-about-the-users-current-playback> 
    /// Note: currently only supports Tracks and not Episodes. Unexpected behavior may occur with Episodes. 
    /// 
    /// Requires scope: user-read-playback-state
    /// 
    /// # Arguments
    /// * `market` - An ISO 3166-1 alpha-2 country code.
    /// 
    pub fn get_playback_state(&mut self, market: Option<&str>) -> Result<Playback, SpotifyError> {
        let mut url_extension = String::from("me/player?additional_types=track"); // create url extension (Note: only supporting tracks, not episodes)

        self.check_scope("user-read-playback-state")?; // check scope

        if let Some(market) = market { // if market is Some then add it to url extension
            url_extension.push_str(&format!("&market={}", market));
        }
        let response = self.spotify_request(&url_extension, RequestMethod::Get)?; // send request

        return Ok(Playback::new(&response)); // return playback
    }
}