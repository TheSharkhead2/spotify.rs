use crate::srequest::{spotify_request, RequestMethod};
use crate::spotify::{Spotify, SpotifyError, Artist, Albums};
use crate::object_formatting::{format_artist, format_albums};

impl Spotify {
    /// Get information on a single aritst: https://developer.spotify.com/documentation/web-api/reference/#/operations/get-an-artist 
    /// Required scope: none 
    pub fn get_artist(&self, artist_id: &str) -> Result<Artist, SpotifyError> {
        let url_extension = format!("artists/{}", artist_id);

        match self.access_token() { // Get access token
            Ok(access_token) => {
                match spotify_request(&access_token, &url_extension, RequestMethod::Get) { // make request 
                    Ok(response) => {
                        return Ok(format_artist(&response)) // format and return result
                    },
                    Err(e) => return Err(SpotifyError::RequestError(e.to_string())),
                }
            },
            Err(e) => return Err(e), // On error with access token, return error
        }
    }

    /// Gets information on several artists: https://developer.spotify.com/documentation/web-api/reference/#/operations/get-multiple-artists
    /// Required scope: none
    pub fn get_several_artists(&self, artist_ids: Vec<&str>) -> Result<Vec<Artist>, SpotifyError> {
        let url_extension = format!("artists/?ids={}", artist_ids.join(",")); // base url with artist ids added

        match self.access_token() { // get access token 
            Ok(access_token) => {
                match spotify_request(&access_token, &url_extension, RequestMethod::Get) { // make request
                    Ok(response) => { // format request into vector with formatted artists
                        let mut artists: Vec<Artist> = Vec::new();
                        for artist in response["artists"].members() { 
                            artists.push(format_artist(&artist));
                        }
                        return Ok(artists)
                    },
                    Err(e) => return Err(SpotifyError::RequestError(e.to_string())),
                }
            },
            Err(e) => return Err(e), // On error with access token, return error
        }
    }

    /// Get artist's albums: https://developer.spotify.com/documentation/web-api/reference/#/operations/get-an-artists-albums
    /// Required scope: none
    pub fn get_artist_albums(&self, artist_id: &str, include_groups: Option<Vec<&str>>, limit: Option<u32>, market: Option<&str>, offset: Option<u32>) -> Result<Albums, SpotifyError>{
        let mut url_extension = format!("artists/{}/albums", artist_id); // base url 

        if include_groups != None || limit != None || market != None || offset != None { // if any optional parameters are set, add query question mark 
            url_extension.push_str("?");
        }

        if let Some(include_groups) = include_groups { // if include_groups is set, add to url
            url_extension.push_str(&format!("&include_groups={}", include_groups.join(",")));
        }

        if let Some(limit) = limit { // if limit is set, add to url
            url_extension.push_str(&format!("&limit={}", limit));
        }

        if let Some(market) = market { // if market is set, add to url
            url_extension.push_str(&format!("&market={}", market));
        }

        if let Some(offset) = offset { // if offset is set, add to url
            url_extension.push_str(&format!("&offset={}", offset));
        }

        match self.access_token() { // get access token 
            Ok(access_token) => {
                match spotify_request(&access_token, &url_extension, RequestMethod::Get) { // make request
                    Ok(response) => {
                        return Ok(format_albums(&response)) // format response and return
                    },
                    Err(e) => return Err(SpotifyError::RequestError(e.to_string())), // request error 
                }
            },
            Err(e) => return Err(e), // error on access token error
        }
    }

    /// Get artist's top tracks: https://developer.spotify.com/documentation/web-api/reference/#/operations/get-an-artists-top-tracks
    /// Required scope: none 
    pub fn get_artist_top_tracks(&self, artist_id: &str, market: &str) -> Result<Vec<Artist>, SpotifyError> {
        let url_extension = format!("artists/{}/top-tracks?market={}", artist_id, market); // base url 

        match self.access_token() { // get access token 
            Ok(access_token) => {
                match spotify_request(&access_token, &url_extension, RequestMethod::Get) { // make request
                    Ok(response) => { // format response into vector with formatted artists
                        let mut artists: Vec<Artist> = Vec::new(); 
                        for artist in response["tracks"].members() { 
                            artists.push(format_artist(&artist));
                        }
                        return Ok(artists)
                    },
                    Err(e) => return Err(SpotifyError::RequestError(e.to_string())), // request error 
                }
            },
            Err(e) => return Err(e), // error on access token error
        }
    }

    /// Get artist's related to specified artist: https://developer.spotify.com/documentation/web-api/reference/#/operations/get-an-artists-related-artists
    /// Required scope: none
    pub fn get_artist_related_artists(&self, artist_id: &str) -> Result<Vec<Artist>, SpotifyError> {
        let url_extension = format!("artists/{}/related-artists", artist_id); // base url 

        match self.access_token() { // get access token 
            Ok(access_token) => {
                match spotify_request(&access_token, &url_extension, RequestMethod::Get) { // make request
                    Ok(response) => { // format response into vector with formatted artists
                        let mut artists: Vec<Artist> = Vec::new();
                        for artist in response["artists"].members() { 
                            artists.push(format_artist(&artist));
                        }
                        return Ok(artists)
                    },
                    Err(e) => return Err(SpotifyError::RequestError(e.to_string())), // request error 
                }
            },
            Err(e) => return Err(e), // error on access token error
        }
    }
}