use crate::spotify::{
    Album, DatedAlbum, Spotify, SpotifyCollection, SpotifyError, SpotifyObject, Track,
};
use crate::srequest::RequestMethod;
use json::JsonValue::Boolean;
use serde_json::Value;
use std::collections::HashMap;

impl Spotify {
    /// Get an album: <https://developer.spotify.com/documentation/web-api/reference/#/operations/get-an-album>
    ///
    /// Required scope: none
    ///
    /// # Arguments
    /// * `album_id` - The Spotify ID of the album.
    /// * `market` - An ISO 3166-1 alpha-2 country code.
    ///
    pub fn get_album(
        &mut self,
        album_id: &str,
        market: Option<&str>,
    ) -> Result<Album, SpotifyError> {
        let mut url_extension = format!("albums/{}", album_id); // base url

        // if market parameter supplied, add to request as query parameter
        if let Some(market) = market {
            url_extension.push_str(&format!("?market={}", market));
        }

        let response = self.spotify_request(&url_extension, RequestMethod::Get)?; // make request

        return Ok(Album::new(&response)); // format and return result
    }

    /// Get several albums: <https://developer.spotify.com/documentation/web-api/reference/#/operations/get-multiple-albums>
    ///
    /// Required scope: none
    ///
    /// # Arguments
    /// * `album_ids` - A vector of Spotify IDs for the albums.
    /// * `market` - An ISO 3166-1 alpha-2 country code.
    ///
    pub fn get_albums(
        &mut self,
        album_ids: Vec<&str>,
        market: Option<&str>,
    ) -> Result<Vec<Album>, SpotifyError> {
        let mut url_extension = format!("albums/?ids={}", album_ids.join(",")); // base url

        // if market parameter supplied, add to request as query parameter
        if let Some(market) = market {
            url_extension.push_str(&format!("&market={}", market));
        }

        let response = self.spotify_request(&url_extension, RequestMethod::Get)?; // make request

        let mut albums = Vec::new(); // create vector to store albums
        for album in response["albums"].members() {
            albums.push(Album::new(&album)); // format album and push to vector
        }
        return Ok(albums); // return vector of albums
    }

    /// Get an album's tracks: <https://developer.spotify.com/documentation/web-api/reference/#/operations/get-an-albums-tracks>
    ///
    /// Required scope: none
    ///
    /// # Arguments
    /// * `album_id` - The Spotify ID of the album.
    /// * `market` - An ISO 3166-1 alpha-2 country code.
    /// * `limit` - The maximum number of tracks to return. Default: 20. Minimum: 1. Maximum: 50.
    /// * `offset` - The index of the first track to return. Default: 0 (the first object). Use with limit to get the next set of tracks.
    ///
    pub fn get_album_tracks(
        &mut self,
        album_id: &str,
        market: Option<&str>,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<SpotifyCollection<Track>, SpotifyError> {
        let mut url_extension = format!("albums/{}/tracks", album_id); // base url

        // if any parameter is supplied, add to request as query parameter
        if market != None || limit != None || offset != None {
            url_extension.push_str("?");
        }

        // if market parameter supplied, add to request as query parameter
        if let Some(market) = market {
            url_extension.push_str(&format!("&market={}", market));
        }

        // if limit parameter supplied, add to request as query parameter
        if let Some(limit) = limit {
            url_extension.push_str(&format!("&limit={}", limit));
        }

        // if offset parameter supplied, add to request as query parameter
        if let Some(offset) = offset {
            url_extension.push_str(&format!("&offset={}", offset));
        }

        let response = self.spotify_request(&url_extension, RequestMethod::Get)?; // make request

        return Ok(SpotifyCollection::<Track>::new(&response)); // format and return result
    }

    /// Get albums saved in user's library: <https://developer.spotify.com/documentation/web-api/reference/#/operations/get-users-saved-albums>
    ///
    /// Required scope: user-library-read
    ///
    /// # Arguments
    /// * `limit` - The maximum number of albums to return. Default: 20. Minimum: 1. Maximum: 50.
    /// * `offset` - The index of the first album to return. Default: 0 (the first object). Use with limit to get the next set of albums.
    ///
    pub fn get_saved_albums(
        &mut self,
        limit: Option<u32>,
        market: Option<&str>,
        offset: Option<u32>,
    ) -> Result<SpotifyCollection<DatedAlbum>, SpotifyError> {
        let mut url_extension = String::from("me/albums"); // base url

        self.check_scope("user-library-read")?; // check scope

        // if any parameter is supplied, add to request as query parameter
        if market != None || limit != None || offset != None {
            url_extension.push_str("?");
        }

        // if market parameter supplied, add to request as query parameter
        if let Some(market) = market {
            url_extension.push_str(&format!("&market={}", market));
        }

        // if limit parameter supplied, add to request as query parameter
        if let Some(limit) = limit {
            url_extension.push_str(&format!("&limit={}", limit));
        }

        // if offset parameter supplied, add to request as query parameter
        if let Some(offset) = offset {
            url_extension.push_str(&format!("&offset={}", offset));
        }

        let response = self.spotify_request(&url_extension, RequestMethod::Get)?; // make request

        return Ok(SpotifyCollection::<DatedAlbum>::new(&response)); // format and return result
    }

    /// Save albums for current user: <https://developer.spotify.com/documentation/web-api/reference/#/operations/save-albums-user>
    ///
    /// Required scope: user-library-modify
    ///
    /// # Arguments
    /// * `album_ids` - A vector of Spotify IDs for the albums.
    ///
    pub fn save_albums(&mut self, album_ids: Vec<&str>) -> Result<(), SpotifyError> {
        let album_ids_string = album_ids.join(","); // join album ids into string seperated by commas

        let url_extension = format!("me/albums?ids={}", album_ids_string); // base url with album ids to add

        self.check_scope("user-library-modify")?; // check scope

        // create HashMap for request body
        let mut body = HashMap::new();
        body.insert(
            "ids".to_string(),
            Value::Array(
                album_ids
                    .iter()
                    .map(|s| Value::String(s.to_string()))
                    .collect(),
            ),
        );

        self.spotify_request(&url_extension, RequestMethod::Put(body))?; // make request

        return Ok(()); // return nothing
    }

    /// Remove saved albums from current user's library: <https://developer.spotify.com/documentation/web-api/reference/#/operations/remove-albums-user>
    ///
    /// Required scope: user-library-modify
    ///
    /// # Arguments
    /// * `album_ids` - A vector of Spotify IDs for the albums.
    ///
    pub fn remove_albums(&mut self, album_ids: Vec<&str>) -> Result<(), SpotifyError> {
        let album_ids_string = album_ids.join(","); // join album ids into string seperated by commas

        let url_extension = format!("me/albums?ids={}", album_ids_string); // base url with album ids to remove

        self.check_scope("user-library-modify")?; // check scope

        // Create HashMap for request body
        let mut body = HashMap::new();
        body.insert(
            "ids".to_string(),
            Value::Array(
                album_ids
                    .iter()
                    .map(|s| Value::String(s.to_string()))
                    .collect(),
            ),
        );

        self.spotify_request(&url_extension, RequestMethod::Delete(body))?; // make request

        return Ok(()); // return nothing
    }

    /// Checks to see if albums are already saved to user's library: <https://developer.spotify.com/documentation/web-api/reference/#/operations/check-users-saved-albums>
    ///
    /// Required scope: user-library-read
    ///
    /// # Arguments
    /// * `album_ids` - A vector of Spotify IDs for the albums.
    ///
    pub fn check_saved_albums(&mut self, album_ids: Vec<&str>) -> Result<Vec<bool>, SpotifyError> {
        let album_ids_string = album_ids.join(","); // join album ids into string seperated by commas

        let url_extension = format!("me/albums/contains?ids={}", album_ids_string); // base url with album ids to check

        self.check_scope("user-library-read")?; // check scope

        let response = self.spotify_request(&url_extension, RequestMethod::Get)?; // make request

        let mut saved_albums = Vec::new(); // create vector to store saved albums

        for album in response.members() {
            match album {
                Boolean(saved) => saved_albums.push(*saved),
                _ => return Err(SpotifyError::RequestError("Invalid response".to_string())),
            }
        }
        return Ok(saved_albums); // return vector of saved albums
    }

    /// Get a list of new album releases featured in Spotify: <https://developer.spotify.com/documentation/web-api/reference/#/operations/get-new-releases>
    ///
    /// Required scope: none
    ///
    /// # Arguments
    /// * `country` - An ISO 3166-1 alpha-2 country code.
    /// * `limit` - The maximum number of albums to return. Default: 20. Minimum: 1. Maximum: 50.
    /// * `offset` - The index of the first album to return. Default: 0 (the first object). Use with limit to get the next set of albums.
    ///
    pub fn get_new_releases(
        &mut self,
        country: Option<&str>,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<SpotifyCollection<Album>, SpotifyError> {
        let mut url_extension = String::from("browse/new-releases"); // base url

        // if any parameter is supplied, add to request as query parameter
        if country != None || limit != None || offset != None {
            url_extension.push_str("?");
        }

        // if country parameter supplied, add to request as query parameter
        if let Some(country) = country {
            url_extension.push_str(&format!("&country={}", country));
        }

        // if limit parameter supplied, add to request as query parameter
        if let Some(limit) = limit {
            url_extension.push_str(&format!("&limit={}", limit));
        }

        // if offset parameter supplied, add to request as query parameter
        if let Some(offset) = offset {
            url_extension.push_str(&format!("&offset={}", offset));
        }

        let response = self.spotify_request(&url_extension, RequestMethod::Get)?; // make request

        return Ok(SpotifyCollection::<Album>::new(&response["albums"])); // format and return result
    }
}
