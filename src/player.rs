use std::collections::HashMap;
use crate::srequest::RequestMethod;
use crate::spotify::{Spotify, SpotifyError, Playback, Device}; 
use serde_json::Value;

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
}