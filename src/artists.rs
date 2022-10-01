use crate::srequest::RequestMethod;
use crate::spotify::{Spotify, SpotifyError, Artist, Albums};
use crate::object_formatting::{format_artist, format_albums};

impl Spotify {
    /// Get information on a single aritst: https://developer.spotify.com/documentation/web-api/reference/#/operations/get-an-artist 
    /// Required scope: none 
    pub fn get_artist(&mut self, artist_id: &str) -> Result<Artist, SpotifyError> {
        let url_extension = format!("artists/{}", artist_id);

        let response = self.spotify_request(&url_extension, RequestMethod::Get)?; // make request

        return Ok(format_artist(&response)); // format and return result
    }

    /// Gets information on several artists: https://developer.spotify.com/documentation/web-api/reference/#/operations/get-multiple-artists
    /// Required scope: none
    pub fn get_several_artists(&mut self, artist_ids: Vec<&str>) -> Result<Vec<Artist>, SpotifyError> {
        let url_extension = format!("artists/?ids={}", artist_ids.join(",")); // base url with artist ids added

        let response = self.spotify_request(&url_extension, RequestMethod::Get)?; // make request

        let mut artists = Vec::new(); // create vector to store artists
        for artist in response["artists"].members() {
            artists.push(format_artist(&artist)); // format artist and push to vector
        }
        return Ok(artists) // return vector of artists
    }

    /// Get artist's albums: https://developer.spotify.com/documentation/web-api/reference/#/operations/get-an-artists-albums
    /// Required scope: none
    pub fn get_artist_albums(&mut self, artist_id: &str, include_groups: Option<Vec<&str>>, limit: Option<u32>, market: Option<&str>, offset: Option<u32>) -> Result<Albums, SpotifyError>{
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

        let response = self.spotify_request(&url_extension, RequestMethod::Get)?; // make request

        return Ok(format_albums(&response)); // format and return result
    }

    /// Get artist's top tracks: https://developer.spotify.com/documentation/web-api/reference/#/operations/get-an-artists-top-tracks
    /// Required scope: none 
    pub fn get_artist_top_tracks(&mut self, artist_id: &str, market: &str) -> Result<Vec<Artist>, SpotifyError> {
        let url_extension = format!("artists/{}/top-tracks?market={}", artist_id, market); // base url 

        let response = self.spotify_request(&url_extension, RequestMethod::Get)?; // make request

        let mut artists = Vec::new(); // create vector to store artists
        for artist in response["tracks"].members() {
            artists.push(format_artist(&artist)); // format artist and push to vector
        }
        return Ok(artists) // return vector of artists
    }

    /// Get artist's related to specified artist: https://developer.spotify.com/documentation/web-api/reference/#/operations/get-an-artists-related-artists
    /// Required scope: none
    pub fn get_artist_related_artists(&mut self, artist_id: &str) -> Result<Vec<Artist>, SpotifyError> {
        let url_extension = format!("artists/{}/related-artists", artist_id); // base url 

        let response = self.spotify_request(&url_extension, RequestMethod::Get)?; // make request

        let mut artists = Vec::new(); // create vector to store artists
        for artist in response["artists"].members() {
            artists.push(format_artist(&artist)); // format artist and push to vector
        }
        return Ok(artists) // return vector of artists
    }
}