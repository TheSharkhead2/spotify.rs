use crate::srequest::{spotify_request, RequestMethod};
use crate::spotify::{Spotify, SpotifyError, Album, Tracks, DatedAlbums};
use crate::object_formatting::{format_album, format_tracks, format_dated_albums};

impl Spotify {
    /// Get an album: https://developer.spotify.com/documentation/web-api/reference/#/operations/get-an-album
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
        let mut url_extension = String::from("me/albums/"); // base url

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
}