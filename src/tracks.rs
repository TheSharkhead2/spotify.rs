use crate::srequest::{spotify_request, RequestMethod};
use crate::spotify::{Spotify, SpotifyError, Track, DatedTracks};
use crate::object_formatting::{format_track, format_dated_tracks};

impl Spotify {
    /// Get information on a single track: https://developer.spotify.com/documentation/web-api/reference/#/operations/get-track
    /// Required scope: none
    pub fn get_track(&self, track_id: &str) -> Result<Track, SpotifyError> {
        let url_extension = format!("tracks/{}", track_id);

        match self.access_token() { // Get access token
            Ok(access_token) => {
                match spotify_request(&access_token, &url_extension, RequestMethod::Get) { // make request 
                    Ok(response) => {
                        return Ok(format_track(&response)) // format and return result
                    },
                    Err(e) => return Err(SpotifyError::RequestError(e.to_string())),
                }
            },
            Err(e) => return Err(e), // On error with access token, return error
        }
    }

    /// Get information on many tracks: https://developer.spotify.com/documentation/web-api/reference/#/operations/get-several-tracks
    /// Required scope: none
    pub fn get_several_tracks(&self, track_ids: Vec<&str>, market: Option<&str>) -> Result<Vec<Track>, SpotifyError> {
        let mut url_extension = format!("tracks/?ids={}", track_ids.join(",")); // base url with track ids added

        if let Some(market) = market { // if market is set, add to url
            url_extension.push_str(&format!("?market={}", market));
        }

        match self.access_token() { // get access token 
            Ok(access_token) => {
                match spotify_request(&access_token, &url_extension, RequestMethod::Get) { // make request
                    Ok(response) => { // format request into vector with formatted tracks
                        let mut tracks: Vec<Track> = Vec::new();
                        for track in response["tracks"].members() { 
                            tracks.push(format_track(&track));
                        }
                        return Ok(tracks)
                    },
                    Err(e) => return Err(SpotifyError::RequestError(e.to_string())),
                }
            },
            Err(e) => return Err(e), // On error with access token, return error
        }
    }

    /// Get user's saved tracks: https://developer.spotify.com/documentation/web-api/reference/#/operations/get-users-saved-tracks
    /// Required scope: user-library-read
    pub fn get_user_saved_tracks(&self, limit: Option<u32>, market: Option<&str>, offset: Option<u32>) -> Result<DatedTracks, SpotifyError> {
        let mut url_extension = String::from("me/tracks"); // base url

        if limit != None || market != None || offset != None { // if any optional parameters are set, add ? to url
            url_extension.push_str("?");
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
                        return Ok(format_dated_tracks(&response)) // format and return result
                    },
                    Err(e) => return Err(SpotifyError::RequestError(e.to_string())),
                }
            },
            Err(e) => return Err(e), // On error with access token, return error
        }
    }
}