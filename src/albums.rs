use crate::srequest::{spotify_request, RequestMethod};
use crate::spotify::{Spotify, SpotifyError, Album, Tracks, DatedAlbums, Albums};
use crate::object_formatting::{format_album, format_tracks, format_dated_albums, format_albums};
use json::JsonValue::{Array, Boolean};

impl Spotify {
    /// Get an album: https://developer.spotify.com/documentation/web-api/reference/#/operations/get-an-album
    /// Required scope: none
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
                        return Ok(format_album(&response)) // format album and return
                    },
                    Err(e) => return Err(SpotifyError::RequestError(e.to_string())), // return error specific to request errors
                }
            },
            Err(e) => return Err(e), // on error, return error
        } 

    }

    /// Get several albums: https://developer.spotify.com/documentation/web-api/reference/#/operations/get-multiple-albums
    /// Required scope: none
    pub fn get_albums(&self, album_ids: &Vec<&str>, market: Option<&str>) -> Result<Vec<Album>, SpotifyError> {
        let mut url_extension = format!("albums/?ids={}", album_ids.join(",")); // base url 

        // if market parameter supplied, add to request as query parameter 
        if let Some(market) = market {
            url_extension.push_str(&format!("&market={}", market));
        }

        match self.access_token() { // get access token
            Ok(access_token) => {
                match spotify_request(&access_token, &url_extension, RequestMethod::Get){ // make request  
                    Ok(response) => {
                        let mut albums = Vec::new(); // create vector to store albums
                        for album in response["albums"].members() {
                            albums.push(format_album(&album)); // format album and push to vector
                        }
                        return Ok(albums) // return vector of albums 
                    },
                    Err(e) => return Err(SpotifyError::RequestError(e.to_string())), // return error specific to request errors
                }
            },
            Err(e) => return Err(e), // on error, return error
        } 

    }

    /// Get an album's tracks: https://developer.spotify.com/documentation/web-api/reference/#/operations/get-an-albums-tracks
    /// Required scope: none
    pub fn get_album_tracks(&self, album_id: &str, market: Option<&str>, limit: Option<u32>, offset: Option<u32>) -> Result<Tracks, SpotifyError> {
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

        match self.access_token() { // get access token
            Ok(access_token) => {
                match spotify_request(&access_token, &url_extension, RequestMethod::Get){ // make request  
                    Ok(response) => {
                        let tracks = format_tracks(&response); // format tracks

                        return Ok(tracks) // return tracks
                    },
                    Err(e) => return Err(SpotifyError::RequestError(e.to_string())),
                }
            },
            Err(e) => return Err(e), // on error, return error
        } 

    }

    /// Get albums saved in user's library: https://developer.spotify.com/documentation/web-api/reference/#/operations/get-users-saved-albums
    /// Required scope: user-library-read
    pub fn get_saved_albums(&self, limit: Option<u32>, market: Option<&str>, offset: Option<u32>) -> Result<DatedAlbums, SpotifyError> {
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

        match self.access_token() { // get access token
            Ok(access_token) => {
                match spotify_request(&access_token, &url_extension, RequestMethod::Get){ // make request
                    Ok(response) => {
                        let albums = format_dated_albums(&response); // format albums

                        return Ok(albums)
                    },
                    Err(e) => return Err(SpotifyError::RequestError(e.to_string())),
                }
            },
            Err(e) => return Err(e), // on error, return error
        }
    }

    /// Save albums for current user: https://developer.spotify.com/documentation/web-api/reference/#/operations/save-albums-user
    /// Required scope: user-library-modify
    pub fn save_albums(&self, album_ids: Vec<&str>) -> Result<(), SpotifyError> {
        let album_ids_string = album_ids.join(","); // join album ids into string seperated by commas 

        let url_extension = format!("me/albums?ids={}", album_ids_string); // base url with album ids to add

        self.check_scope("user-library-modify")?; // check scope

        match self.access_token() { // get access token
            Ok(access_token) => {
                match spotify_request(&access_token, &url_extension, RequestMethod::Put){ // make request
                    Ok(_) => {
                        return Ok(())
                    },
                    Err(e) => return Err(SpotifyError::RequestError(e.to_string())),
                }
            },
            Err(e) => return Err(e), // on error, return error
        }
    }

    /// Remove saved albums from current user's library: https://developer.spotify.com/documentation/web-api/reference/#/operations/remove-albums-user 
    /// Required scope: user-library-modify
    pub fn remove_albums(&self, album_ids: Vec<&str>) -> Result<(), SpotifyError> {
        let album_ids_string = album_ids.join(","); // join album ids into string seperated by commas

        let url_extension = format!("me/albums?ids={}", album_ids_string); // base url with album ids to remove

        self.check_scope("user-library-modify")?; // check scope

        match self.access_token() { // get access token
            Ok(access_token) => {
                match spotify_request(&access_token, &url_extension, RequestMethod::Delete){ // make request
                    Ok(_) => {
                        return Ok(())
                    },
                    Err(e) => return Err(SpotifyError::RequestError(e.to_string())),
                }
            },
            Err(e) => return Err(e), // on error, return error
        }
    }

    /// Checks to see if albums are already saved to user's library: https://developer.spotify.com/documentation/web-api/reference/#/operations/check-users-saved-albums
    /// Required scope: user-library-read
    pub fn check_saved_albums(&self, album_ids: Vec<&str>) -> Result<Vec<bool>, SpotifyError> {
        let album_ids_string = album_ids.join(","); // join album ids into string seperated by commas

        let url_extension = format!("me/albums/contains?ids={}", album_ids_string); // base url with album ids to check

        self.check_scope("user-library-read")?; // check scope

        match self.access_token() { // get access token
            Ok(access_token) => {
                match spotify_request(&access_token, &url_extension, RequestMethod::Get){ // make request
                    Ok(response) => {
                        match response {
                            Array(response) => { // make sure it is an array
                                let mut saved_albums = Vec::new(); // vector to hold saved albums

                                for album in response { // iterate through response
                                    match album {
                                        Boolean(saved) => saved_albums.push(saved), // add saved status to vector
                                        _ => return Err(SpotifyError::RequestError("Invalid response".to_string())), // when the type of response isn't recognized, return error
                                    }
                                }

                                return Ok(saved_albums) // return saved albums
                            },
                            _ => return Err(SpotifyError::RequestError("Invalid response".to_string())),
                        }
                    },
                    Err(e) => return Err(SpotifyError::RequestError(e.to_string())),
                }
            },
            Err(e) => return Err(e), // on error, return error
        }
    }

    /// Get a list of new album releases featured in Spotify: https://developer.spotify.com/documentation/web-api/reference/#/operations/get-new-releases 
    /// Required scope: none
    pub fn get_new_releases(&self, country: Option<&str>, limit: Option<u32>, offset: Option<u32>) -> Result<Albums, SpotifyError> {
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

        match self.access_token() {
            Ok(access_token) => {
                match spotify_request(&access_token, &url_extension, RequestMethod::Get){ // make request
                    Ok(response) => {
                        let albums = format_albums(&response["albums"]); // format albums
        
                        return Ok(albums)
                    },
                    Err(e) => return Err(SpotifyError::RequestError(e.to_string())),
                }
            },
            Err(e) => return Err(e), // on error, return error
        }
        
    }
}