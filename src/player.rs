use std::collections::HashMap;
use crate::srequest::RequestMethod;
use crate::spotify::{Spotify, SpotifyError, Playback, Device, SpotifyContext, RepeatState, SpotifyCollection, PlayedTrack, Track, SpotifyObject}; 
use chrono::NaiveDateTime;
use serde_json::{Value, Map, Number};

impl Spotify {
    /// Gets current playback state of current user: <https://developer.spotify.com/documentation/web-api/reference/#/operations/get-information-about-the-users-current-playback> 
    /// Note: currently only supports Tracks and not Episodes. Unexpected behavior may occur with Episodes. 
    /// 
    /// Requires scope: user-read-playback-state
    /// 
    /// # Arguments
    /// * `market` - An ISO 3166-1 alpha-2 country code.
    /// 
    pub fn get_playback_state(&mut self, market: Option<&str>) -> Result<Playback, SpotifyError> {
        let mut url_extension = String::from("me/player?additional_types=track"); // create url extension (Note: only supporting tracks, not episodes)

        self.check_scope("user-read-playback-state")?; // check scope

        if let Some(market) = market { // if market is Some then add it to url extension
            url_extension.push_str(&format!("&market={}", market));
        }
        let response = self.spotify_request(&url_extension, RequestMethod::Get)?; // send request

        return Ok(Playback::new(&response)); // return playback
    }

    /// Transfers playback to another device and whether or not the new device should play: <https://developer.spotify.com/documentation/web-api/reference/#/operations/transfer-a-users-playback>
    /// 
    /// Requires scope: user-modify-playback-state
    /// 
    /// # Arguments
    /// * `device_id` - The device id to transfer playback to
    /// * `play` - Whether or not to start playback on the new device
    /// 
    pub fn transfer_playback(&mut self, device_id: &str, play: bool) -> Result<(), SpotifyError> {
        let url_extension = String::from("me/player"); // create url extension

        self.check_scope("user-modify-playback-state")?; // check scope

        let mut body: HashMap<String, Value> = HashMap::new(); // create body
        body.insert("device_ids".to_string(), Value::Array(vec![Value::String(device_id.to_string())])); // insert device id
        body.insert("play".to_string(), Value::Bool(play)); // insert play

        self.spotify_request(&url_extension, RequestMethod::Put(body))?; // send request

        return Ok(());
    }

    /// Gets all the available spotify devices for playback: <https://developer.spotify.com/documentation/web-api/reference/#/operations/get-a-users-available-devices>
    /// 
    /// Requires scope: user-read-playback-state
    /// 
    pub fn get_available_devices(&mut self) -> Result<Vec<Device>, SpotifyError> {
        let url_extension = String::from("me/player/devices"); // create url extension

        self.check_scope("user-read-playback-state")?;

        let response = self.spotify_request(&url_extension, RequestMethod::Get)?; // send request

        let mut devices: Vec<Device> = Vec::new(); // empty vector to store devices 

        for device in response["devices"].members() { // iterate through devices
            devices.push(Device::new(&device)); // push device to vector
        }

        return Ok(devices); // return vector
    }

    /// Gets the currently playing track: <https://developer.spotify.com/documentation/web-api/reference/#/operations/get-the-users-currently-playing-track>
    /// Note: Currently this only supports tracks and not episodes. Weird behavior may occur if an episode is being played. 
    /// 
    /// Requires scope: user-read-currently-playing
    /// 
    /// # Arguments
    /// * `market` - An ISO 3166-1 alpha-2 country code which the returned track should be in the market of 
    /// 
    pub fn get_currently_playing_track(&mut self, market: Option<&str>) -> Result<Playback, SpotifyError> {
        let mut url_extension = String::from("me/player/currently-playing?additional_types=track"); // create url extension. Only supporting tracks right now

        self.check_scope("user-read-currently-playing")?; // check scope

        if let Some(market) = market { // if market is Some then add it to url extension
            url_extension.push_str(&format!("&market={}", market));
        }

        let response = self.spotify_request(&url_extension, RequestMethod::Get)?; // send request

        return Ok(Playback::new(&response)); // return playback
    }

    /// Start a new context in player or resume playback of a device: <https://developer.spotify.com/documentation/web-api/reference/#/operations/start-a-users-playback> 
    /// 
    /// Requires scope: user-modify-playback-state
    /// 
    /// # Arguments
    /// * `device_id` - The id of the device to start playback on
    /// * `context` - The context to start playback on. Valid contexts: Album, Artist, Playlist
    /// * `track_ids` - The track ids to start playback on
    /// * `offset_position` - Indicates where in the context the playback should start. For example, starting on the 2nd song of an album with offset=1. 
    /// * `offset_track` - Indicates which track in context to begin playback on. This is a track id. Note: this will be ignored if offset_position is set.
    /// * `position_ms` - Where in the song to begin playback
    /// 
    pub fn start_resume_playback(&mut self, device_id: Option<&str>, context: Option<SpotifyContext>, track_ids: Option<Vec<&str>>, offset_position: Option<i32>, offset_track: Option<&str>, position_ms: Option<i32>) -> Result<(), SpotifyError> {
        let mut url_extension = String::from("me/player/play"); // create url extension

        self.check_scope("user-modify-playback-state")?; // check scope

        if let Some(device_id) = device_id {
            url_extension.push_str(&format!("?device_id={}", device_id)); // if device_id is supplied, then add it to url extension
        }

        let mut body: HashMap<String, Value> = HashMap::new(); // create body

        if let Some(context) = context {
            body.insert("context_uri".to_string(), Value::String(context.uri())); // if context is supplied, then add it to body
        }

        if let Some(track_ids) = track_ids {
            let mut tracks: Vec<Value> = Vec::new(); // create vector to store track ids

            for track_id in track_ids {
                tracks.push(Value::String(track_id.to_string())); // push track id to vector
            }

            body.insert("uris".to_string(), Value::Array(tracks)); // insert track ids into body
        }

        if let Some(offset) = offset_position {
            let mut m = Map::new();
            m.insert("position".to_string(), Value::Number(Number::from(offset))); // if offset_position is supplied, then add it to body

            body.insert("offset".to_string(), Value::Object(m)); // if offset is supplied, then add it to body
        }

        if offset_position.is_none() { //if offset_position wasn't supplied 
            if let Some(track_id) = offset_track {
                let mut m = Map::new();
                m.insert("uri".to_string(), Value::String(format!("spotify:track:{}", track_id))); // if offset_track is supplied, then add it to body

                body.insert("offset".to_string(), Value::Object(m)); // if offset is supplied, then add it to body
            }
        }

        if let Some(position_ms) = position_ms {
            body.insert("position_ms".to_string(), Value::Number(Number::from(position_ms))); // if position_ms is supplied, then add it to body
        }

        self.spotify_request(&url_extension, RequestMethod::Put(body))?; // send request

        return Ok(())
    }

    /// Pauses the user's playback: <https://developer.spotify.com/documentation/web-api/reference/#/operations/pause-a-users-playback> 
    /// 
    /// Requires scope: user-modify-playback-state
    /// 
    /// # Arguments
    /// * `device_id` - The id of the device to pause playback on
    /// 
    pub fn pause_playback(&mut self, device_id: Option<&str>) -> Result<(), SpotifyError> {
        let mut url_extension = String::from("me/player/pause"); // create url extension

        self.check_scope("user-modify-playback-state")?; // check scope 

        if let Some(device_id) = device_id {
            url_extension.push_str(&format!("?device_id={}", device_id)); // if device_id is supplied, then add it to url extension
        }

        self.spotify_request(&url_extension, RequestMethod::Put(HashMap::new()))?; // send request

        return Ok(())
    }

    /// Skips the currently playing track to the next track: <https://developer.spotify.com/documentation/web-api/reference/#/operations/skip-users-playback-to-next-track> 
    /// 
    /// Requires scope: user-modify-playback-state
    /// 
    /// # Arguments
    /// * `device_id` - The id of the device to skip on 
    /// 
    pub fn skip_next(&mut self, device_id: Option<&str>) -> Result<(), SpotifyError> {
        let mut url_extension = String::from("me/player/next"); // create url extension

        self.check_scope("user-modify-playback-state")?; // check scope

        if let Some(device_id) = device_id {
            url_extension.push_str(&format!("?device_id={}", device_id)); // if device_id is supplied, then add it to url extension
        }

        self.spotify_request(&url_extension, RequestMethod::Post(HashMap::new()))?; // send request

        return Ok(())
    }

    /// Skips the currently playing track to the previous track: <https://developer.spotify.com/documentation/web-api/reference/#/operations/skip-users-playback-to-previous-track>
    /// 
    /// Requires scope: user-modify-playback-state
    /// 
    /// # Arguments
    /// * `device_id` - The id of the device to skip on
    /// 
    pub fn skip_previous(&mut self, device_id: Option<&str>) -> Result<(), SpotifyError> {
        let mut url_extension = String::from("me/player/previous"); // create url extension

        self.check_scope("user-modify-playback-state")?; // check scope

        if let Some(device_id) = device_id {
            url_extension.push_str(&format!("?device_id={}", device_id)); // if device_id is supplied, then add it to url extension
        }

        self.spotify_request(&url_extension, RequestMethod::Post(HashMap::new()))?; // send request

        return Ok(())
    }

    /// Seeks to specified position in currently playing track: <https://developer.spotify.com/documentation/web-api/reference/#/operations/seek-to-position-in-currently-playing-track> 
    /// 
    /// Requires scope: user-modify-playback-state
    /// 
    /// # Arguments
    /// * `position` - The position in milliseconds to seek to
    /// * `device_id` - The id of the device to seek on
    /// 
    pub fn seek_position(&mut self, position: i32, device_id: Option<&str>) -> Result<(), SpotifyError> {
        let mut url_extension = format!("me/player/seek?position_ms={}", position); // create url extension

        self.check_scope("user-modify-playback-state")?; // check scope

        if let Some(device_id) = device_id {
            url_extension.push_str(&format!("&device_id={}", device_id)); // if device_id is supplied, then add it to url extension
        }

        self.spotify_request(&url_extension, RequestMethod::Put(HashMap::new()))?; // send request

        return Ok(())
    }

    /// Sets the repeat mode for the user's playback: <https://developer.spotify.com/documentation/web-api/reference/#/operations/set-repeat-mode-on-users-playback> 
    /// 
    /// Requires scope: user-modify-playback-state
    /// 
    /// # Arguments
    /// * `state` - The repeat state to set. Valid values are: track, context, off
    /// * `device_id` - The id of the device to set repeat mode on
    /// 
    pub fn set_repeat_mode(&mut self, state: RepeatState, device_id: Option<&str>) -> Result<(), SpotifyError> {
        let mut url_extension = format!("me/player/repeat?state={}", state.to_string()); // create url extension

        self.check_scope("user-modify-playback-state")?; // check scope

        if let Some(device_id) = device_id {
            url_extension.push_str(&format!("&device_id={}", device_id)); // if device_id is supplied, then add it to url extension
        }

        self.spotify_request(&url_extension, RequestMethod::Put(HashMap::new()))?; // send request

        return Ok(())
    }

    /// Sets the volume for the user's playback: <https://developer.spotify.com/documentation/web-api/reference/#/operations/set-volume-for-users-playback> 
    /// 
    /// Requires scope: user-modify-playback-state
    /// 
    /// # Arguments
    /// * `volume` - The volume to set. Must be a value from 0 to 100 inclusive
    /// * `device_id` - The id of the device to set volume on
    /// 
    pub fn set_playback_volume(&mut self, volume: i32, device_id: Option<&str>) -> Result<(), SpotifyError> {
        let mut url_extension = format!("me/player/volume?volume_percent={}", volume); // create url extension

        self.check_scope("user-modify-playback-state")?; // check scope

        if let Some(device_id) = device_id {
            url_extension.push_str(&format!("&device_id={}", device_id)); // if device_id is supplied, then add it to url extension
        }

        self.spotify_request(&url_extension, RequestMethod::Put(HashMap::new()))?; // send request

        return Ok(())
    }

    /// Toggles shuffle state: <https://developer.spotify.com/documentation/web-api/reference/#/operations/toggle-shuffle-for-users-playback>
    /// 
    /// Requires scope: user-modify-playback-state
    /// 
    /// # Arguments
    /// * `state` - The shuffle state to set. Valid values are: true, false
    /// * `device_id` - The id of the device to set shuffle on
    /// 
    pub fn toggle_shuffle(&mut self, state: bool, device_id: Option<&str>) -> Result<(), SpotifyError> {
        let mut url_extension = format!("me/player/shuffle?state={}", state); // create url extension

        self.check_scope("user-modify-playback-state")?; // check scope

        if let Some(device_id) = device_id {
            url_extension.push_str(&format!("&device_id={}", device_id)); // if device_id is supplied, then add it to url extension
        }

        self.spotify_request(&url_extension, RequestMethod::Put(HashMap::new()))?; // send request

        return Ok(())
    }

    /// Returns user's recently played items: <https://developer.spotify.com/documentation/web-api/reference/#/operations/get-recently-played>
    /// 
    /// Requires scope: user-read-recently-played
    /// 
    /// # Arguments
    /// * `after` - Returns all items *after* this time stamp. 
    /// * `before` - Returns all items *before* this time stamp. Will be ignored if after is specified 
    /// * `limit` - The maximum number of items to return. Default: 20. Minimum: 1. Maximum: 50
    /// 
    pub fn get_recently_played_tracks(&mut self, after: Option<NaiveDateTime>, before: Option<NaiveDateTime>, limit: Option<i32>) -> Result<SpotifyCollection<PlayedTrack>, SpotifyError> {
        let mut url_extension = String::from("me/player/recently-played"); // create url extension

        self.check_scope("user-read-recently-played")?; // check scope

        if !after.is_none() || !before.is_none() || !limit.is_none() {
            url_extension.push_str("?"); // if any of the optional arguments are supplied, then add a ? to the url extension
        }

        if let Some(after) = after {
            url_extension.push_str(&format!("&after={}", after.timestamp_millis())); // if after is supplied, then add it to url extension
        }

        if after.is_none() { // only if after isn't supplied consider before
            if let Some(before) = before {
                url_extension.push_str(&format!("&before={}", before.timestamp_millis())); // if before is supplied, then add it to url extension
            }
        }

        if let Some(limit) = limit {
            url_extension.push_str(&format!("&limit={}", limit)); // if limit is supplied, then add it to url extension
        }

        let response = self.spotify_request(&url_extension, RequestMethod::Get)?; // send request

        return Ok(SpotifyCollection::<PlayedTrack>::new(&response)); // return response
    }

    /// Returns the user's currently playing track and queue: <https://developer.spotify.com/documentation/web-api/reference/#/operations/get-queue> 
    /// Note: This wrapper currently only supports tracks and unexpected errors may occur if the queue contains episodes
    /// 
    /// Requires scope: user-read-currently-playing user-read-playback-state
    /// 
    pub fn get_users_queue(&mut self) -> Result<(Track, Vec<Track>), SpotifyError> {
        let url_extension = String::from("me/player/queue"); // create url extension

        self.check_scope("user-read-currently-playing user-read-playback-state")?; // check scope

        let response = self.spotify_request(&url_extension, RequestMethod::Get)?; // send request

        let track = Track::new(&response["currently_playing"]); // get and format currently playing field

        let mut tracks = Vec::new(); // create vector to store tracks

        for track in response["queue"].members() {
            tracks.push(Track::new(&track)); // add track to vector
        }

        return Ok((track, tracks)); // return response
    }
}