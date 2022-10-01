use crate::srequest::RequestMethod;
use crate::spotify::{Spotify, SpotifyError, Track, DatedTracks, FeatureTrack, AnalysisTrack};
use crate::object_formatting::{format_track, format_dated_tracks, format_feature_track, format_analysis_track};
use json::JsonValue::Boolean;

impl Spotify {
    /// Get information on a single track: https://developer.spotify.com/documentation/web-api/reference/#/operations/get-track
    /// Required scope: none
    pub fn get_track(&mut self, track_id: &str) -> Result<Track, SpotifyError> {
        let url_extension = format!("tracks/{}", track_id);

        let response = self.spotify_request(&url_extension, RequestMethod::Get)?; // make request

        return Ok(format_track(&response)); // format and return result
    }

    /// Get information on many tracks: https://developer.spotify.com/documentation/web-api/reference/#/operations/get-several-tracks
    /// Required scope: none
    pub fn get_several_tracks(&mut self, track_ids: Vec<&str>, market: Option<&str>) -> Result<Vec<Track>, SpotifyError> {
        let mut url_extension = format!("tracks/?ids={}", track_ids.join(",")); // base url with track ids added

        if let Some(market) = market { // if market is set, add to url
            url_extension.push_str(&format!("?market={}", market));
        }

        let response = self.spotify_request(&url_extension, RequestMethod::Get)?; // make request

        let mut tracks = Vec::new(); // create vector to store tracks
        for track in response["tracks"].members() {
            tracks.push(format_track(&track)); // format track and push to vector
        }

        return Ok(tracks); // return vector of tracks
    }

    /// Get user's saved tracks: https://developer.spotify.com/documentation/web-api/reference/#/operations/get-users-saved-tracks
    /// Required scope: user-library-read
    pub fn get_user_saved_tracks(&mut self, limit: Option<u32>, market: Option<&str>, offset: Option<u32>) -> Result<DatedTracks, SpotifyError> {
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

        let response = self.spotify_request(&url_extension, RequestMethod::Get)?; // make request

        return Ok(format_dated_tracks(&response)); // format and return result
    }

    /// Save tracks into current user's library: https://developer.spotify.com/documentation/web-api/reference/#/operations/save-tracks-user
    /// Required scope: user-library-modify
    pub fn save_tracks(&mut self, track_ids: Vec<&str>) -> Result<(), SpotifyError> {
        let url_extension = format!("me/tracks?ids={}", track_ids.join(",")); // base url

        self.check_scope("user-library-modify")?; // check scope

        self.spotify_request(&url_extension, RequestMethod::Put)?; // make request

        return Ok(()); // return nothing
    }

    /// Remove tracks from current user's library: https://developer.spotify.com/documentation/web-api/reference/#/operations/remove-tracks-user
    /// Required scope: user-library-modify
    pub fn remove_tracks(&mut self, track_ids: Vec<&str>) -> Result<(), SpotifyError> { 
        let url_extension = format!("me/tracks?ids={}", track_ids.join(",")); // base url

        self.check_scope("user-library-modify")?; // check scope

        self.spotify_request(&url_extension, RequestMethod::Delete)?; // make request

        return Ok(()); // return nothing
    }

    /// Checks to see if specified tracks are saved in user's library: https://developer.spotify.com/documentation/web-api/reference/#/operations/check-users-saved-tracks
    /// Required scope: user-library-read
    pub fn check_saved_tracks(&mut self, track_ids: Vec<&str>) -> Result<Vec<bool>, SpotifyError> {
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

    /// Gets audio features for specified track(s): https://developer.spotify.com/documentation/web-api/reference/#/operations/get-several-audio-features
    /// Required scope: none 
    pub fn get_tracks_audio_features(&mut self, track_ids: Vec<&str>) -> Result<Vec<FeatureTrack>, SpotifyError> {
        let url_extension = format!("audio-features/?ids={}", track_ids.join(",")); // base url

        let response = self.spotify_request(&url_extension, RequestMethod::Get)?; // make request

        let mut feature_tracks = Vec::new(); // create vector to store tracks

        for track in response["audio_features"].members() {
            feature_tracks.push(format_feature_track(&track)); // format track and push to vector
        }

        return Ok(feature_tracks); // return vector of tracks
    }

    /// Gets audio features for specified track: https://developer.spotify.com/documentation/web-api/reference/#/operations/get-audio-features
    /// Required scope: none
    pub fn get_track_audio_features(&mut self, track_id: &str) -> Result<FeatureTrack, SpotifyError> {
        let url_extension = format!("audio-features/{}", track_id); // base url

        let response = self.spotify_request(&url_extension, RequestMethod::Get)?; // make request

        return Ok(format_feature_track(&response)); // format and return track
    }

    /// Gets audio analysis for specified track: https://developer.spotify.com/documentation/web-api/reference/#/operations/get-audio-analysis
    /// Required scope: none
    pub fn get_track_audio_analysis(&mut self, track_id: &str) -> Result<AnalysisTrack, SpotifyError> {
        let url_extension = format!("audio-analysis/{}", track_id); // base url

        let response = self.spotify_request(&url_extension, RequestMethod::Get)?; // make request

        return Ok(format_analysis_track(&response)?); // format and return track
    }

    /// Gets track recommendations based on seed artists, tracks, or genres: https://developer.spotify.com/documentation/web-api/reference/#/operations/get-recommendations
    /// Required scope: none
    /// 
    /// # Arguments
    /// * `seed_artists` - A list of seed artists. Up to 5 seed values can be supplied between ```seed_artists```, ```seed_tracks```, and ```seed_genres```. Must supply 1.
    /// * `seed_tracks` - A list of seed tracks. Up to 5 seed values can be supplied between ```seed_artists```, ```seed_tracks```, and ```seed_genres```. Must supply 1.
    /// * `seed_genres` - A list of seed genres. Up to 5 seed values can be supplied between ```seed_artists```, ```seed_tracks```, and ```seed_genres```. Must supply 1.
    /// * `limit` - The target size of the list of recommended tracks. Default: 20. Minimum: 1. Maximum: 100.
    /// * `market` - An ISO 3166-1 alpha-2 country code 
    /// * `min_acousticness` - minimum acousticness value for tracks, between 0 and 1
    /// * `max_acousticness` - maximum acousticness value for track, between 0 and 1
    /// * `target_acousticness` - target acousticness value for track, between 0 and 1
    /// * `min_danceability` - minimum danceability value for tracks, between 0 and 1
    /// * `max_danceability` - maximum danceability value for track, between 0 and 1
    /// * `target_danceability` - target danceability value for track, between 0 and 1
    /// * `min_duration` - minimum duration of tracks in ms
    /// * `max_duration` - maximum duration of track in ms 
    /// * `target_duration` - target duration of track in ms
    /// * `min_energy` - minimum energy value for tracks, between 0 and 1
    /// * `max_energy` - maximum energy value for track, between 0 and 1
    /// * `target_energy` - target energy value for track, between 0 and 1
    /// * `min_instrumentalness` - minimum instrumentalness value for tracks, between 0 and 1
    /// * `max_instrumentalness` - maximum instrumentalness value for track, between 0 and 1
    /// * `target_instrumentalness` - target instrumentalness value for track, between 0 and 1
    /// * `min_key` - minimum key value for tracks, between 0 and 11
    /// * `max_key` - maximum key value for track, between 0 and 11
    /// * `target_key` - target key value for track, between 0 and 11
    /// * `min_liveness` - minimum liveness value for tracks, between 0 and 1
    /// * `max_liveness` - maximum liveness value for track, between 0 and 1
    /// * `target_liveness` - target liveness value for track, between 0 and 1
    /// * `min_loudness` - minimum loudness value for tracks in dB
    /// * `max_loudness` - maximum loudness value for track in dB
    /// * `target_loudness` - target loudness value for track in dB 
    /// * `min_mode` - minimum mode value for tracks, between 0 and 1
    /// * `max_mode` - maximum mode value for track, between 0 and 1
    /// * `target_mode` - target mode value for track, between 0 and 1
    /// * `min_popularity` - minimum popularity value for tracks, between 0 and 100
    /// * `max_popularity` - maximum popularity value for track, between 0 and 100
    /// * `target_popularity` - target popularity value for track, between 0 and 100
    /// * `min_speechiness` - minimum speechiness value for tracks, between 0 and 1
    /// * `max_speechiness` - maximum speechiness value for track, between 0 and 1
    /// * `target_speechiness` - target speechiness value for track, between 0 and 1
    /// * `min_tempo` - minimum tempo value for tracks in BPM
    /// * `max_tempo` - maximum tempo value for track in BPM
    /// * `target_tempo` - target tempo value for track in BPM
    /// * `min_time_signature` - minimum time signature value for tracks
    /// * `max_time_signature` - maximum time signature value for track
    /// * `target_time_signature` - target time signature value for track
    /// * `min_valence` - minimum valence value for tracks, between 0 and 1
    /// * `max_valence` - maximum valence value for track, between 0 and 1
    /// * `target_valence` - target valence value for track, between 0 and 1
    /// 
    /// # Panics 
    /// * If ```seed_artists```, ```seed_tracks```, and ```seed_genres``` are all empty. Need to supply a seed value.
    /// * If more than 5 seed values are supplied across `seed_artists`, `seed_tracks`, and `seed_genres`.
    /// 
    pub fn get_recommendations(&mut self, seed_artists: Option<Vec<&str>>, seed_genres: Option<Vec<&str>>, seed_tracks: Option<Vec<&str>>, limit: Option<i32>, market: Option<&str>,
        max_acousticness: Option<f64>, min_acousticness: Option<f64>, target_acousticness: Option<f64>, max_danceability: Option<f64>, min_danceability: Option<f64>, target_danceability: Option<f64>,
        max_duration: Option<i32>, min_duration: Option<i32>, target_duration: Option<i32>, max_energy: Option<f64>, min_energy: Option<f64>, target_energy: Option<f64>,
        max_instrumentalness: Option<f64>, min_instrumentalness: Option<f64>, target_instrumentalness: Option<f64>, max_key: Option<i32>, min_key: Option<i32>, target_key: Option<i32>,
        max_liveness: Option<f64>, min_liveness: Option<f64>, target_liveness: Option<f64>, max_loudness: Option<f64>, min_loudness: Option<f64>, target_loudness: Option<f64>,
        max_mode: Option<i32>, min_mode: Option<i32>, target_mode: Option<i32>, max_popularity: Option<i32>, min_popularity: Option<i32>, target_popularity: Option<i32>,
        max_speechiness: Option<f64>, min_speechiness: Option<f64>, target_speechiness: Option<f64>, max_tempo: Option<f64>, min_tempo: Option<f64>, target_tempo: Option<f64>,
        max_time_signature: Option<i32>, min_time_signature: Option<i32>, target_time_signature: Option<i32>, max_valence: Option<i32>, min_valence: Option<i32>, target_valence: Option<i32>) -> Result<Vec<Track>, SpotifyError> {
        // panic if not supplied with sufficient seed values
        if seed_artists == None && seed_genres == None && seed_tracks == None {
            panic!("Must supply at least one seed value");
        }

        // panic if more than 5 seed values are supplied
        if match &seed_artists {Some(seed_artists) => seed_artists.len(), None => 0} + match &seed_genres {Some(seed_genres) => seed_genres.len(), None => 0} + match &seed_tracks {Some(seed_tracks) => seed_tracks.len(), None => 0} > 5 {
            panic!("Cannot supply more than 5 seed values");
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

        // limit parameter
        if let Some(limit) = limit {
            url_extension.push_str(&format!("&limit={}", limit));
        }

        // market parameter
        if let Some(market) = market {
            url_extension.push_str(&format!("&market={}", market));
        }

        // acousticness parameters
        if let Some(max_acousticness) = max_acousticness {
            url_extension.push_str(&format!("&max_acousticness={}", max_acousticness));
        }

        if let Some(min_acousticness) = min_acousticness {
            url_extension.push_str(&format!("&min_acousticness={}", min_acousticness));
        }

        if let Some(target_acousticness) = target_acousticness {
            url_extension.push_str(&format!("&target_acousticness={}", target_acousticness));
        }

        // danceability parameters
        if let Some(max_danceability) = max_danceability {
            url_extension.push_str(&format!("&max_danceability={}", max_danceability));
        }

        if let Some(min_danceability) = min_danceability {
            url_extension.push_str(&format!("&min_danceability={}", min_danceability));
        }

        if let Some(target_danceability) = target_danceability {
            url_extension.push_str(&format!("&target_danceability={}", target_danceability));
        }

        // duration parameters
        if let Some(max_duration) = max_duration {
            url_extension.push_str(&format!("&max_duration_ms={}", max_duration));
        }

        if let Some(min_duration) = min_duration {
            url_extension.push_str(&format!("&min_duration_ms={}", min_duration));
        }

        if let Some(target_duration) = target_duration {
            url_extension.push_str(&format!("&target_duration_ms={}", target_duration));
        }

        // energy parameters
        if let Some(max_energy) = max_energy {
            url_extension.push_str(&format!("&max_energy={}", max_energy));
        }

        if let Some(min_energy) = min_energy {
            url_extension.push_str(&format!("&min_energy={}", min_energy));
        }

        if let Some(target_energy) = target_energy {
            url_extension.push_str(&format!("&target_energy={}", target_energy));
        }

        // instrumentalness parameters
        if let Some(max_instrumentalness) = max_instrumentalness {
            url_extension.push_str(&format!("&max_instrumentalness={}", max_instrumentalness));
        }

        if let Some(min_instrumentalness) = min_instrumentalness {
            url_extension.push_str(&format!("&min_instrumentalness={}", min_instrumentalness));
        }

        if let Some(target_instrumentalness) = target_instrumentalness {
            url_extension.push_str(&format!("&target_instrumentalness={}", target_instrumentalness));
        }

        // key parameters
        if let Some(max_key) = max_key {
            url_extension.push_str(&format!("&max_key={}", max_key));
        }

        if let Some(min_key) = min_key {
            url_extension.push_str(&format!("&min_key={}", min_key));
        }

        if let Some(target_key) = target_key {
            url_extension.push_str(&format!("&target_key={}", target_key));
        }

        // liveness parameters
        if let Some(max_liveness) = max_liveness {
            url_extension.push_str(&format!("&max_liveness={}", max_liveness));
        }

        if let Some(min_liveness) = min_liveness {
            url_extension.push_str(&format!("&min_liveness={}", min_liveness));
        }

        if let Some(target_liveness) = target_liveness {
            url_extension.push_str(&format!("&target_liveness={}", target_liveness));
        }

        // loudness parameters
        if let Some(max_loudness) = max_loudness {
            url_extension.push_str(&format!("&max_loudness={}", max_loudness));
        }

        if let Some(min_loudness) = min_loudness {
            url_extension.push_str(&format!("&min_loudness={}", min_loudness));
        }

        if let Some(target_loudness) = target_loudness {
            url_extension.push_str(&format!("&target_loudness={}", target_loudness));
        }

        // mode parameters
        if let Some(max_mode) = max_mode {
            url_extension.push_str(&format!("&max_mode={}", max_mode));
        }

        if let Some(min_mode) = min_mode {
            url_extension.push_str(&format!("&min_mode={}", min_mode));
        }

        if let Some(target_mode) = target_mode {
            url_extension.push_str(&format!("&target_mode={}", target_mode));
        }

        // popularity parameters
        if let Some(max_popularity) = max_popularity {
            url_extension.push_str(&format!("&max_popularity={}", max_popularity));
        }

        if let Some(min_popularity) = min_popularity {
            url_extension.push_str(&format!("&min_popularity={}", min_popularity));
        }

        if let Some(target_popularity) = target_popularity {
            url_extension.push_str(&format!("&target_popularity={}", target_popularity));
        }

        // speechiness parameters
        if let Some(max_speechiness) = max_speechiness {
            url_extension.push_str(&format!("&max_speechiness={}", max_speechiness));
        }

        if let Some(min_speechiness) = min_speechiness {
            url_extension.push_str(&format!("&min_speechiness={}", min_speechiness));
        }

        if let Some(target_speechiness) = target_speechiness {
            url_extension.push_str(&format!("&target_speechiness={}", target_speechiness));
        }

        // tempo parameters
        if let Some(max_tempo) = max_tempo {
            url_extension.push_str(&format!("&max_tempo={}", max_tempo));
        }

        if let Some(min_tempo) = min_tempo {
            url_extension.push_str(&format!("&min_tempo={}", min_tempo));
        }

        if let Some(target_tempo) = target_tempo {
            url_extension.push_str(&format!("&target_tempo={}", target_tempo));
        }

        // time signature parameters
        if let Some(max_time_signature) = max_time_signature {
            url_extension.push_str(&format!("&max_time_signature={}", max_time_signature));
        }

        if let Some(min_time_signature) = min_time_signature {
            url_extension.push_str(&format!("&min_time_signature={}", min_time_signature));
        }

        if let Some(target_time_signature) = target_time_signature {
            url_extension.push_str(&format!("&target_time_signature={}", target_time_signature));
        }

        // valence parameters
        if let Some(max_valence) = max_valence {
            url_extension.push_str(&format!("&max_valence={}", max_valence));
        }

        if let Some(min_valence) = min_valence {
            url_extension.push_str(&format!("&min_valence={}", min_valence));
        }

        if let Some(target_valence) = target_valence {
            url_extension.push_str(&format!("&target_valence={}", target_valence));
        }

        let response = self.spotify_request(&url_extension, RequestMethod::Get)?; // make request 

        let mut tracks = Vec::new(); // create vector to hold tracks
        for track in response["tracks"].members() {
            tracks.push(format_track(track)); // format track and push to vector
        }

        return Ok(tracks); // return vector
    }
}