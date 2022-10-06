use crate::srequest::RequestMethod;
use crate::spotify::{Spotify, SpotifyError, Playlist};

impl Spotify {
    /// Get a playlist owned by a Spotify user: <https://developer.spotify.com/documentation/web-api/reference/#/operations/get-playlist> 
    /// Note: no support for episodes at the moment so unexpected results may occur with playlists that contain episodes
    /// 
    /// Required scope: none 
    /// 
    /// # Arguments
    /// * `playlist_id` - The Spotify ID of the playlist.
    /// * `market` - An ISO 3166-1 alpha-2 country code.
    ///
    pub fn get_playlist(&mut self, playlist_id: &str, market: Option<&str>) -> Result<Playlist, SpotifyError> {
        let mut url_extension = format!("playlists/{}?additional_types=track", playlist_id); // base url. Currently this only supports tracks, not episodes

        if let Some(market) = market { // if market is set, add to url
            url_extension.push_str(&format!("&market={}", market));
        }

        let response = self.spotify_request::<String>(&url_extension, RequestMethod::Get)?; // make request (abitrarily choose string as type parameter, not used here)

        return Ok(Playlist::new(&response)); // format and return result
    }
}