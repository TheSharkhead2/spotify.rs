use crate::srequest::RequestMethod;
use crate::spotify::{Spotify, SpotifyError, SpotifyObject, Category, SpotifyCollection};

impl Spotify {
    /// Get a set of categories used to tag items in Spotify: <https://developer.spotify.com/documentation/web-api/reference/#/operations/get-categories> 
    /// 
    /// Requires scope: None
    /// 
    /// # Arguments
    /// * `country` - An ISO 3166-1 alpha-2 country code. 
    /// * `locale` - The desired language, consisting of an ISO 639 language code and an ISO 3166-1 alpha-2 country code, joined by an underscore.
    /// * `limit` - The maximum number of categories to return. Default: 20. Minimum: 1. Maximum: 50. 
    /// * `offset` - The index of the first category to return. Default: 0 (the first object). Use with limit to get the next set of categories.
    /// 
    pub fn get_several_browse_categories(&mut self, country: Option<&str>, locale: Option<&str>, limit: Option<i32>, offset: Option<i32>) -> Result<SpotifyCollection<Category>, SpotifyError> {
        let mut url_extension = String::from("browse/categories"); // base url 

        if !country.is_none() || !locale.is_none() || !limit.is_none() || !offset.is_none() { // if any optional arguments are present
            url_extension.push_str("?"); // add ? to url
        }

        if let Some(country) = country {
            url_extension.push_str(&format!("country={}&", country)); // add country to url
        }

        if let Some(locale) = locale {
            url_extension.push_str(&format!("locale={}&", locale)); // add locale to url
        }

        if let Some(limit) = limit {
            url_extension.push_str(&format!("limit={}&", limit)); // add limit to url
        }

        if let Some(offset) = offset {
            url_extension.push_str(&format!("offset={}&", offset)); // add offset to url
        }


        let response = self.spotify_request(&url_extension, RequestMethod::Get)?; // send request

        Ok(SpotifyCollection::<Category>::new(&response["categories"])) // return collection
    }

    /// Gets a single Spotify category: <https://developer.spotify.com/documentation/web-api/reference/#/operations/get-a-category> 
    /// 
    /// Requires scope: None
    /// 
    /// # Arguments
    /// * `category_id` - The Spotify category ID for the category.
    /// * `country` - An ISO 3166-1 alpha-2 country code.
    /// * `locale` - The desired language, consisting of an ISO 639 language code and an ISO 3166-1 alpha-2 country code, joined by an underscore.
    /// 
    pub fn get_single_browse_category(&mut self, category_id: &str, country: Option<&str>, locale: Option<&str>) -> Result<Category, SpotifyError> {
        let mut url_extension = format!("browse/categories/{}", category_id); // base url 

        if !country.is_none() || !locale.is_none() { // if any optional arguments are present
            url_extension.push_str("?"); // add ? to url
        }

        if let Some(country) = country {
            url_extension.push_str(&format!("country={}&", country)); // add country to url
        }

        if let Some(locale) = locale {
            url_extension.push_str(&format!("locale={}&", locale)); // add locale to url
        }

        let response = self.spotify_request(&url_extension, RequestMethod::Get)?; // send request

        Ok(Category::new(&response)) // return category
    } 
}