use crate::srequest::RequestMethod;
use crate::spotify::{Spotify, SpotifyError, User};


impl Spotify {
    /// Get information on current user: <https://developer.spotify.com/documentation/web-api/reference/#/operations/get-current-users-profile>
    /// Requires scope: user-read-private user-read-email
    pub fn get_current_users_profile(&mut self) -> Result<User, SpotifyError> {
        let url_extension = "me";

        self.check_scope("user-read-private user-read-email")?;

        let response = self.spotify_request(url_extension, RequestMethod::Get)?;

        return Ok(User::new(&response))
    }
}