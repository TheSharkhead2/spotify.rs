use crate::srequest::{spotify_request, RequestMethod};
use crate::spotify::{Spotify, SpotifyError, Track, DatedTracks, FeatureTrack, AnalysisTrack};
use crate::object_formatting::{format_track, format_dated_tracks, format_feature_track, format_analysis_track};
use json::JsonValue::{Array, Boolean};

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

        self.check_scope("user-library-read")?; // check scope

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

    /// Save tracks into current user's library: https://developer.spotify.com/documentation/web-api/reference/#/operations/save-tracks-user
    /// Required scope: user-library-modify
    pub fn save_tracks(&self, track_ids: Vec<&str>) -> Result<(), SpotifyError> {
        let url_extension = format!("me/tracks?ids={}", track_ids.join(",")); // base url

        self.check_scope("user-library-modify")?; // check scope

        match self.access_token() { // get access token
            Ok(access_token) => {
                match spotify_request(&access_token, &url_extension, RequestMethod::Put) { // make request
                    Ok(_) => return Ok(()), // return Ok if no error
                    Err(e) => return Err(SpotifyError::RequestError(e.to_string())),
                }
            },
            Err(e) => return Err(e), // On error with access token, return error
        }
    }

    /// Remove tracks from current user's library: https://developer.spotify.com/documentation/web-api/reference/#/operations/remove-tracks-user
    /// Required scope: user-library-modify
    pub fn remove_tracks(&self, track_ids: Vec<&str>) -> Result<(), SpotifyError> { 
        let url_extension = format!("me/tracks?ids={}", track_ids.join(",")); // base url

        self.check_scope("user-library-modify")?; // check scope

        match self.access_token() { // get access token
            Ok(access_token) => {
                match spotify_request(&access_token, &url_extension, RequestMethod::Delete) { // make request
                    Ok(_) => return Ok(()), // return Ok if no error
                    Err(e) => return Err(SpotifyError::RequestError(e.to_string())),
                }
            },
            Err(e) => return Err(e), // On error with access token, return error
        }
    }

    /// Checks to see if specified tracks are saved in user's library: https://developer.spotify.com/documentation/web-api/reference/#/operations/check-users-saved-tracks
    /// Required scope: user-library-read
    pub fn check_saved_tracks(&self, track_ids: Vec<&str>) -> Result<Vec<bool>, SpotifyError> {
        let url_extension = format!("me/tracks/contains?ids={}", track_ids.join(",")); // base url

        self.check_scope("user-library-read")?; // check scope

        match self.access_token() { // get access token
            Ok(access_token) => {
                match spotify_request(&access_token, &url_extension, RequestMethod::Get) { // make request
                    Ok(response) => {
                        match response {
                            Array(response) => {
                                let mut saved_albums = Vec::new(); // vector for all bools 

                                for track in response {
                                    match track {
                                        Boolean(saved) => saved_albums.push(saved), // add bool to vector
                                        _ => return Err(SpotifyError::RequestError("Invalid response".to_string())), // blanket error that shouldn't happen
                                    }
                                }
                                return Ok(saved_albums) // return bool values
                            },
                            _ => return Err(SpotifyError::RequestError("Response was not an array".to_string())), // blanket error, shouldn't occur
                        }
                    },
                    Err(e) => return Err(SpotifyError::RequestError(e.to_string())),
                }
            },
            Err(e) => return Err(e), // On error with access token, return error
        }
    }

    /// Gets audio features for specified track(s): https://developer.spotify.com/documentation/web-api/reference/#/operations/get-several-audio-features
    /// Required scope: none 
    pub fn get_tracks_audio_features(&self, track_ids: Vec<&str>) -> Result<Vec<FeatureTrack>, SpotifyError> {
        let url_extension = format!("audio-features/?ids={}", track_ids.join(",")); // base url

        match self.access_token() { // get access token 
            Ok(access_token) => {
                match spotify_request(&access_token, &url_extension, RequestMethod::Get) { // make request 
                    Ok(response) => {
                        match &response["audio_features"] {
                            Array(response) => {
                                let mut audio_features = Vec::new(); // vector for all audio features

                                for track in response {
                                        audio_features.push(format_feature_track(&track)); // add audio features to vector
                                        
                                };
                                return Ok(audio_features) // return audio features
                            },
                            _ => return Err(SpotifyError::RequestError("Response was not an array".to_string())), // blanket error, shouldn't occur
                        }
                    },
                    Err(e) => return Err(SpotifyError::RequestError(e.to_string())),
                }
            }, 
            Err(e) => return Err(e), // On error with access token, return error
        }
    }

    /// Gets audio features for specified track: https://developer.spotify.com/documentation/web-api/reference/#/operations/get-audio-features
    /// Required scope: none
    pub fn get_track_audio_features(&self, track_id: &str) -> Result<FeatureTrack, SpotifyError> {
        let url_extension = format!("audio-features/{}", track_id); // base url

        match self.access_token() { // get access token
            Ok(access_token) => {
                match spotify_request(&access_token, &url_extension, RequestMethod::Get) { // make request
                    Ok(response) => {
                        return Ok(format_feature_track(&response)) // format and return result
                    },
                    Err(e) => return Err(SpotifyError::RequestError(e.to_string())),
                }
            },
            Err(e) => return Err(e), // On error with access token, return error
        }
    }

    /// Gets audio analysis for specified track: https://developer.spotify.com/documentation/web-api/reference/#/operations/get-audio-analysis
    /// Required scope: none
    pub fn get_track_audio_analysis(&self, track_id: &str) -> Result<AnalysisTrack, SpotifyError> {
        let url_extension = format!("audio-analysis/{}", track_id); // base url

        match self.access_token() { // get access token
            Ok(access_token) => {
                match spotify_request(&access_token, &url_extension, RequestMethod::Get) { // make request
                    Ok(response) => {
                        return match format_analysis_track(&response) { // format and return result
                            Ok(track) => Ok(track),
                            Err(e) => Err(e),
                        }
                    },
                    Err(e) => return Err(SpotifyError::RequestError(e.to_string())),
                }
            },
            Err(e) => return Err(e), // On error with access token, return error
        }
    }
}