use crate::spotify::{
    AnalysisTrack, DatedTrack, FeatureTrack, Spotify, SpotifyCollection, SpotifyError,
    SpotifyObject, Track,
};
use crate::srequest::RequestMethod;
use json::JsonValue::Boolean;
use querystring::stringify;
use serde_json::Value;
use std::collections::HashMap;

impl Spotify {
    /// Get information on a single track: <https://developer.spotify.com/documentation/web-api/reference/#/operations/get-track>
    ///
    /// Required scope: none
    ///
    /// # Arguments
    /// * `track_id` - The Spotify ID of the track.
    ///
    pub fn get_track(&self, track_id: &str) -> Result<Track, SpotifyError> {
        let url_extension = format!("tracks/{}", track_id);

        let response = self.spotify_request(&url_extension, RequestMethod::Get)?; // make request

        return Ok(Track::new(&response)); // format and return result
    }

    /// Get information on many tracks: <https://developer.spotify.com/documentation/web-api/reference/#/operations/get-several-tracks>
    ///
    /// Required scope: none
    ///
    /// # Arguments
    /// * `track_ids` - A vector of Spotify track ids.
    /// * `market` - An ISO 3166-1 alpha-2 country code.
    ///
    pub fn get_several_tracks(
        &self,
        track_ids: Vec<&str>,
        market: Option<&str>,
    ) -> Result<Vec<Track>, SpotifyError> {
        let mut url_extension = format!("tracks/?ids={}", track_ids.join(",")); // base url with track ids added

        if let Some(market) = market {
            // if market is set, add to url
            url_extension.push_str(&format!("?market={}", market));
        }

        let response = self.spotify_request(&url_extension, RequestMethod::Get)?; // make request
        let mut tracks = Vec::new(); // create vector to store tracks
        for track in response["tracks"].members() {
            tracks.push(Track::new(&track)); // format track and push to vector
        }

        return Ok(tracks); // return vector of tracks
    }

    /// Get user's saved tracks: <https://developer.spotify.com/documentation/web-api/reference/#/operations/get-users-saved-tracks>
    ///
    /// Required scope: user-library-read
    ///
    /// # Arguments
    /// * `limit` - The number of tracks to return. Default: 20. Minimum: 1. Maximum: 50.
    /// * `market` - An ISO 3166-1 alpha-2 country code.
    /// * `offset` - The index of the first track to return. Default: 0 (i.e., the first track). Use with limit to get the next set of tracks.
    ///
    pub fn get_user_saved_tracks(
        &self,
        limit: Option<u32>,
        market: Option<&str>,
        offset: Option<u32>,
    ) -> Result<SpotifyCollection<DatedTrack>, SpotifyError> {
        let mut url_extension = String::from("me/tracks"); // base url

        self.check_scope("user-library-read")?; // check scope

        if limit != None || market != None || offset != None {
            // if any optional parameters are set, add ? to url
            url_extension.push_str("?");
        }

        if let Some(limit) = limit {
            // if limit is set, add to url
            url_extension.push_str(&format!("&limit={}", limit));
        }

        if let Some(market) = market {
            // if market is set, add to url
            url_extension.push_str(&format!("&market={}", market));
        }

        if let Some(offset) = offset {
            // if offset is set, add to url
            url_extension.push_str(&format!("&offset={}", offset));
        }

        let response = self.spotify_request(&url_extension, RequestMethod::Get)?; // make request

        return Ok(SpotifyCollection::<DatedTrack>::new(&response)); // format and return result
    }

    /// Save tracks into current user's library: <https://developer.spotify.com/documentation/web-api/reference/#/operations/save-tracks-user>
    ///
    /// Required scope: user-library-modify
    ///
    /// # Arguments
    /// * `track_ids` - A vector of Spotify track ids
    ///
    pub fn save_tracks(&self, track_ids: Vec<&str>) -> Result<(), SpotifyError> {
        let url_extension = format!("me/tracks?ids={}", track_ids.join(",")); // base url

        self.check_scope("user-library-modify")?; // check scope

        // create HashMap for request body
        let mut body = HashMap::new();
        body.insert(
            "ids".to_string(),
            Value::Array(
                track_ids
                    .iter()
                    .map(|&s| Value::String(s.to_string()))
                    .collect(),
            ),
        );

        self.spotify_request(&url_extension, RequestMethod::Put(body))?; // make request

        return Ok(()); // return nothing
    }

    /// Remove tracks from current user's library: <https://developer.spotify.com/documentation/web-api/reference/#/operations/remove-tracks-user>
    ///
    /// Required scope: user-library-modify
    ///
    /// # Arguments
    /// * `track_ids` - A vector of Spotify track IDs
    ///
    pub fn remove_tracks(&self, track_ids: Vec<&str>) -> Result<(), SpotifyError> {
        let url_extension = format!("me/tracks?ids={}", track_ids.join(",")); // base url

        self.check_scope("user-library-modify")?; // check scope

        // create HashMap for request body
        let mut body = HashMap::new();
        body.insert(
            "ids".to_string(),
            Value::Array(
                track_ids
                    .iter()
                    .map(|&s| Value::String(s.to_string()))
                    .collect(),
            ),
        );

        self.spotify_request(&url_extension, RequestMethod::Delete(body))?; // make request (abitrarily choose string as type parameter, not used here)

        return Ok(()); // return nothing
    }

    /// Checks to see if specified tracks are saved in user's library: <https://developer.spotify.com/documentation/web-api/reference/#/operations/check-users-saved-tracks>
    ///
    /// Required scope: user-library-read
    ///
    /// # Arguments
    /// * `track_ids` - A vector of track ids to check
    ///
    pub fn check_saved_tracks(&self, track_ids: Vec<&str>) -> Result<Vec<bool>, SpotifyError> {
        let url_extension = format!("me/tracks/contains?ids={}", track_ids.join(",")); // base url

        self.check_scope("user-library-read")?; // check scope

        let response = self.spotify_request(&url_extension, RequestMethod::Get)?; // make request

        let mut saved_tracks = Vec::new(); // create vector to store saved albums

        for track in response.members() {
            match track {
                Boolean(saved) => saved_tracks.push(*saved), // push saved status to vector
                _ => return Err(SpotifyError::RequestError("Invalid response".to_string())), // return error if invalid response
            }
        }

        return Ok(saved_tracks); // return vector of saved tracks
    }

    /// Gets audio features for specified track(s): <https://developer.spotify.com/documentation/web-api/reference/#/operations/get-several-audio-features>
    ///
    /// Required scope: none
    ///
    /// # Arguments
    /// * `track_ids` - A vector of track ids
    ///
    pub fn get_tracks_audio_features(
        &self,
        track_ids: Vec<&str>,
    ) -> Result<Vec<FeatureTrack>, SpotifyError> {
        let url_extension = format!("audio-features/?ids={}", track_ids.join(",")); // base url

        let response = self.spotify_request(&url_extension, RequestMethod::Get)?; // make request

        let mut feature_tracks = Vec::new(); // create vector to store tracks

        for track in response["audio_features"].members() {
            feature_tracks.push(FeatureTrack::new(&track)); // format track and push to vector
        }

        return Ok(feature_tracks); // return vector of tracks
    }

    /// Gets audio features for specified track: <https://developer.spotify.com/documentation/web-api/reference/#/operations/get-audio-features>
    ///
    /// Required scope: none
    ///
    /// # Arguments
    /// * `track_id` - Spotify ID of track
    ///
    pub fn get_track_audio_features(
        &self,
        track_id: &str,
    ) -> Result<FeatureTrack, SpotifyError> {
        let url_extension = format!("audio-features/{}", track_id); // base url

        let response = self.spotify_request(&url_extension, RequestMethod::Get)?; // make request

        return Ok(FeatureTrack::new(&response)); // format and return track
    }

    /// Gets audio analysis for specified track: <https://developer.spotify.com/documentation/web-api/reference/#/operations/get-audio-analysis>
    ///
    /// Required scope: none
    ///
    /// # Arguments
    /// * `track_id` - Spotify ID of track
    ///
    pub fn get_track_audio_analysis(
        &self,
        track_id: &str,
    ) -> Result<AnalysisTrack, SpotifyError> {
        let url_extension = format!("audio-analysis/{}", track_id); // base url

        let response = self.spotify_request(&url_extension, RequestMethod::Get)?; // make request

        return Ok(AnalysisTrack::new(&response)?); // format and return track
    }

    /// Gets track recommendations based on seed artists, tracks, or genres: <https://developer.spotify.com/documentation/web-api/reference/#/operations/get-recommendations>
    ///
    /// Required scope: none
    ///
    /// # Arguments
    /// * `seed_artists` - A list of seed artists. Up to 5 seed values can be supplied between ```seed_artists```, ```seed_tracks```, and ```seed_genres```. Must supply 1.
    /// * `seed_tracks` - A list of seed tracks. Up to 5 seed values can be supplied between ```seed_artists```, ```seed_tracks```, and ```seed_genres```. Must supply 1.
    /// * `seed_genres` - A list of seed genres. Up to 5 seed values can be supplied between ```seed_artists```, ```seed_tracks```, and ```seed_genres```. Must supply 1.
    /// * `optional_parameters` - A vector mapping parameters to values. Can have any of the following as keys:
    ///     * `limit` - The target size of the list of recommended tracks. Default: 20. Minimum: 1. Maximum: 100.
    ///     * `market` - An ISO 3166-1 alpha-2 country code
    ///     * `min_acousticness` - minimum acousticness value for tracks, between 0 and 1
    ///     * `max_acousticness` - maximum acousticness value for track, between 0 and 1
    ///     * `target_acousticness` - target acousticness value for track, between 0 and 1
    ///     * `min_danceability` - minimum danceability value for tracks, between 0 and 1
    ///     * `max_danceability` - maximum danceability value for track, between 0 and 1
    ///     * `target_danceability` - target danceability value for track, between 0 and 1
    ///     * `min_duration` - minimum duration of tracks in ms
    ///     * `max_duration` - maximum duration of track in ms
    ///     * `target_duration` - target duration of track in ms
    ///     * `min_energy` - minimum energy value for tracks, between 0 and 1
    ///     * `max_energy` - maximum energy value for track, between 0 and 1
    ///     * `target_energy` - target energy value for track, between 0 and 1
    ///     * `min_instrumentalness` - minimum instrumentalness value for tracks, between 0 and 1
    ///     * `max_instrumentalness` - maximum instrumentalness value for track, between 0 and 1
    ///     * `target_instrumentalness` - target instrumentalness value for track, between 0 and 1
    ///     * `min_key` - minimum key value for tracks, between 0 and 11
    ///     * `max_key` - maximum key value for track, between 0 and 11
    ///     * `target_key` - target key value for track, between 0 and 11
    ///     * `min_liveness` - minimum liveness value for tracks, between 0 and 1
    ///     * `max_liveness` - maximum liveness value for track, between 0 and 1
    ///     * `target_liveness` - target liveness value for track, between 0 and 1
    ///     * `min_loudness` - minimum loudness value for tracks in dB
    ///     * `max_loudness` - maximum loudness value for track in dB
    ///     * `target_loudness` - target loudness value for track in dB
    ///     * `min_mode` - minimum mode value for tracks, between 0 and 1
    ///     * `max_mode` - maximum mode value for track, between 0 and 1
    ///     * `target_mode` - target mode value for track, between 0 and 1
    ///     * `min_popularity` - minimum popularity value for tracks, between 0 and 100
    ///     * `max_popularity` - maximum popularity value for track, between 0 and 100
    ///     * `target_popularity` - target popularity value for track, between 0 and 100
    ///     * `min_speechiness` - minimum speechiness value for tracks, between 0 and 1
    ///     * `max_speechiness` - maximum speechiness value for track, between 0 and 1
    ///     * `target_speechiness` - target speechiness value for track, between 0 and 1
    ///     * `min_tempo` - minimum tempo value for tracks in BPM
    ///     * `max_tempo` - maximum tempo value for track in BPM
    ///     * `target_tempo` - target tempo value for track in BPM
    ///     * `min_time_signature` - minimum time signature value for tracks
    ///     * `max_time_signature` - maximum time signature value for track
    ///     * `target_time_signature` - target time signature value for track
    ///     * `min_valence` - minimum valence value for tracks, between 0 and 1
    ///     * `max_valence` - maximum valence value for track, between 0 and 1
    ///     * `target_valence` - target valence value for track, between 0 and 1
    ///
    pub fn get_recommendations(
        &self,
        seed_artists: Option<Vec<&str>>,
        seed_genres: Option<Vec<&str>>,
        seed_tracks: Option<Vec<&str>>,
        optional_parameters: Option<Vec<(&str, &str)>>,
    ) -> Result<Vec<Track>, SpotifyError> {
        // panic if not supplied with sufficient seed values
        if seed_artists == None && seed_genres == None && seed_tracks == None {
            return Err(SpotifyError::InvalidRequest(String::from(
                "Must supply at least one seed value",
            )));
        }

        // panic if more than 5 seed values are supplied
        if match &seed_artists {
            Some(seed_artists) => seed_artists.len(),
            None => 0,
        } + match &seed_genres {
            Some(seed_genres) => seed_genres.len(),
            None => 0,
        } + match &seed_tracks {
            Some(seed_tracks) => seed_tracks.len(),
            None => 0,
        } > 5
        {
            return Err(SpotifyError::InvalidRequest(String::from(
                "Cannot supply more than 5 seed values",
            )));
        }

        let mut url_extension = String::from("recommendations?");

        // add seed values to url
        if let Some(seed_artists) = seed_artists {
            url_extension.push_str(&format!("&seed_artists={}", seed_artists.join(",")));
        }

        if let Some(seed_genres) = seed_genres {
            url_extension.push_str(&format!("&seed_genres={}", seed_genres.join(",")));
        }

        if let Some(seed_tracks) = seed_tracks {
            url_extension.push_str(&format!("&seed_tracks={}", seed_tracks.join(",")));
        }

        // add optional parameters to url
        if let Some(optional_parameters) = optional_parameters {
            url_extension.push_str("&"); // need to make sure can accept more parameters
            url_extension.push_str(&stringify(optional_parameters)); // stringify and add optional parameters
        }

        let response = self.spotify_request(&url_extension, RequestMethod::Get)?; // make request

        let mut tracks = Vec::new(); // create vector to hold tracks
        for track in response["tracks"].members() {
            tracks.push(Track::new(track)); // format track and push to vector
        }

        return Ok(tracks); // return vector
    }
}
