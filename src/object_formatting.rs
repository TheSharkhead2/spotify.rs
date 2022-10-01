use chrono::{NaiveDate, NaiveDateTime};
use json::{JsonValue::{self, Array, Null}};

use crate::spotify::{SpotifyImage, AlbumType, RestrictionReason, ReleaseDatePrecision, ExternalTrackIds, Album, Artist, Track, Tracks, Albums, DatedAlbum, DatedAlbums, DatedTrack, DatedTracks, FeatureTrack, AnalysisTrack, Bar, Beat, Section, Segment, Tatum, SpotifyError};

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

/// Takes JsonValue object representing a set of DatedTracks and formats them 
/// 
/// # Arguments
/// * `raw_object` - JsonValue object representing a set of DatedTracks
/// 
pub fn format_dated_tracks(raw_object: &JsonValue) -> DatedTracks {
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
        Array(tracks) => {tracks.iter().map(|track| format_dated_track(track)).collect()}, // turn JsonValue Array type to vec of SpotifyImage objects 
        _ => vec![], // default to empty vec 
    };

    DatedTracks {
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

/// Takes JsonValue for DatedTrack and formats it
/// 
/// # Arguments
/// * `raw_object` - JsonValue object representing DatedTrack from API request
/// 
pub fn format_dated_track(raw_object: &JsonValue) -> DatedTrack {
    let added_at = match &raw_object["added_at"] {
        Null => None, // default to no date
        date_string => Some(NaiveDateTime::parse_from_str(&date_string.to_string(), "%Y-%m-%dT%H:%M:%S%.fZ").unwrap()),
    };

    let track = format_track(&raw_object["track"]);

    DatedTrack {
        date_added: added_at,
        track,
    }
}

/// Takes JsonValue representing audio features for a track and formats it into FeatureTrack struct
/// 
/// # Arguments
/// * `raw_object` - JsonValue object representing audio features for a track from API request
/// 
pub fn format_feature_track(raw_object: &JsonValue) -> FeatureTrack {
    let acousticness = match raw_object["acousticness"].as_f64() {
        Some(acousticness) => acousticness,
        None => 0.0, // default to 0.0
    };

    let analysis_url = &raw_object["analysis_url"].to_string();

    let danceability = match raw_object["danceability"].as_f64() {
        Some(danceability) => danceability,
        None => 0.0, // default to 0.0
    };

    let duration = match raw_object["duration_ms"].as_i32() {
        Some(duration) => duration,
        None => 0, // default to 0
    };

    let energy = match raw_object["energy"].as_f64() {
        Some(energy) => energy,
        None => 0.0, // default to 0.0
    };

    let id = &raw_object["id"].to_string();

    let instrumentalness = match raw_object["instrumentalness"].as_f64() {
        Some(instrumentalness) => instrumentalness,
        None => 0.0, // default to 0.0
    };

    let key = match raw_object["key"].as_i32() {
        Some(key) => key,
        None => 0, // default to 0
    };

    let liveness = match raw_object["liveness"].as_f64() {
        Some(liveness) => liveness,
        None => 0.0, // default to 0.0
    };

    let loudness = match raw_object["loudness"].as_f64() {
        Some(loudness) => loudness,
        None => 0.0, // default to 0.0
    };

    let mode = match raw_object["mode"].as_i32() {
        Some(mode) => mode,
        None => 0, // default to 0
    };

    let speechiness = match raw_object["speechiness"].as_f64() {
        Some(speechiness) => speechiness,
        None => 0.0, // default to 0.0
    };

    let tempo = match raw_object["tempo"].as_f64() {
        Some(tempo) => tempo,
        None => 0.0, // default to 0.0
    };

    let time_signature = match raw_object["time_signature"].as_i32() {
        Some(time_signature) => time_signature,
        None => 0, // default to 0
    };

    let track_href = &raw_object["track_href"].to_string();

    let uri = &raw_object["uri"].to_string();

    let valence = match raw_object["valence"].as_f64() {
        Some(valence) => valence,
        None => 0.0, // default to 0.0
    };

    FeatureTrack {
        acousticness,
        analysis_url: analysis_url.to_string(),
        danceability,
        duration,
        energy,
        id: id.to_string(),
        instrumentalness,
        key,
        liveness,
        loudness,
        mode,
        speechiness,
        tempo,
        time_signature,
        track_href: track_href.to_string(),
        uri: uri.to_string(),
        valence,
    }
}

/// Takes JsonValue representing audion analysis and formats it into AnalysisTrack struct
/// 
/// # Arguments
/// * `raw_object` - JsonValue object representing audio analysis for a track from API request
///
pub fn format_analysis_track(raw_object: &JsonValue) -> Result<AnalysisTrack, SpotifyError> {
    // Check for errors before formatting. An error has occured on status code = 1
    if let Some(1) = raw_object["meta"]["status_code"].as_i32() {
        return Err(SpotifyError::FailedRequest(raw_object["meta"]["detailed_status"].to_string()))
    }

    let analyzer_version = raw_object["meta"]["analyzer_version"].to_string();

    let platform = raw_object["meta"]["platform"].to_string();

    let detailed_status = raw_object["meta"]["detailed_status"].to_string();

    let timestamp = match raw_object["meta"]["timestamp"].as_i64() {
        Some(timestamp) => timestamp,
        None => 0, // default to 0
    };

    let analysis_time = match raw_object["meta"]["analysis_time"].as_f64() {
        Some(analysis_time) => analysis_time,
        None => 0.0, // default to 0.0
    };

    let input_process = raw_object["meta"]["input_process"].to_string();

    let num_samples = match raw_object["track"]["num_samples"].as_i32() {
        Some(num_samples) => num_samples,
        None => 0, // default to 0
    };

    let duration = match raw_object["track"]["duration"].as_f64() {
        Some(duration) => duration,
        None => 0.0, // default to 0.0
    };

    let analysis_sample_rate = match raw_object["track"]["analysis_sample_rate"].as_i32() {
        Some(analysis_sample_rate) => analysis_sample_rate,
        None => 0, // default to 0
    };
    
    let analysis_channels = match raw_object["track"]["analysis_channels"].as_i32() {
        Some(analysis_channels) => analysis_channels,
        None => 0, // default to 0
    };

    let end_fade_in = match raw_object["track"]["end_of_fade_in"].as_f64() {
        Some(end_of_fade_in) => end_of_fade_in,
        None => 0.0, // default to 0.0
    };

    let start_fade_out = match raw_object["track"]["start_of_fade_out"].as_f64() {
        Some(start_of_fade_out) => start_of_fade_out,
        None => 0.0, // default to 0.0
    };

    let loudness = match raw_object["track"]["loudness"].as_f64() {
        Some(loudness) => loudness,
        None => 0.0, // default to 0.0
    };

    let tempo = match raw_object["track"]["tempo"].as_f64() {
        Some(tempo) => tempo,
        None => 0.0, // default to 0.0
    };

    let tempo_confidence = match raw_object["track"]["tempo_confidence"].as_f64() {
        Some(tempo_confidence) => tempo_confidence,
        None => 0.0, // default to 0.0
    };

    let time_signature = match raw_object["track"]["time_signature"].as_i32() {
        Some(time_signature) => time_signature,
        None => 0, // default to 0
    };

    let time_signature_confidence = match raw_object["track"]["time_signature_confidence"].as_f64() {
        Some(time_signature_confidence) => time_signature_confidence,
        None => 0.0, // default to 0.0
    };

    let key = match raw_object["track"]["key"].as_i32() {
        Some(key) => key,
        None => 0, // default to 0
    };

    let key_confidence = match raw_object["track"]["key_confidence"].as_f64() {
        Some(key_confidence) => key_confidence,
        None => 0.0, // default to 0.0
    };

    let mode = match raw_object["track"]["mode"].as_i32() {
        Some(mode) => mode,
        None => 0, // default to 0
    };

    let mode_confidence = match raw_object["track"]["mode_confidence"].as_f64() {
        Some(mode_confidence) => mode_confidence,
        None => 0.0, // default to 0.0
    };

    let code_string = raw_object["track"]["codestring"].to_string();

    let code_version = raw_object["track"]["code_version"].to_string();

    let echoprint_string = raw_object["track"]["echoprintstring"].to_string();

    let echoprint_version = raw_object["track"]["echoprint_version"].to_string();

    let synch_string = raw_object["track"]["synchstring"].to_string();

    let synch_version = raw_object["track"]["synch_version"].to_string();

    let rhythm_string = raw_object["track"]["rhythmstring"].to_string();

    let rhythm_version = raw_object["track"]["rhythm_version"].to_string();

    let mut bars: Vec<Bar> = Vec::new(); // empty vector for Bar objects

    for bar in raw_object["bars"].members() { // loop through array and format Bar objects
        let start = match bar["start"].as_f64() {
            Some(start) => start,
            None => 0.0, // default to 0.0
        };

        let duration = match bar["duration"].as_f64() {
            Some(duration) => duration,
            None => 0.0, // default to 0.0
        };

        let confidence = match bar["confidence"].as_f64() {
            Some(confidence) => confidence,
            None => 0.0, // default to 0.0
        };

        bars.push(Bar {
            start,
            duration,
            confidence,
        });
    }

    let mut beats: Vec<Beat> = Vec::new(); // empty vector for Beat objects

    for beat in raw_object["beats"].members() { // loop through array and format Beat objects
        let start = match beat["start"].as_f64() {
            Some(start) => start,
            None => 0.0, // default to 0.0
        };

        let duration = match beat["duration"].as_f64() {
            Some(duration) => duration,
            None => 0.0, // default to 0.0
        };

        let confidence = match beat["confidence"].as_f64() {
            Some(confidence) => confidence,
            None => 0.0, // default to 0.0
        };

        beats.push(Beat {
            start,
            duration,
            confidence,
        });
    }

    let mut sections: Vec<Section> = Vec::new(); // empty vector for Section objects

    for section in raw_object["sections"].members() { // loop through array and format Section objects
        let start = match section["start"].as_f64() {
            Some(start) => start,
            None => 0.0, // default to 0.0
        };

        let duration = match section["duration"].as_f64() {
            Some(duration) => duration,
            None => 0.0, // default to 0.0
        };

        let confidence = match section["confidence"].as_f64() {
            Some(confidence) => confidence,
            None => 0.0, // default to 0.0
        };

        let loudness = match section["loudness"].as_f64() {
            Some(loudness) => loudness,
            None => 0.0, // default to 0.0
        };

        let tempo = match section["tempo"].as_f64() {
            Some(tempo) => tempo,
            None => 0.0, // default to 0.0
        };

        let tempo_confidence = match section["tempo_confidence"].as_f64() {
            Some(tempo_confidence) => tempo_confidence,
            None => 0.0, // default to 0.0
        };

        let key = match section["key"].as_i32() {
            Some(key) => key,
            None => 0, // default to 0
        };

        let key_confidence = match section["key_confidence"].as_f64() {
            Some(key_confidence) => key_confidence,
            None => 0.0, // default to 0.0
        };

        let mode = match section["mode"].as_i32() {
            Some(mode) => mode,
            None => 0, // default to 0
        };

        let mode_confidence = match section["mode_confidence"].as_f64() {
            Some(mode_confidence) => mode_confidence,
            None => 0.0, // default to 0.0
        };

        let time_signature = match section["time_signature"].as_i32() {
            Some(time_signature) => time_signature,
            None => 0, // default to 0
        };

        let time_signature_confidence = match section["time_signature_confidence"].as_f64() {
            Some(time_signature_confidence) => time_signature_confidence,
            None => 0.0, // default to 0.0
        };

        sections.push(Section {
            start,
            duration,
            confidence,
            loudness,
            tempo,
            tempo_confidence,
            key,
            key_confidence,
            mode,
            mode_confidence,
            time_signature,
            time_signature_confidence,
        });
    }

    let mut segments: Vec<Segment> = Vec::new(); // empty vector for Segment objects

    for segment in raw_object["segments"].members() { // loop through array and format Segment objects
        let start = match segment["start"].as_f64() {
            Some(start) => start,
            None => 0.0, // default to 0.0
        };

        let duration = match segment["duration"].as_f64() {
            Some(duration) => duration,
            None => 0.0, // default to 0.0
        };

        let confidence = match segment["confidence"].as_f64() {
            Some(confidence) => confidence,
            None => 0.0, // default to 0.0
        };

        let loudness_start = match segment["loudness_start"].as_f64() {
            Some(loudness_start) => loudness_start,
            None => 0.0, // default to 0.0
        };

        let loudness_max_time = match segment["loudness_max_time"].as_f64() {
            Some(loudness_max_time) => loudness_max_time,
            None => 0.0, // default to 0.0
        };

        let loudness_max = match segment["loudness_max"].as_f64() {
            Some(loudness_max) => loudness_max,
            None => 0.0, // default to 0.0
        };

        let loudness_end = match segment["loudness_end"].as_f64() {
            Some(loudness_end) => loudness_end,
            None => 0.0, // default to 0.0
        };

        let pitches: Vec<f64> = segment["pitches"].members().map(|p| p.as_f64().unwrap()).collect();

        let timbre: Vec<f64> = segment["timbre"].members().map(|t| t.as_f64().unwrap()).collect();

        segments.push(Segment {
            start,
            duration,
            confidence,
            loudness_start,
            loudness_max_time,
            loudness_max,
            loudness_end,
            pitches,
            timbre,
        });
    }

    let mut tatums: Vec<Tatum> = Vec::new(); // empty vector for Tatum objects

    for tatum in raw_object["tatums"].members() { // loop through array and format Tatum objects
        let start = match tatum["start"].as_f64() {
            Some(start) => start,
            None => 0.0, // default to 0.0
        };

        let duration = match tatum["duration"].as_f64() {
            Some(duration) => duration,
            None => 0.0, // default to 0.0
        };

        let confidence = match tatum["confidence"].as_f64() {
            Some(confidence) => confidence,
            None => 0.0, // default to 0.0
        };

        tatums.push(Tatum {
            start,
            duration,
            confidence,
        });
    }

    Ok(AnalysisTrack {
        analyzer_version,
        platform,
        detailed_status,
        timestamp,
        analysis_time,
        input_process,
        num_samples,
        duration,
        analysis_sample_rate,
        analysis_channels,
        end_fade_in,
        start_fade_out,
        loudness,
        tempo,
        tempo_confidence,
        time_signature,
        time_signature_confidence,
        key,
        key_confidence,
        mode,
        mode_confidence,
        code_string,
        code_version,
        echoprint_string,
        echoprint_version,
        synch_string,
        synch_version,
        rhythm_string,
        rhythm_version,
        bars,
        beats,
        sections,
        segments,
        tatums,
    })

}