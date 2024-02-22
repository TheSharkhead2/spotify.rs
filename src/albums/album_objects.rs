use url::Url;

use crate::Error;

/// Object representing a Spotify album id
pub struct AlbumId {
    id: String,
}

impl AlbumId {
    /// Returns the album id as a string
    pub fn id(&self) -> &str {
        &self.id[..]
    }

    /// Constructs `AlbumId` object from a Spotify ID. Will not check validity
    pub fn from_id(id: &str) -> Self {
        AlbumId { id: id.to_owned() }
    }

    /// Constructs `AlbumId` object from a Spotify ID. Will attempt to check validity, but cannot be certain this is a valid AlbumId.
    pub fn try_from_id(id: &str) -> Result<Self, Error> {
        // check for empty string
        if id == "" {
            return Err(Error::InvalidId(id.to_owned()));
        }

        // Spotify requires that IDs are Base62, equivalent to checking alphanumeric
        if !id.chars().all(char::is_alphanumeric) {
            return Err(Error::InvalidId(id.to_owned()));
        }

        Ok(AlbumId { id: id.to_owned() })
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

        // don't allow blank id
        if uri_components[2] == "" {
            return Err(Error::MalformedUri(uri.to_owned()));
        }

        // spotify id needs to be alphanumeric (base62)
        if !uri_components[2].chars().all(char::is_alphanumeric) {
            return Err(Error::MalformedUri(uri.to_owned()));
        }

        // assume third item is a valid id (TODO: Add extra checks here?)
        Ok(AlbumId {
            id: uri_components[2].to_owned(),
        })
    }

    /// Constructs `AlbumId` object from Spotify URL
    pub fn from_url(url: &str) -> Result<Self, Error> {
        // checks for http:// at begining
        let mut url_to_parse = String::from(url);
        if &url[..7] != "http://" && &url[..8] != "https://" {
            // adding this lets url library parse correctly
            url_to_parse = String::from("http://") + url;
        }

        // check to make sure url is as expected
        let parsed_url = Url::parse(&url_to_parse[..])?;

        // check that it is the expected host
        if parsed_url.host_str() != Some("open.spotify.com") {
            return Err(Error::MalformedUrl(url.to_owned()));
        }

        // check to make sure it is the correct type
        let url_segments = parsed_url.path_segments();
        if let Some(url_segments) = url_segments {
            let mut url_seg_peek = url_segments.peekable(); // make iterator peekable

            // url needs to be specifying album here
            if url_seg_peek.next() != Some("album") {
                return Err(Error::InvalidUrlType(
                    String::from("album"),
                    String::from(*url_seg_peek.peek().unwrap_or_else(|| &"none")),
                ));
            }

            if let Some(album_id) = url_seg_peek.next() {
                if album_id == "" {
                    // don't allow blank id
                    return Err(Error::MalformedUrl(url.to_owned()));
                }

                // spotify id needs to be alphanumeric (base62)
                if !album_id.chars().all(char::is_alphanumeric) {
                    return Err(Error::MalformedUrl(url.to_owned()));
                }

                // TODO: add extra checks here?
                Ok(AlbumId {
                    id: String::from(album_id),
                })
            } else {
                // we got none, not a valid url
                Err(Error::MalformedUrl(url.to_owned()))
            }
        } else {
            // here can't parse segments, so somehow invalid
            Err(Error::MalformedUrl(url.to_owned()))
        }
    }
}

impl TryFrom<&str> for AlbumId {
    type Error = crate::Error;

    /// Attempts to convert provided string into AlbumId object. Will try to test for some pitfalls, but cannot guarantee it is a valid ID.
    fn try_from(value: &str) -> Result<AlbumId, Self::Error> {
        // first try to convert from url
        if let Ok(album_id) = AlbumId::from_url(value) {
            // it worked, so return
            return Ok(album_id);
        }

        // try uri
        if let Ok(album_id) = AlbumId::from_uri(value) {
            // if it worked, return
            return Ok(album_id);
        }

        // finally, try and create id
        AlbumId::try_from_id(value)
    }
}

impl TryFrom<String> for AlbumId {
    type Error = crate::Error;

    /// Attempts to convert provided String into AlbumId object. Will try to test for some common pitfalls, but cannot guarantee it is valid.
    fn try_from(value: String) -> Result<AlbumId, Self::Error> {
        // call logic for &str
        (&value[..]).try_into()
    }
}
