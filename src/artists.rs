use crate::spotify::{Album, Artist, Spotify, SpotifyCollection, SpotifyError, SpotifyObject};
use crate::srequest::RequestMethod;

impl Spotify {
    /// Get information on a single aritst: <https://developer.spotify.com/documentation/web-api/reference/#/operations/get-an-artist>
    ///  
    /// Required scope: none
    ///
    /// # Arguments
    /// * `artist_id` - The Spotify ID of the artist.
    ///  
    pub fn get_artist(&self, artist_id: &str) -> Result<Artist, SpotifyError> {
        let url_extension = format!("artists/{}", artist_id);

        let response = self.spotify_request(&url_extension, RequestMethod::Get)?; // make request

        return Ok(Artist::new(&response)); // format and return result
    }

    /// Gets information on several artists: <https://developer.spotify.com/documentation/web-api/reference/#/operations/get-multiple-artists>
    ///
    /// Required scope: none
    ///
    /// # Arguments
    /// * `artist_ids` - A vector of the Spotify IDs for the artists. Maximum: 50 IDs.
    ///
    pub fn get_several_artists(
        &self,
        artist_ids: Vec<&str>,
    ) -> Result<Vec<Artist>, SpotifyError> {
        let url_extension = format!("artists/?ids={}", artist_ids.join(",")); // base url with artist ids added

        let response = self.spotify_request(&url_extension, RequestMethod::Get)?; // make request

        let mut artists = Vec::new(); // create vector to store artists
        for artist in response["artists"].members() {
            artists.push(Artist::new(&artist)); // format artist and push to vector
        }
        return Ok(artists); // return vector of artists
    }

    /// Get artist's albums: <https://developer.spotify.com/documentation/web-api/reference/#/operations/get-an-artists-albums>
    ///
    /// Required scope: none
    ///
    /// # Arguments
    /// * `artist_id` - The Spotify ID of the artist.
    /// * `include_groups` - A list of keywords that will be used to filter the response. If not supplied, all album types will be returned. Valid values: album, single, appears_on, compilation.
    /// * `market` - An ISO 3166-1 alpha-2 country code.
    /// * `limit` - The maximum number of items to return. Default: 20. Minimum: 1. Maximum: 50.
    /// * `offset` - The index of the first item to return. Default: 0 (the first object). Use with limit to get the next set of items.
    ///
    pub fn get_artist_albums(
        &self,
        artist_id: &str,
        include_groups: Option<Vec<&str>>,
        limit: Option<u32>,
        market: Option<&str>,
        offset: Option<u32>,
    ) -> Result<SpotifyCollection<Album>, SpotifyError> {
        let mut url_extension = format!("artists/{}/albums", artist_id); // base url

        if include_groups != None || limit != None || market != None || offset != None {
            // if any optional parameters are set, add query question mark
            url_extension.push_str("?");
        }

        if let Some(include_groups) = include_groups {
            // if include_groups is set, add to url
            url_extension.push_str(&format!("&include_groups={}", include_groups.join(",")));
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

        return Ok(SpotifyCollection::<Album>::new(&response)); // format and return result
    }

    /// Get artist's top tracks: <https://developer.spotify.com/documentation/web-api/reference/#/operations/get-an-artists-top-tracks>
    ///
    /// Required scope: none
    ///
    /// # Arguments
    /// * `artist_id` - The Spotify ID of the artist.
    /// * `market` - An ISO 3166-1 alpha-2 country code.
    ///
    pub fn get_artist_top_tracks(
        &self,
        artist_id: &str,
        market: &str,
    ) -> Result<Vec<Artist>, SpotifyError> {
        let url_extension = format!("artists/{}/top-tracks?market={}", artist_id, market); // base url

        let response = self.spotify_request(&url_extension, RequestMethod::Get)?; // make request

        let mut artists = Vec::new(); // create vector to store artists
        for artist in response["tracks"].members() {
            artists.push(Artist::new(&artist)); // format artist and push to vector
        }
        return Ok(artists); // return vector of artists
    }

    /// Get artist's related to specified artist: <https://developer.spotify.com/documentation/web-api/reference/#/operations/get-an-artists-related-artists>
    ///
    /// Required scope: none
    ///
    /// # Arguments
    /// * `artist_id` - The Spotify ID of the artist.
    ///
    pub fn get_artist_related_artists(
        &self,
        artist_id: &str,
    ) -> Result<Vec<Artist>, SpotifyError> {
        let url_extension = format!("artists/{}/related-artists", artist_id); // base url

        let response = self.spotify_request(&url_extension, RequestMethod::Get)?; // make request

        let mut artists = Vec::new(); // create vector to store artists
        for artist in response["artists"].members() {
            artists.push(Artist::new(&artist)); // format artist and push to vector
        }
        return Ok(artists); // return vector of artists
    }
}
