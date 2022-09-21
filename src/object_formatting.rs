use chrono::{NaiveDate, NaiveDateTime};
use json::{JsonValue::{self, Array, Null}};

use crate::spotify::{SpotifyImage, AlbumType, RestrictionReason, ReleaseDatePrecision, ExternalTrackIds, Album, Artist, Track, Tracks, Albums, DatedAlbum, DatedAlbums};

/// Take JsonValue object representing image from API request and turn into 
/// SpotifyImage object (for ease of use).
/// 
/// # Arguments 
/// * `image` - JsonValue object representing image from API request
/// 
/// # Panics 
/// If height or width value can't be converted to i32 (shouldn't happen)
/// 
fn format_image(image: &JsonValue) -> SpotifyImage {
    SpotifyImage {
        url: image["url"].to_string(),
        height: image["height"].as_i32().unwrap(),
        width: image["width"].as_i32().unwrap(),
    }
}

/// Takes JsonValue object representing possible external ids and formats them into ExternalTrackIds struct 
/// 
/// # Arguments
/// * `external_ids` - JsonValue object representing possible external ids
/// 
fn format_external_ids(external_ids: &JsonValue) -> ExternalTrackIds {
    ExternalTrackIds {
        isrc: match external_ids["isrc"].as_str() {
            Some(isrc) => Some(isrc.to_string()),
            None => None,
        },
        ean: match external_ids["ean"].as_str() {
            Some(ean) => Some(ean.to_string()),
            None => None,
        },
        upc: match external_ids["upc"].as_str() {
            Some(upc) => Some(upc.to_string()),
            None => None,
        },
    }
}

/// Takes JsonValue object representing album and formats it into struct for ease of use
/// 
/// # Arguments
/// * `raw_object` - JsonValue object representing album
/// 
pub fn format_album(raw_object: &JsonValue) -> Album {
    let album_type = match raw_object["album_type"].as_str() {
        Some("album") => AlbumType::Album,
        Some("single") => AlbumType::Single,
        Some("compilation") => AlbumType::Compilation,
        Some(_) => AlbumType::Album, // default to album
        None => AlbumType::Album, // default to album
    };

    let total_tracks = match raw_object["total_tracks"].as_i32() {
        Some(total_tracks) => total_tracks,
        None => 0, // default to 0
    };

    let available_markets: Vec<String> = match &raw_object["avaliable_markets"] {
        Array(markets) => {markets.iter().map(|market| market.to_string()).collect()}, // turn JsonValue Array type to vec of Strings 
        _ => vec![], // default to empty vec 
    };

    let spotify_url = &raw_object["external_urls"]["spotify"].to_string(); 

    let href = &raw_object["href"].to_string();

    let id = &raw_object["id"].to_string();

    let images = match &raw_object["images"] {
        Array(images) => {images.iter().map(|image| format_image(image)).collect()}, // turn JsonValue Array type to vec of SpotifyImage objects 
        _ => vec![], // default to empty vec 
    };

    let name = &raw_object["name"].to_string();

    let release_date_precision = match raw_object["release_date_precision"].as_str() {
        Some("year") => ReleaseDatePrecision::Year,
        Some("month") => ReleaseDatePrecision::Month,
        Some("day") => ReleaseDatePrecision::Day,
        Some(_) => ReleaseDatePrecision::None, // default to none
        None => ReleaseDatePrecision::None, // default to none
    };

    let release_date = {
        // parse string to date based on precision. Panic if unable to do so for whatever reason (hopefully this doens't happen, but it will be a problem for later me if it does)
        match &raw_object["release_date"] {
            Null => None, // default to no date
            date_string => {
                let date_string_temp = match release_date_precision {
                    ReleaseDatePrecision::Year => Some(NaiveDate::parse_from_str(&date_string.to_string(), "%Y")),
                    ReleaseDatePrecision::Month => Some(NaiveDate::parse_from_str(&date_string.to_string(), "%Y-%m")),
                    ReleaseDatePrecision::Day => Some(NaiveDate::parse_from_str(&date_string.to_string(), "%Y-%m-%d")),
                    ReleaseDatePrecision::None => None, // default to no date
                }; 
                match date_string_temp {
                    Some(Ok(date)) => Some(date),
                    Some(Err(_)) => None, // default to no date
                    None => None, // pass through none
                }
            }
        }
    };

    // This very well might not work becuase I didn't invest the time to find an album that actually had restrictions to test this
    let restriction_reason = match &raw_object["restrictions"]["reason"].as_str() {
        Some("market") => RestrictionReason::Market,
        Some("product") => RestrictionReason::Product,
        Some("explicit") => RestrictionReason::Explicit,
        _ => RestrictionReason::None, // default to none
    };

    let uri = &raw_object["uri"].to_string();

    let artists: Option<Vec<Artist>> = match &raw_object["artists"] {
        Array(artists) => {Some(artists.iter().map(|artist| format_artist(artist)).collect())}, // turn JsonValue Array type to vec of Artist objects 
        _ => None, // if artist array can't be found, return None
    };

    let tracks = match &raw_object["tracks"] {
        Null => None, 
        _ => Some(format_tracks(&raw_object["tracks"])),
    };

    Album {
        album_type,
        total_tracks,
        available_markets,
        spotify_url: spotify_url.to_string(),
        href: href.to_string(),
        id: id.to_string(),
        images,
        name: name.to_string(),
        release_date_precision,
        release_date,
        restriction_reason,
        uri: uri.to_string(),
        artists,
        tracks,
    }

}

/// Takes JsonValue object for DatedAlbum and formats it 
/// 
/// # Arguments
/// * `raw_object` - JsonValue object representing DatedAlbum
/// 
pub fn format_dated_album(raw_object: &JsonValue) -> DatedAlbum {
    let added_at = match &raw_object["added_at"] {
        Null => None, // default to no date
        date_string => Some(NaiveDateTime::parse_from_str(&date_string.to_string(), "%Y-%m-%dT%H:%M:%S%.fZ").unwrap()),
    };

    let album = format_album(&raw_object["album"]);

    DatedAlbum {
        date_added: added_at,
        album,
    }
}

/// Formats a set of albums from API request into struct for ease of use 
/// 
/// # Arguments
/// * `raw_object` - JsonValue object representing set of albums
/// 
pub fn format_dated_albums(raw_object: &JsonValue) -> DatedAlbums {
    let href = &raw_object["href"].to_string();

    let albums: Vec<DatedAlbum> = match &raw_object["items"] {
        Array(items) => {items.iter().map(|item| format_dated_album(item)).collect()}, // turn JsonValue Array type to vec of DatedAlbum objects 
        _ => vec![], // default to empty vec 
    };

    let limit = match raw_object["limit"].as_i32() {
        Some(limit) => limit,
        None => 0, // default to 0
    };

    let next = match &raw_object["next"] {
        Null => None,
        _ => Some(raw_object["next"].to_string()),
    };

    let offset = match raw_object["offset"].as_i32() {
        Some(offset) => offset,
        None => 0, // default to 0
    };

    let previous = match &raw_object["previous"] {
        Null => None,
        _ => Some(raw_object["previous"].to_string()),
    };

    let total = match raw_object["total"].as_i32() {
        Some(total) => total,
        None => 0, // default to 0
    };

    DatedAlbums {
        href: href.to_string(),
        albums,
        limit,
        next,
        offset,
        previous,
        total,
    }
}

/// Formats array of albums from API request into struct object for ease of use 
/// 
/// # Arguments
/// * `raw_object` - JsonValue object representing array of albums
///
pub fn format_albums(raw_object: &JsonValue) -> Albums {
    let href = &raw_object["href"].to_string();

    let albums: Vec<Album> = match &raw_object["items"] {
        Array(items) => {items.iter().map(|item| format_album(item)).collect()}, // turn JsonValue Array type to vec of Album objects 
        _ => vec![], // default to empty vec 
    };

    let limit = match raw_object["limit"].as_i32() {
        Some(limit) => limit,
        None => 0, // default to 0
    };

    let next = match &raw_object["next"] {
        Null => None,
        _ => Some(raw_object["next"].to_string()),
    };

    let offset = match raw_object["offset"].as_i32() {
        Some(offset) => offset,
        None => 0, // default to 0
    };

    let previous = match &raw_object["previous"] {
        Null => None,
        _ => Some(raw_object["previous"].to_string()),
    };

    let total = match raw_object["total"].as_i32() {
        Some(total) => total,
        None => 0, // default to 0
    };

    Albums {
        href: href.to_string(),
        albums,
        limit,
        next,
        offset,
        previous,
        total,
    }
}

/// Take JsonValue object representing artist from API request and turn into Artist struct for 
/// ease of use. 
/// 
/// # Arguments 
/// * `raw_object` - JsonValue object representing artist from API request 
/// 
pub fn format_artist(raw_object: &JsonValue) -> Artist {
    let spotify_url = &raw_object["external_urls"]["spotify"].to_string();

    let followers = match raw_object["followers"]["total"].as_i32() {
        Some(followers) => followers,
        None => 0, // default to 0
    };

    let genres: Vec<String> = match &raw_object["genres"] {
        Array(genres) => {genres.iter().map(|genre| genre.to_string()).collect()}, // turn JsonValue Array type to vec of Strings 
        _ => vec![], // default to empty vec 
    };

    let href = &raw_object["href"].to_string();

    let id = &raw_object["id"].to_string();

    let images = match &raw_object["images"] {
        Array(images) => {images.iter().map(|image| format_image(image)).collect()}, // turn JsonValue Array type to vec of SpotifyImage objects 
        _ => vec![], // default to empty vec 
    };

    let name = &raw_object["name"].to_string();

    let popularity = match raw_object["popularity"].as_i32() {
        Some(popularity) => popularity,
        None => 0, // default to 0
    };

    let uri = &raw_object["uri"].to_string();

    Artist {
        spotify_url: spotify_url.to_string(),
        total_followers: followers,
        genres,
        href: href.to_string(),
        id: id.to_string(),
        images,
        name: name.to_string(),
        popularity,
        uri: uri.to_string(),
    }
}

/// Formats array of tracks from API request into struct object for ease of use 
/// 
/// # Arguments 
/// * `raw_object` - JsonValue object representing tracks from API request
/// 
pub fn format_tracks(raw_object: &JsonValue) -> Tracks {
    let href = &raw_object["href"].to_string();

    let limit = match raw_object["limit"].as_i32() {
        Some(limit) => limit,
        None => 0, // default to 0
    };

    let next = match raw_object["next"] {
        Null => None, 
        _ => Some(raw_object["next"].to_string()), // if not null, assume string next url exists 
    };
    
    let offset = match raw_object["offset"].as_i32() {
        Some(offset) => offset,
        None => 0, // default to 0
    };

    let previous = match raw_object["previous"] {
        Null => None, 
        _ => Some(raw_object["previous"].to_string()), // if not null, assume string previous url exists 
    };

    let total = match raw_object["total"].as_i32() {
        Some(total) => total,
        None => 0, // default to 0
    };

    let tracks = match &raw_object["items"] {
        Array(tracks) => {tracks.iter().map(|track| format_track(track)).collect()}, // turn JsonValue Array type to vec of SpotifyImage objects 
        _ => vec![], // default to empty vec 
    };

    Tracks {
        href: href.to_string(),
        limit,
        next,
        offset,
        previous,
        total,
        tracks,
    }
}

/// Format a single track in the form of a JsonValue from API request into struct for ease of use 
/// 
/// # Arguments
/// * `raw_object` - JsonValue object representing track from API request
/// 
pub fn format_track(raw_object: &JsonValue) -> Track {
    let album = match &raw_object["album"] {
        Null => None, // if album object doesn't exist, return None 
        _ => Some(format_album(&raw_object["album"])), // if album object exists, format it
    };

    let artists: Option<Vec<Artist>> = match &raw_object["artists"] {
        Array(artists) => {Some(artists.iter().map(|artist| format_artist(artist)).collect())}, // turn JsonValue Array type to vec of Artist objects 
        _ => None, // default to None
    };

    let available_markets: Vec<String> = match &raw_object["available_markets"] {
        Array(markets) => {markets.iter().map(|market| market.to_string()).collect()}, // turn JsonValue Array type to vec of Strings 
        _ => vec![], // default to empty vec 
    };

    let disc_number = match raw_object["disc_number"].as_i32() {
        Some(disc_number) => disc_number,
        None => 0, // default to 0
    };

    let duration = match raw_object["duration_ms"].as_i32() {
        Some(duration) => duration,
        None => 0, // default to 0
    };

    let explicit = match raw_object["explicit"].as_bool() {
        Some(explicit) => explicit,
        None => false, // default to false
    };

    let external_ids = match &raw_object["external_ids"] {
        Null => ExternalTrackIds { isrc: None, ean: None, upc: None }, // if external_ids object doesn't exist, just set all ids to None 
        _ => format_external_ids(&raw_object["external_ids"]), // if external_ids object exists, format it
    };

    let spotify_url = &raw_object["external_urls"]["spotify"].to_string();

    let href = &raw_object["href"].to_string();

    let id = &raw_object["id"].to_string();

    let restriction_reason = match raw_object["restrictions"]["reason"].as_str() {
        Some("market") => RestrictionReason::Market,
        Some("product") => RestrictionReason::Product,
        Some("explicit") => RestrictionReason::Explicit,
        _ => RestrictionReason::None, // default to none
    };

    let name = &raw_object["name"].to_string();

    let popularity = match raw_object["popularity"].as_i32() {
        Some(popularity) => popularity,
        None => 0, // default to 0
    };

    let preview_url = match raw_object["preview_url"] {
        Null => None, 
        _ => Some(raw_object["preview_url"].to_string()), // if not null, assume string preview url exists 
    };

    let track_number = match raw_object["track_number"].as_i32() {
        Some(track_number) => track_number,
        None => 0, // default to 0
    };

    let uri = &raw_object["uri"].to_string();

    let is_local = match raw_object["is_local"].as_bool() {
        Some(is_local) => is_local,
        None => false, // default to false
    };

    Track {
        album,
        artists,
        available_markets,
        disc_number,
        duration,
        explicit,
        external_ids,
        spotify_url: spotify_url.to_string(),
        href: href.to_string(),
        id: id.to_string(),
        restriction_reason,
        name: name.to_string(),
        popularity,
        preview_url,
        track_number,
        uri: uri.to_string(),
        is_local,
    }
}