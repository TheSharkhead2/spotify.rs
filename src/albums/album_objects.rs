use url::Url;

use crate::Error;

/// Object representing a Spotify album id
pub struct AlbumId {
    id: String,
}

impl AlbumId {
    /// Returns the album id as a string
    pub fn id(&self) -> String {
        self.id
    }

    /// Constructs `AlbumId` object from a Spotify ID
    pub fn from_id(id: &str) -> Self {
        AlbumId { id: id.to_owned() }
    }

    /// Constructs `AlbumId` object from Spotify URI
    pub fn from_uri(uri: &str) -> Result<Self, Error> {
        let uri_components = uri.split(":").collect::<Vec<&str>>(); // split uri along colons

        // check for valid uri
        if uri_components.len() < 3 || uri_components.len() > 3 {
            // uri is too long for some reason
            return Err(Error::MalformedUri(uri.to_owned()));
        }

        if uri_components[0] != "spotify" {
            // first part should just be "spotify"
            return Err(Error::MalformedUri(uri.to_owned()));
        }

        if uri_components[1] != "album" {
            // this is the wrong type of uri
            return Err(Error::InvalidUriType(
                String::from("album"),
                uri_components[1].to_owned(),
            ));
        }

        // assume third item is a valid id (TODO: Add extra checks here?)
        Ok(AlbumId {
            id: uri_components[2].to_owned(),
        })
    }

    /// Constructs `AlbumId` object from Spotify URL
    pub fn from_url(url: &str) -> Result<Self, Error> {
        // check to make sure url is as expected
        let parsed_url = Url::parse(url)?;

        // check that it is the expected host
        if parsed_url.host_str() != Some("open.spotify.com") {
            return Err(Error::MalformedUrl(url.to_owned()));
        }

        // check to make sure it is the correct type
        // TODO: use parsed_url.path_segments() to look for album
    }
}
