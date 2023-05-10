use crate::spotify::{Spotify, SpotifyError};
use crate::srequest::RequestMethod;

impl Spotify {
    /// Gets set of available genres: <https://developer.spotify.com/documentation/web-api/reference/#/operations/get-recommendation-genres>
    ///
    /// Requires scope: none
    ///
    pub fn get_available_genre_seeds(&self) -> Result<Vec<String>, SpotifyError> {
        let url_extension = String::from("recommendations/available-genre-seeds");

        let response = self.spotify_request(&url_extension, RequestMethod::Get)?;

        let mut genres = Vec::new(); // create vector to store genres

        for genre in response["genres"].members() {
            // iterate over genres
            genres.push(genre.to_string()); // add genre to vector
        }

        Ok(genres)
    }
}
