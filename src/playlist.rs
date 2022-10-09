use std::collections::HashMap;
use chrono::NaiveDateTime;
use serde_json::{Value, Number, Map};
use crate::srequest::RequestMethod;
use crate::spotify::{Spotify, SpotifyError, SpotifyObject, Playlist, SpotifyCollection, PlaylistTrack, SpotifyImage};

impl Spotify {
    /// Get a playlist owned by a Spotify user: <https://developer.spotify.com/documentation/web-api/reference/#/operations/get-playlist> 
    /// Note: no support for episodes at the moment so unexpected results may occur with playlists that contain episodes
    /// 
    /// Required scope: none 
    /// 
    /// # Arguments
    /// * `playlist_id` - The Spotify ID of the playlist.
    /// * `market` - An ISO 3166-1 alpha-2 country code.
    ///
    pub fn get_playlist(&mut self, playlist_id: &str, market: Option<&str>) -> Result<Playlist, SpotifyError> {
        let mut url_extension = format!("playlists/{}?additional_types=track", playlist_id); // base url. Currently this only supports tracks, not episodes

        if let Some(market) = market { // if market is set, add to url
            url_extension.push_str(&format!("&market={}", market));
        }

        let response = self.spotify_request(&url_extension, RequestMethod::Get)?; // make request

        return Ok(Playlist::new(&response)); // format and return result
    }

    /// Change a playlist's name, public/private state, collaborative state, and description: <https://developer.spotify.com/documentation/web-api/reference/#/operations/change-playlist-details>
    /// 
    /// Required scope: playlist-modify-public playlist-modify-private
    /// 
    /// # Arguments
    /// * `playlist_id` - The Spotify ID of the playlist.
    /// * `name` - The new name for the playlist. 
    /// * `public` - If true the playlist will be public, if false it will be private.
    /// * `collaborative` - If true the playlist will become collaborative and other users will be able to modify the playlist in their Spotify client. Note: You can only set collaborative to true on non-public playlists.
    /// * `description` - Value for playlist description as displayed in Spotify Clients and in the Web API.
    /// 
    pub fn change_playlist_details(&mut self, playlist_id: &str, name: Option<&str>, public: Option<bool>, collaborative: Option<bool>, description: Option<&str>) -> Result<(), SpotifyError> {
        let url_extension = format!("playlists/{}", playlist_id); // base url

        let mut body: HashMap<String, Value> = HashMap::new(); // create body

        self.check_scope("playlist-modify-public playlist-modify-private")?;

        if name.is_none() && public.is_none() && collaborative.is_none() && description.is_none() { // if no arguments are set, return error
            return Err(SpotifyError::InvalidRequest("No arguments set. Must set one of: name, public, collaborative, description".to_string()));
        }

        if let Some(name) = name {
            body.insert(String::from("name"), Value::String(String::from(name)));
        }

        if let Some(public) = public {
            body.insert(String::from("public"), Value::Bool(public));
        }

        if let Some(collaborative) = collaborative {
            body.insert(String::from("collaborative"), Value::Bool(collaborative));
        }

        if let Some(description) = description {
            body.insert(String::from("description"), Value::String(String::from(description)));
        }

        self.spotify_request(&url_extension, RequestMethod::Put(body))?; // make request 

        Ok(())
    }

    /// Get all items in playlist: <https://developer.spotify.com/documentation/web-api/reference/#/operations/get-playlists-tracks> 
    /// Note: no support for episodes at the moment so unexpected results may occur with playlists that contain episodes
    /// 
    /// Required scope: none
    /// 
    /// # Arguments
    /// * `playlist_id` - The Spotify ID of the playlist.
    /// * `market` - An ISO 3166-1 alpha-2 country code.
    /// * `limit` - The maximum number of items to return. Default: 100. Minimum: 0. Maximum: 100.
    /// * `offset` - The index of the first item to return. Default: 0 (the first object). Use with limit to get the next set of items.
    /// 
    pub fn get_playlist_tracks(&mut self, playlist_id: &str, market: Option<&str>, limit: Option<i32>, offset: Option<i32>) -> Result<SpotifyCollection<PlaylistTrack>, SpotifyError> {
        let mut url_extension = format!("playlists/{}/tracks?additional_types=track", playlist_id); // base url. Currently this only supports tracks, not episodes

        if let Some(market) = market { // if market is set, add to url
            url_extension.push_str(&format!("&market={}", market));
        }

        if let Some(limit) = limit { // if limit is set, add to url
            url_extension.push_str(&format!("&limit={}", limit));
        }

        if let Some(offset) = offset { // if offset is set, add to url
            url_extension.push_str(&format!("&offset={}", offset));
        }

        let response = self.spotify_request(&url_extension, RequestMethod::Get)?; // make request

        return Ok(SpotifyCollection::<PlaylistTrack>::new(&response)); // format and return result
    }

    /// Add one or more tracks to a user's playlist: <https://developer.spotify.com/documentation/web-api/reference/#/operations/add-tracks-to-playlist>
    /// Note: currently only supports tracks, not episodes. 
    /// 
    /// Required scope: playlist-modify-public playlist-modify-private
    /// 
    /// # Arguments
    /// * `playlist_id` - The Spotify ID of the playlist.
    /// * `track_ids` - A list of Spotify track URIs to add, can be a maximum of 100. 
    /// * `position` - The position to insert the tracks, a zero-based index. For example, to insert the tracks in the first position: position=0; to insert the tracks in the third position: position=2. If omitted, the tracks will be appended to the playlist.
    /// 
    pub fn add_tracks_to_playlist(&mut self, playlist_id: &str, track_ids: Vec<&str>, position: Option<i32>) -> Result<(), SpotifyError> {
        let url_extension = format!("playlists/{}/tracks", playlist_id); // base url

        self.check_scope("playlist-modify-public playlist-modify-private")?;

        let mut track_uris: Vec<String> = Vec::new(); // create vector of track uris

        for track_id in track_ids { // add track ids to vector
            track_uris.push(format!("spotify:track:{}", track_id)); // format track ids into uris
        }

        let mut body: HashMap<String, Value> = HashMap::new(); // create body

        body.insert(String::from("uris"), Value::Array(track_uris.iter().map(|x| Value::String(x.to_string())).collect())); // add track uris to body

        if let Some(position) = position { // if position is set, add to body
            body.insert(String::from("position"), Value::Number(Number::from(position)));
        }

        self.spotify_request(&url_extension, RequestMethod::Post(body))?; // make request 

        Ok(())
    }

    /// Replace tracks in user's playlist. A derivative of: <https://developer.spotify.com/documentation/web-api/reference/#/operations/reorder-or-replace-playlists-tracks> 
    /// Returns the new snapshot ID of the playlist. 
    /// Note: Currently there is only support for spotifiy tracks, not episodes. 
    /// 
    /// Required scope: playlist-modify-public playlist-modify-private
    /// 
    /// # Arguments
    /// * `playlist_id` - The Spotify ID of the playlist.
    /// * `track_ids` - A list of Spotify track URIs to add, can be a maximum of 100.
    /// 
    pub fn replace_playlist_tracks(&mut self, playlist_id: &str, track_ids: Vec<&str>) -> Result<String, SpotifyError> {
        let url_extension = format!("playlists/{}/tracks", playlist_id); // base url

        self.check_scope("playlist-modify-public playlist-modify-private")?;

        let track_uris = Value::Array(track_ids.iter().map(|x| Value::String(format!("spotify:track:{}", x))).collect()); // create vector of track uris

        let mut body: HashMap<String, Value> = HashMap::new(); // create body

        body.insert(String::from("uris"), track_uris); // add track uris to body

        let response = self.spotify_request(&url_extension, RequestMethod::Put(body))?; // make request

        return match response["snapshot_id"].as_str() { // return snapshot id
            Some(snapshot_id) => Ok(String::from(snapshot_id)),
            None => Err(SpotifyError::RequestError(String::from("No snapshot id returned"))),
        };
    }

    /// Reorder tracks in user's playlist. A derivative of: <https://developer.spotify.com/documentation/web-api/reference/#/operations/reorder-or-replace-playlists-tracks>
    /// Returns the new snapshot ID of the playlist.
    /// 
    /// Required scope: playlist-modify-public playlist-modify-private
    /// 
    /// # Arguments
    /// * `playlist_id` - The Spotify ID of the playlist.
    /// * `range_start` - The position of the first track to be reordered.
    /// * `insert_before` - The position where the tracks should be inserted.
    /// * `range_length` - The amount of tracks to be reordered. Defaults to 1 if not set.
    /// * `snapshot_id` - The playlist's snapshot ID against which you want to make the changes.
    /// 
    pub fn reorder_playlist_tracks(&mut self, playlist_id: &str, range_start: i32, insert_before: i32, range_length: Option<i32>, snapshot_id: Option<&str>) -> Result<String, SpotifyError> {
        let url_extension = format!("playlists/{}/tracks", playlist_id); // base url

        self.check_scope("playlist-modify-public playlist-modify-private")?;

        let mut body: HashMap<String, Value> = HashMap::new(); // create body

        body.insert(String::from("range_start"), Value::Number(Number::from(range_start))); // add range start to body
        body.insert(String::from("insert_before"), Value::Number(Number::from(insert_before))); // add insert before to body

        if let Some(range_length) = range_length { // if range length is set, add to body
            body.insert(String::from("range_length"), Value::Number(Number::from(range_length)));
        }

        if let Some(snapshot_id) = snapshot_id { // if snapshot id is set, add to body
            body.insert(String::from("snapshot_id"), Value::String(String::from(snapshot_id)));
        }

        let response = self.spotify_request(&url_extension, RequestMethod::Put(body))?; // make request

        return match response["snapshot_id"].as_str() { // return snapshot id
            Some(snapshot_id) => Ok(String::from(snapshot_id)),
            None => Err(SpotifyError::RequestError(String::from("No snapshot id returned"))),
        };
    }
    
    /// Remove tracks from user's playlist: <https://developer.spotify.com/documentation/web-api/reference/#/operations/remove-tracks-playlist> 
    /// Returns the new snapshot ID of the playlist.
    /// Note: currently only support for spotify tracks, not episodes.
    /// 
    /// Required scope: playlist-modify-public playlist-modify-private
    /// 
    /// # Arguments
    /// * `playlist_id` - The Spotify ID of the playlist.
    /// * `track_ids` - A list of Spotify track URIs to remove, can be a maximum of 100.
    /// * `snapshot_id` - The playlist's snapshot ID against which you want to make the changes.
    /// 
    pub fn remove_playlist_tracks(&mut self, playlist_id: &str, track_ids: Vec<&str>, snapshot_id: Option<&str>) -> Result<String, SpotifyError> {
        let url_extension = format!("playlists/{}/tracks", playlist_id); // base url

        self.check_scope("playlist-modify-public playlist-modify-private")?;

        let mut body: HashMap<String, Value> = HashMap::new(); // create body

        // create a vector of Object mappings from "uri" to the uri of each track
        body.insert(String::from("tracks"), Value::Array(track_ids.iter().map(|x| Value::Object({ // insert tracks field into body
            let mut m = Map::new(); // new blank map for each object
            m.insert(String::from("uri"), Value::String(format!("spotify:track:{}", x))); // format track id into uri and insert into map under field "uri"
            m // return map into array
        })).collect())); // collect into array

        if let Some(snapshot_id) = snapshot_id { // if snapshot id is set, add to body
            body.insert(String::from("snapshot_id"), Value::String(String::from(snapshot_id)));
        }

        let response = self.spotify_request(&url_extension, RequestMethod::Delete(body))?; // make request

        return match response["snapshot_id"].as_str() { // return snapshot id
            Some(snapshot_id) => Ok(String::from(snapshot_id)),
            None => Err(SpotifyError::RequestError(String::from("No snapshot id returned"))),
        };
    }

    /// Get current user's playlists: <https://developer.spotify.com/documentation/web-api/reference/#/operations/get-a-list-of-current-users-playlists>
    /// 
    /// Required scope: playlist-read-private playlist-read-collaborative
    /// 
    /// # Arguments
    /// * `limit` - The maximum number of playlists to return. Default: 20. Minimum: 1. Maximum: 50.
    /// * `offset` - The index of the first playlist to return. Default: 0 (the first object). Use with limit to get the next set of playlists.
    /// 
    pub fn get_current_users_playlists(&mut self, limit: Option<i32>, offset: Option<i32>) -> Result<SpotifyCollection<Playlist>, SpotifyError> {
        let mut url_extension = String::from("me/playlists"); // base url

        self.check_scope("playlist-read-private playlist-read-collaborative")?;

        if !limit.is_none() || !offset.is_none() { // if one optional parameter is specified
            url_extension.push_str("?"); // add ? to url
        }

        if let Some(limit) = limit { // if limit is set, add to url
            url_extension.push_str(&format!("limit={}&", limit));
        }

        if let Some(offset) = offset { // if offset is set, add to url
            url_extension.push_str(&format!("offset={}&", offset));
        }

        let response = self.spotify_request(&url_extension, RequestMethod::Get)?; // make request

        return Ok(SpotifyCollection::<Playlist>::new(&response)); // return playlists
    }

    /// Get a specified user's playlists: <https://developer.spotify.com/documentation/web-api/reference/#/operations/get-list-users-playlists>
    /// 
    /// Required scope: playlist-read-private playlist-read-collaborative
    /// 
    /// # Arguments
    /// * `user_id` - The user's Spotify user ID.
    /// * `limit` - The maximum number of playlists to return. Default: 20. Minimum: 1. Maximum: 50.
    /// * `offset` - The index of the first playlist to return. Default: 0 (the first object). Use with limit to get the next set of playlists.
    /// 
    pub fn get_users_playlists(&mut self, user_id: &str, limit: Option<i32>, offset: Option<i32>) -> Result<SpotifyCollection<Playlist>, SpotifyError> {
        let mut url_extension = format!("users/{}/playlists", user_id); // base url

        self.check_scope("playlist-read-private playlist-read-collaborative")?;

        if !limit.is_none() || !offset.is_none() { // if one optional parameter is specified
            url_extension.push_str("?"); // add ? to url
        }

        if let Some(limit) = limit { // if limit is set, add to url
            url_extension.push_str(&format!("limit={}&", limit));
        }

        if let Some(offset) = offset { // if offset is set, add to url
            url_extension.push_str(&format!("offset={}&", offset));
        }

        let response = self.spotify_request(&url_extension, RequestMethod::Get)?; // make request

        return Ok(SpotifyCollection::<Playlist>::new(&response)); // return playlists
    }

    /// Create a playlist for the current user: <https://developer.spotify.com/documentation/web-api/reference/#/operations/create-playlist> 
    /// 
    /// Required scope: playlist-modify-public playlist-modify-private
    /// 
    /// # Arguments
    /// * `user_id` - The user's Spotify user ID.
    /// * `name` - The name for the new playlist 
    /// * `public` - Defaults to true. If true the playlist will be public, if false it will be private.
    /// * `collaborative` - Defaults to false. If true the playlist will be collaborative. Note that to create a collaborative playlist you must also set public to false.
    /// * `description` - Value for playlist description as displayed in Spotify Clients and in the Web API.
    /// 
    pub fn create_playlist(&mut self, user_id: &str, name: &str, public: Option<bool>, collaborative: Option<bool>, description: Option<&str>) -> Result<Playlist, SpotifyError> {
        let url_extension = format!("users/{}/playlists", user_id); // base url

        self.check_scope("playlist-modify-public playlist-modify-private")?;

        let mut body: HashMap<String, Value> = HashMap::new(); // create body

        body.insert(String::from("name"), Value::String(String::from(name))); // insert name into body

        if let Some(public) = public { // if public is set, add to body
            body.insert(String::from("public"), Value::Bool(public));
        }

        if let Some(collaborative) = collaborative { // if collaborative is set, add to body
            body.insert(String::from("collaborative"), Value::Bool(collaborative));
        }

        if let Some(description) = description { // if description is set, add to body
            body.insert(String::from("description"), Value::String(String::from(description)));
        }

        let response = self.spotify_request(&url_extension, RequestMethod::Post(body))?; // make request

        return Ok(Playlist::new(&response)); // return playlist
    }

    /// Gets playlists featured in Browse tab: <https://developer.spotify.com/documentation/web-api/reference/#/operations/get-featured-playlists> 
    /// 
    /// Required scope: none
    /// 
    /// # Arguments
    /// * `country` - An ISO 3166-1 alpha-2 country code. Will return the featured playlists for that country.
    /// * `locale` - The desired language, consisting of an ISO 639 language code and an ISO 3166-1 alpha-2 country code, joined by an underscore. For example: es_MX, meaning “Spanish (Mexico)”. Provide this parameter if you want the results returned in a particular language. Note that, if locale is not supplied, or if the specified language is not available, all strings will be returned in the Spotify default language (American English). The locale parameter, combined with the country parameter, may give odd results if not carefully matched. For example, country=SE&locale=de_DE will return a list of categories relevant to Sweden but as German language strings.
    /// * `limit` - The maximum number of items to return. Default: 20. Minimum: 1. Maximum: 50.
    /// * `offset` - The index of the first item to return. Default: 0 (the first object). Use with limit to get the next set of items.
    /// * `timestamp` - A timestamp for which the playlists would be relevant. Defaults to current time if not provided 
    /// 
    pub fn get_featured_playlists(&mut self, country: Option<&str>, locale: Option<&str>, limit: Option<i32>, offset: Option<i32>, timestamp: Option<NaiveDateTime>) -> Result<SpotifyCollection<Playlist>, SpotifyError> {
        let mut url_extension = String::from("browse/featured-playlists"); // base url

        if !country.is_none() || !locale.is_none() || !limit.is_none() || !offset.is_none() || !timestamp.is_none() { // if one optional parameter is specified
            url_extension.push_str("?"); // add ? to url
        }

        if let Some(country) = country { // if country is set, add to url
            url_extension.push_str(&format!("country={}&", country));
        }

        if let Some(locale) = locale { // if locale is set, add to url
            url_extension.push_str(&format!("locale={}&", locale));
        }

        if let Some(limit) = limit { // if limit is set, add to url
            url_extension.push_str(&format!("limit={}&", limit));
        }

        if let Some(offset) = offset { // if offset is set, add to url
            url_extension.push_str(&format!("offset={}&", offset));
        }

        if let Some(timestamp) = timestamp { // if timestamp is set, add to url
            url_extension.push_str(&format!("timestamp={}&", timestamp.format("%Y-%m-%dT%H:%M:%S")));
        }

        let response = self.spotify_request(&url_extension, RequestMethod::Get)?; // make request

        return Ok(SpotifyCollection::<Playlist>::new(&response["playlists"])) // return playlists (or albums?)
    }

    /// Get a set of Spotify playlsits tagged with a particular category: <https://developer.spotify.com/documentation/web-api/reference/#/operations/get-a-categories-playlists>
    /// 
    /// Required scope: none
    /// 
    /// # Arguments
    /// * `category_id` - The Spotify category ID for the category.
    /// * `country` - An ISO 3166-1 alpha-2 country code. 
    /// * `limit` - The maximum number of items to return. Default: 20. Minimum: 1. Maximum: 50.
    /// * `offset` - The index of the first item to return. Default: 0 (the first object). Use with limit to get the next set of items.
    /// 
    pub fn get_categorys_playlists(&mut self, category_id: &str, country: Option<&str>, limit: Option<i32>, offset: Option<i32>) -> Result<SpotifyCollection<Playlist>, SpotifyError> {
        let mut url_extension = format!("browse/categories/{}/playlists", category_id); // base url

        if !country.is_none() || !limit.is_none() || !offset.is_none() { // if one optional parameter is specified
            url_extension.push_str("?"); // add ? to url
        }

        if let Some(country) = country { // if country is set, add to url
            url_extension.push_str(&format!("country={}&", country));
        }

        if let Some(limit) = limit { // if limit is set, add to url
            url_extension.push_str(&format!("limit={}&", limit));
        }

        if let Some(offset) = offset { // if offset is set, add to url
            url_extension.push_str(&format!("offset={}&", offset));
        }

        let response = self.spotify_request(&url_extension, RequestMethod::Get)?; // make request

        return Ok(SpotifyCollection::<Playlist>::new(&response["playlists"])) // return playlists
    }

    /// Gets the current image associated with a playlist: <https://developer.spotify.com/documentation/web-api/reference/#/operations/get-playlist-cover> 
    /// 
    /// Required scope: none 
    /// 
    /// # Arguments
    /// * `playlist_id` - The Spotify ID for the playlist.
    /// 
    pub fn get_playlist_cover_image(&mut self, playlist_id: &str) -> Result<Vec<SpotifyImage>, SpotifyError> {
        let url_extension = format!("playlists/{}/images", playlist_id); // base url

        let response = self.spotify_request(&url_extension, RequestMethod::Get)?; // make request

        let mut images: Vec<SpotifyImage> = Vec::new(); // create vector to store images

        for image in response.members() {
            images.push(SpotifyImage::new(&image)); // add image to vector
        }

        return Ok(images); // return images
    }

    // TODO: implement: https://developer.spotify.com/documentation/web-api/reference/#/operations/upload-custom-playlist-cover. However, unclear how image is uploaded.
}