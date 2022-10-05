use std::collections::HashMap;

use crate::srequest::RequestMethod;
use crate::spotify::{Spotify, SpotifyError, User, TimeRange, Artists, Tracks};


impl Spotify {
    /// Get information on current user: <https://developer.spotify.com/documentation/web-api/reference/#/operations/get-current-users-profile>
    /// Requires scope: user-read-private user-read-email
    pub fn get_current_users_profile(&mut self) -> Result<User, SpotifyError> {
        let url_extension = "me";

        self.check_scope("user-read-private user-read-email")?;

        let response = self.spotify_request::<String>(url_extension, RequestMethod::Get)?; // make request (abitrarily choose string as type parameter, not used here)

        return Ok(User::new(&response))
    }

    /// Gets the user's top artists. A derivative of: <https://developer.spotify.com/documentation/web-api/reference/#/operations/get-users-top-artists-and-tracks>
    /// Requires scope: user-top-read
    pub fn get_users_top_artists(&mut self, time_range: Option<TimeRange>, limit: Option<i32>, offset: Option<i32>) -> Result<Artists, SpotifyError> {
        let mut url_extension = String::from("me/top/artists");

        self.check_scope("user-top-read")?;

        // add query section to string if optional parameters supplied 
        if !time_range.is_none() || !limit.is_none() || !offset.is_none() {
            url_extension.push_str("?"); 
        }

        // add time range to string if supplied
        match time_range {
            Some(TimeRange::ShortTerm) => url_extension.push_str("time_range=short_term&"),
            Some(TimeRange::MediumTerm) => url_extension.push_str("time_range=medium_term&"),
            Some(TimeRange::LongTerm) => url_extension.push_str("time_range=long_term&"),
            None => (),
        }

        // add limit to string if supplied
        if let Some(limit) = limit {
            url_extension.push_str(&format!("limit={}&", limit));
        }

        // add offset to string if supplied
        if let Some(offset) = offset {
            url_extension.push_str(&format!("offset={}&", offset));
        }

        let response = self.spotify_request::<String>(&url_extension, RequestMethod::Get)?; // make request (abitrarily choose string as type parameter, not used here)

        return Ok(Artists::new(&response))
    }

    /// Gets the user's top tracks. A derivative of: <https://developer.spotify.com/documentation/web-api/reference/#/operations/get-users-top-artists-and-tracks>
    /// Requires scope: user-top-read
    pub fn get_users_top_tracks(&mut self, time_range: Option<TimeRange>, limit: Option<i32>, offset: Option<i32>) -> Result<Tracks, SpotifyError> {
        let mut url_extension = String::from("me/top/tracks");

        self.check_scope("user-top-read")?;

        // add query section to string if optional parameters supplied 
        if !time_range.is_none() || !limit.is_none() || !offset.is_none() {
            url_extension.push_str("?"); 
        }

        // add time range to string if supplied
        match time_range {
            Some(TimeRange::ShortTerm) => url_extension.push_str("time_range=short_term&"),
            Some(TimeRange::MediumTerm) => url_extension.push_str("time_range=medium_term&"),
            Some(TimeRange::LongTerm) => url_extension.push_str("time_range=long_term&"),
            None => (),
        }

        // add limit to string if supplied
        if let Some(limit) = limit {
            url_extension.push_str(&format!("limit={}&", limit));
        }

        // add offset to string if supplied
        if let Some(offset) = offset {
            url_extension.push_str(&format!("offset={}&", offset));
        }

        let response = self.spotify_request::<String>(&url_extension, RequestMethod::Get)?; // make request (abitrarily choose string as type parameter, not used here)

        return Ok(Tracks::new(&response))
    }

    /// Gets the public profile for a user: <https://developer.spotify.com/documentation/web-api/reference/#/operations/get-users-profile>
    /// Requires scope: none 
    pub fn get_users_profile(&mut self, user_id: &str) -> Result<User, SpotifyError> {
        let url_extension = format!("users/{}", user_id);

        let response = self.spotify_request::<String>(&url_extension, RequestMethod::Get)?; // make request (abitrarily choose string as type parameter, not used here)

        return Ok(User::new(&response))
    }

    /// Add current user as a follower to a playlist: <https://developer.spotify.com/documentation/web-api/reference/#/operations/follow-playlist>
    /// Requires scope: playlist-modify-public playlist-read-private playlist-modify-private
    pub fn follow_playlist(&mut self, playlist_id: &str, public: Option<bool>) -> Result<(), SpotifyError> {
        let url_extension = format!("playlists/{}/followers", playlist_id);

        self.check_scope("playlist-modify-public playlist-read-private playlist-modify-private")?;

        // create HashMap for body
        let mut body: HashMap<String, bool> = HashMap::new();
        if let Some(public) = public { // only insert body param if supplied
            body.insert("public".to_string(), public);
        }

        self.spotify_request(&url_extension, RequestMethod::Put(body))?;

        return Ok(())
    }
}