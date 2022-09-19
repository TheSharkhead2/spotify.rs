use chrono::{DateTime, Utc, Duration, NaiveDate, NaiveDateTime};
use dotenv;
use std::fmt;

use crate::authorization::{generate_verifier, get_authorization_code, get_access_token, refresh_access_token};

/// Struct to represent Spotify images (album art, etc.)
pub struct SpotifyImage {
    pub url: String,
    pub height: i32, 
    pub width: i32,
}

/// Enum to represent three states of album type 
pub enum AlbumType {
    Album,
    Single,
    Compilation,
}

/// Enum to represent reason for album restriction 
pub enum RestrictionReason {
    Market,
    Product,
    Explicit,
    None,
}

// Enum to represent release date precision 
pub enum ReleaseDatePrecision {
    Year,
    Month,
    Day,
    None,
}

/// struct to hold known external ids for tracks
pub struct ExternalTrackIds {
    pub isrc: Option<String>,
    pub ean: Option<String>,
    pub upc: Option<String>,
}

/// Struct to represent Album 
pub struct Album {
    pub album_type: AlbumType, // Type of album: album, single, compilation 
    pub total_tracks: i32, // The number of tracks in album 
    pub available_markets: Vec<String>, // The markets in which the album is available: ISO 3166-1 alpha-2 country codes (Note: considered in market if at least 1 song is in that market)
    pub spotify_url: String, // The Spotify URL for the album
    pub href: String, // A link to the Web API endpoint providing full details of the album
    pub id: String, // The Spotify ID for the album
    pub images: Vec<SpotifyImage>, // The cover art for the album in various sizes, widest first
    pub name: String, // The name of the album. In case of an album takedown, the value may be an empty string
    pub release_date: Option<NaiveDate>, // The date the album was first released 
    pub release_date_precision: ReleaseDatePrecision, // The precision with which release_date value is known: year, month, or day
    pub restriction_reason: RestrictionReason, // The reason for an album being restricted. Albums may be restricted if the content is not available in a given market, to the user's subscription type, or when the user's account is set to not play explicit content.
    pub uri: String, // The Spotify URI for the album  
    pub artists: Option<Vec<Artist>>, // The artists of the album. Can be None
    pub tracks: Option<Tracks>, // The tracks of the album. Can be None
}

/// Implements Debug trait for Album struct
impl fmt::Debug for Album {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Album")
            .field("name", &self.name)
            .field("total_tracks", &self.total_tracks)
            .field("id", &self.id)
            .field("release_date", &self.release_date)
            .field("artists", &self.artists)
            .field("tracks", &self.tracks)
            .finish()
    }
}

/// Struct to represent Album with "date_added" field 
pub struct DatedAlbum {
    pub album: Album, // The album
    pub date_added: Option<NaiveDateTime>, // The date the album was added to the user's library
}

/// Implement Debug trait for DatedAlbum struct
impl fmt::Debug for DatedAlbum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DatedAlbum")
            .field("album", &self.album)
            .field("date_added", &self.date_added)
            .finish()
    }
}

/// Struct to represent a set of Albums
pub struct Albums {
    pub href: String, // A link to the Web API endpoint returning the full result of the request
    pub albums: Vec<Album>, // The requested data
    pub limit: i32, // The maximum number of items in the response (as set in the query or by default)
    pub next: Option<String>, // URL to the next page of items. (null if none)
    pub offset: i32, // The offset of the items returned (as set in the query or by default)
    pub previous: Option<String>, // URL to the previous page of items. (null if none)
    pub total: i32, // The total number of items available to return
}

impl fmt::Debug for Albums {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Albums")
            .field("albums", &self.albums)
            .field("limit", &self.limit)
            .field("offset", &self.offset)
            .field("total", &self.total)
            .finish()
    }
}

/// Struct to represent a set of dated Albums 
pub struct DatedAlbums {
    pub href: String, // A link to the Web API endpoint returning the full result of the request
    pub albums: Vec<DatedAlbum>, // The requested data
    pub limit: i32, // The maximum number of items in the response (as set in the query or by default)
    pub next: Option<String>, // URL to the next page of items. (null if none)
    pub offset: i32, // The offset of the items returned (as set in the query or by default)
    pub previous: Option<String>, // URL to the previous page of items. (null if none)
    pub total: i32, // The total number of items available to return
}

/// Implement Debug trait for DatedAlbums struct
impl fmt::Debug for DatedAlbums {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DatedAlbums")
            .field("albums", &self.albums)
            .field("limit", &self.limit)
            .field("offset", &self.offset)
            .field("total", &self.total)
            .finish()
    }
}

/// Struct to represent Artist 
pub struct Artist {
    pub spotify_url: String, // The Spotify URL for the artist
    pub total_followers: i32,  // The total number of followers
    pub genres: Vec<String>, // A list of the genres the artist is associated with. If not yet classified, the array is empty. 
    pub href: String, // A link to the Web API endpoint providing full details of the artist
    pub id: String, // The Spotify ID for the artist
    pub images: Vec<SpotifyImage>, // Images of the artist in various sizes, widest first
    pub name: String, // The name of the artist
    pub popularity: i32, // The popularity of the artist. The value will be between 0 and 100, with 100 being the most popular. The artist's popularity is calculated from the popularity of all the artist's tracks
    pub uri: String, // The Spotify URI for the artist
}

/// Implements Debug trait for Artist struct
impl fmt::Debug for Artist {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Artist")
            .field("id", &self.id)
            .field("name", &self.name)
            .finish()
    }
}

/// Struct to represent Track
pub struct Track {
    pub album: Option<Album>, // The album on which the track appears. 
    pub artists: Option<Vec<Artist>>, // The artists who performed the track. 
    pub available_markets: Vec<String>, // A list of the countries in which the track can be played, identified by their ISO 3166-1 alpha-2 code.
    pub disc_number: i32, // The disc number (usually 1 unless the album consists of more than one disc)
    pub duration: i32, // The track length in milliseconds
    pub explicit: bool, // Whether or not the track has explicit lyrics ( true = yes it does; false = no it does not OR unknown) 
    pub external_ids: ExternalTrackIds, // Known external IDs for the track 
    pub spotify_url: String, // The Spotify URL for the track
    pub href: String, // A link to the Web API endpoint providing full details of the track
    pub id: String, // The Spotify ID for the track
    pub restriction_reason: RestrictionReason, // The reason for the track being restricted. If a track is restricted, the reason is usually market or explicit.
    pub name: String, // The name of the track
    pub popularity: i32, // The popularity of the track. The value will be between 0 and 100, with 100 being the most popular. 
    pub preview_url: Option<String>, // A URL to a 30 second preview (MP3 format) of the track.
    pub track_number: i32, // The number of the track. If an album has several discs, the track number is the number on the specified disc.
    pub uri: String, // The Spotify URI for the track
    pub is_local: bool,
}

/// Implements Debug trait for Track struct
impl fmt::Debug for Track {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Track")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("artists", &self.artists)
            .finish()
    }
}

/// Struct to represent several Tracks and keep track of offsets and limits 
pub struct Tracks {
    pub href: String, // A link to the Web API endpoint returning the full result of the request
    pub tracks: Vec<Track>, // The requested data
    pub limit: i32, // The maximum number of items in the response (as set in the query or by default)
    pub next: Option<String>, // URL to next page of items, None if none 
    pub offset: i32, // The offset of the items returned (as set in the query or by default)
    pub previous: Option<String>, // URL to previous page of items, None if none
    pub total: i32, // The total number of items available to return
}

/// Implements Debug trait for Tracks struct
impl fmt::Debug for Tracks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Tracks")
            .field("tracks", &self.tracks)
            .field("limit", &self.limit)
            .field("offset", &self.offset)
            .field("total", &self.total)
            .finish()
    }
}

/// Error object for Spotify struct
pub enum SpotifyError {
    AccessTokenExpired,
    RequestError(String),
    InsufficientScope(String),
    // Unknown,
}

/// Implemntation of formatting such that SpotfiyError can be printed
impl fmt::Display for SpotifyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SpotifyError::AccessTokenExpired => write!(f, "Access token expired, please refresh"),
            SpotifyError::RequestError(e) => write!(f, "Request error: {}", e),
            SpotifyError::InsufficientScope(scopes) => write!(f, "Insufficient scope. Need: {:?}", scopes),
            // SpotifyError::Unknown => write!(f, "Unknown error"),
        }
    }
}
impl fmt::Debug for SpotifyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SpotifyError::AccessTokenExpired => write!(f, "Access token expired, please refresh"),
            SpotifyError::RequestError(e) => write!(f, "Request error: {}", e),
            SpotifyError::InsufficientScope(scopes) => write!(f, "Insufficient scope. Need: {:?}", scopes),
            // SpotifyError::Unknown => write!(f, "Unknown error"),
        }
    }
}

/// Wrapper object for Spotify API
pub struct Spotify {
    client_id: String,
    scope: String,
    access_token: String,
    refresh_token: String,
    expires_at: DateTime<Utc>,
}

impl Spotify {
    pub fn authenticate(localhost_port: String, scope: String) -> Spotify {
        let client_id = dotenv::var("CLIENT_ID").unwrap(); // grab client_id from .env

        let (code_verifier, code_challenge) = generate_verifier(); // generate code verifier and code challenge

        let redirect_uri = format!("http://localhost:{}/callback", &localhost_port); // redirect uri for authorization code endpoint

        let auth_code_result = get_authorization_code(&client_id, &localhost_port, &redirect_uri, &scope, &code_challenge);

        let (access_token, refresh_token, expires_in) = match auth_code_result {
            Ok(auth_code) => {
                 get_access_token(&auth_code, &client_id, &code_verifier, &redirect_uri).unwrap() // get access token (be lazy with error handling and just panic if request is bad)
            },
            Err(e) => panic!("{}", e),
        };

        let expires_at = Utc::now() + Duration::seconds(expires_in); // get time when access token expires

        Spotify {
            client_id: client_id,
            scope: scope,
            access_token: access_token,
            refresh_token: refresh_token,
            expires_at: expires_at,
        }
    }

    /// Checks to see if required scope is present in current scope 
    /// 
    /// # Arguments
    /// * `scope` - A string slice that holds required scope
    /// 
    pub fn check_scope(&self, scope: &str) -> Result<(), SpotifyError> {
        let scopes: Vec<&str> = self.scope.split_whitespace().collect();
        let required_scopes: Vec<&str> = scope.split_whitespace().collect();

        let missing_scopes: Vec<&str> = required_scopes.iter().copied().filter(|s| !scopes.contains(s)).collect();

        if missing_scopes.len() > 0 {
            return Err(SpotifyError::InsufficientScope(missing_scopes.join(" ")));
        }

        Ok(())
    }

    pub fn access_token(&self) -> Result<String, SpotifyError> {
        // if access token is expired, return error, otherwise return access token
        if Utc::now() < self.expires_at {
            return Ok(self.access_token.clone())
        } else {
            
            return Err(SpotifyError::AccessTokenExpired) // if access token is expired, need new Spotify object, return error so user can refresh
        }
    }

    pub fn refresh(&self) -> Spotify {
        let (access_token, expires_in) = match refresh_access_token(&self.refresh_token, &self.client_id) {
            Ok((access_token, expires_in)) => (access_token, expires_in), 
            Err(e) => panic!("{}", e), // on error panic
        };
        
        let expires_at = Utc::now() + Duration::seconds(expires_in); // get time when access token expires

        // return new Spotify object with refreshed access token
        Spotify {
            client_id: self.client_id.clone(),
            scope: self.scope.clone(),
            access_token: access_token,
            refresh_token: self.refresh_token.clone(),
            expires_at: expires_at,
        }
    }
}
