use crate::spotify::{Spotify, SpotifyError};
use crate::srequest::RequestMethod;

impl Spotify {
    /// Gets a vector of all markets where Spotify is available: <https://developer.spotify.com/documentation/web-api/reference/#/operations/get-available-markets>
    ///
    /// Requires scope: none
    ///
    pub fn get_available_markets(&self) -> Result<Vec<String>, SpotifyError> {
        let url_extension = String::from("markets");

        let response = self.spotify_request(&url_extension, RequestMethod::Get)?;

        let mut markets = Vec::new(); // create vector to store markets

        for market in response["markets"].members() {
            // iterate over markets
            markets.push(market.to_string()); // add market to vector
        }

        Ok(markets)
    }
}
