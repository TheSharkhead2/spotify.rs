use std::collections::HashMap;

use crate::srequest::RequestMethod;
use crate::spotify::{Spotify, SpotifyError, User, TimeRange, Artists, Tracks};


impl Spotify {
    /// Get information on current user: <https://developer.spotify.com/documentation/web-api/reference/#/operations/get-current-users-profile>
    /// 
    /// Requires scope: user-read-private user-read-email
    /// 
    pub fn get_current_users_profile(&mut self) -> Result<User, SpotifyError> {
        let url_extension = "me";

        self.check_scope("user-read-private user-read-email")?;

        let response = self.spotify_request::<String>(url_extension, RequestMethod::Get)?; // make request (abitrarily choose string as type parameter, not used here)

        return Ok(User::new(&response))
    }

    /// Gets the user's top artists. A derivative of: <https://developer.spotify.com/documentation/web-api/reference/#/operations/get-users-top-artists-and-tracks>
    /// 
    /// Requires scope: user-top-read
    /// 
    /// # Arguments
    /// * `time_range` - The time range over which to retrieve top artists: short, medium, long. Default: medium.
    /// * `limit` - The number of artists to return. Default: 20. Minimum: 1. Maximum: 50.
    /// * `offset` - The index of the first artist to return. Default: 0 (i.e., the first artist). Use with limit to get the next set of artists.
    /// 
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
    /// 
    /// Requires scope: user-top-read
    /// 
    /// # Arguments
    /// * `time_range` - The time range over which to retrieve top tracks: short, medium, long. Default: medium.
    /// * `limit` - The number of tracks to return. Default: 20. Minimum: 1. Maximum: 50.
    /// * `offset` - The index of the first track to return. Default: 0 (i.e., the first track). Use with limit to get the next set of tracks.
    /// 
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
    /// 
    /// Requires scope: none 
    /// 
    /// # Arguments
    /// * `user_id` - The user's Spotify user ID.
    /// 
    pub fn get_users_profile(&mut self, user_id: &str) -> Result<User, SpotifyError> {
        let url_extension = format!("users/{}", user_id);

        let response = self.spotify_request::<String>(&url_extension, RequestMethod::Get)?; // make request (abitrarily choose string as type parameter, not used here)

        return Ok(User::new(&response))
    }

    /// Add current user as a follower to a playlist: <https://developer.spotify.com/documentation/web-api/reference/#/operations/follow-playlist>
    /// 
    /// Requires scope: playlist-modify-public playlist-modify-private
    /// 
    /// # Arguments
    /// * `playlist_id` - The Spotify ID of the playlist.
    /// * `public` - If true the playlist will be included in user's public playlists, if false it will remain private. Default: true.
    /// 
    pub fn follow_playlist(&mut self, playlist_id: &str, public: Option<bool>) -> Result<(), SpotifyError> {
        let url_extension = format!("playlists/{}/followers", playlist_id);

        self.check_scope("playlist-modify-public playlist-modify-private")?;

        // create HashMap for body
        let mut body: HashMap<String, bool> = HashMap::new();
        if let Some(public) = public { // only insert body param if supplied
            body.insert("public".to_string(), public);
        }

        self.spotify_request(&url_extension, RequestMethod::Put(body))?;

        return Ok(())
    }

    /// Remove current user as a follower to a playlist: <https://developer.spotify.com/documentation/web-api/reference/#/operations/unfollow-playlist>
    /// 
    /// Requires scope: playlist-modify-private playlist-modify-public
    /// 
    /// # Arguments
    /// * `playlist_id` - The Spotify ID of the playlist.
    /// 
    pub fn unfollow_playlist(&mut self, playlist_id: &str) -> Result<(), SpotifyError> {
        let url_extension = format!("playlists/{}/followers", playlist_id);

        self.check_scope("playlist-modify-private playlist-modify-public")?;

        let body: HashMap<String, String> = HashMap::new(); // Create empty body (not necessary)

        self.spotify_request::<String>(&url_extension, RequestMethod::Delete(body))?; // make request (abitrarily choose string as type parameter, not used here)

        return Ok(())
    }

    /// Gets the current user's followed artists: <https://developer.spotify.com/documentation/web-api/reference/#/operations/get-followed>
    /// 
    /// Requires scope: user-follow-read
    /// 
    /// # Arguments
    /// * `limit` - The maximum number of items to return. Default: 20. Minimum: 1. Maximum: 50.
    /// 
    pub fn get_followed_artists(&mut self, limit: Option<i32>) -> Result<Artists, SpotifyError> {
        let mut url_extension = String::from("me/following?type=artist");

        self.check_scope("user-follow-read")?;

        // add limit to string if supplied
        if let Some(limit) = limit {
            url_extension.push_str(&format!("&limit={}", limit));
        }

        let response = self.spotify_request::<String>(&url_extension, RequestMethod::Get)?; // make request (abitrarily choose string as type parameter, not used here)

        return Ok(Artists::new(&response["artists"]))
    }

    /// Follows specified artists. A derivative of: <https://developer.spotify.com/documentation/web-api/reference/#/operations/follow-artists-users>
    /// 
    /// Requires scope: user-follow-modify
    /// 
    /// # Arguments
    /// * `artist_ids` - A vector of the artist Spotify IDs to follow.
    /// 
    pub fn follow_artists(&mut self, artist_ids: Vec<&str>) -> Result<(), SpotifyError> {
        let url_extension = format!("me/following?type=artist&ids={}", artist_ids.join(","));

        self.check_scope("user-follow-modify")?;

        // create HashMap for body
        let mut body: HashMap<String, Vec<&str>> = HashMap::new();
        body.insert("ids".to_string(), artist_ids);

        self.spotify_request(&url_extension, RequestMethod::Put(body))?;

        return Ok(())
    } 

    /// Follows specified users. A derivative of: <https://developer.spotify.com/documentation/web-api/reference/#/operations/follow-artists-users>
    /// 
    /// Requires scope: user-follow-modify
    /// 
    /// # Arguments
    /// * `user_ids` - A vector of the user Spotify IDs to follow.
    /// 
    pub fn follow_users(&mut self, user_ids: Vec<&str>) -> Result<(), SpotifyError> {
        let url_extension = format!("me/following?type=user&ids={}", user_ids.join(","));

        self.check_scope("user-follow-modify")?;

        // create HashMap for body
        let mut body: HashMap<String, Vec<&str>> = HashMap::new();
        body.insert("ids".to_string(), user_ids);

        self.spotify_request(&url_extension, RequestMethod::Put(body))?;

        return Ok(())
    }

    /// Unfollows specified artists. A derivative of: <https://developer.spotify.com/documentation/web-api/reference/#/operations/unfollow-artists-users>
    /// 
    /// Requires scope: user-follow-modify
    /// 
    /// # Arguments
    /// * `artist_ids` - A vector of the artist Spotify IDs to unfollow.
    /// 
    pub fn unfollow_artists(&mut self, artist_ids: Vec<&str>) -> Result<(), SpotifyError> {
        let url_extension = format!("me/following?type=artist&ids={}", artist_ids.join(","));

        self.check_scope("user-follow-modify")?;

        // create HashMap for body
        let mut body: HashMap<String, Vec<&str>> = HashMap::new();
        body.insert("ids".to_string(), artist_ids);

        self.spotify_request(&url_extension, RequestMethod::Delete(body))?;

        return Ok(())
    }

    /// Unfollows specified users. A derivative of: <https://developer.spotify.com/documentation/web-api/reference/#/operations/unfollow-artists-users>
    /// 
    /// Requires scope: user-follow-modify
    /// 
    /// # Arguments
    /// * `user_ids` - A vector of the user Spotify IDs to unfollow.
    /// 
    pub fn unfollow_users(&mut self, user_ids: Vec<&str>) -> Result<(), SpotifyError> {
        let url_extension = format!("me/following?type=user&ids={}", user_ids.join(","));

        self.check_scope("user-follow-modify")?;

        // create HashMap for body
        let mut body: HashMap<String, Vec<&str>> = HashMap::new();
        body.insert("ids".to_string(), user_ids);

        self.spotify_request(&url_extension, RequestMethod::Delete(body))?;

        return Ok(())
    }

    /// Check if user follows specific artists. Returns a vector of bools. A derivative of: <https://developer.spotify.com/documentation/web-api/reference/#/operations/check-current-user-follows>
    /// 
    /// Requires scope: user-follow-read
    /// 
    /// # Arguments
    /// * `artist_ids` - A vector of the artist Spotify IDs to check.
    /// 
    /// # Panics
    /// Panics if API returned value is not formatted as expected. Shouldn't happen. 
    /// 
    pub fn check_user_follows_artists(&mut self, artist_ids: Vec<&str>) -> Result<Vec<bool>, SpotifyError> {
        let url_extension = format!("me/following/contains?type=artist&ids={}", artist_ids.join(","));

        self.check_scope("user-follow-read")?;

        let response = self.spotify_request::<String>(&url_extension, RequestMethod::Get)?; // make request (abitrarily choose string as type parameter, not used here)

        let mut follows: Vec<bool> = Vec::new();

        for artist in response.members() {
            follows.push(artist.as_bool().unwrap());
        }

        return Ok(follows)
    }

    /// Check if user follows specific users. Returns a vector of bools. A derivative of: <https://developer.spotify.com/documentation/web-api/reference/#/operations/check-current-user-follows>
    /// 
    /// Requires scope: user-follow-read
    /// 
    /// # Arguments
    /// * `user_ids` - A vector of the user Spotify IDs to check.
    /// 
    /// # Panics
    /// Panics if API returned value is not formatted as expected. Shouldn't happen.
    /// 
    pub fn check_user_follows_users(&mut self, user_ids: Vec<&str>) -> Result<Vec<bool>, SpotifyError> {
        let url_extension = format!("me/following/contains?type=user&ids={}", user_ids.join(","));

        self.check_scope("user-follow-read")?;

        let response = self.spotify_request::<String>(&url_extension, RequestMethod::Get)?; // make request (abitrarily choose string as type parameter, not used here)

        let mut follows: Vec<bool> = Vec::new();

        for user in response.members() {
            follows.push(user.as_bool().unwrap());
        }

        return Ok(follows)
    }
}