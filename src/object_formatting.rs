use chrono::{NaiveDate, NaiveDateTime};
use json::JsonValue::{self, Array, Null};
use std::fmt::Debug;

use crate::spotify::{
    Album, AlbumType, AnalysisTrack, Artist, Bar, Beat, Category, DatedAlbum, DatedTrack, Device,
    ExternalTrackIds, FeatureTrack, Playback, PlaybackActions, PlayedTrack, Playlist,
    PlaylistTrack, ReleaseDatePrecision, RepeatState, RestrictionReason, Section, Segment,
    SpotifyCollection, SpotifyContext, SpotifyError, SpotifyImage, SpotifyObject, Tatum, Track,
    User,
};

impl SpotifyImage {
    /// Take JsonValue object representing image from API request and turn into
    /// SpotifyImage object (for ease of use).
    ///
    /// # Arguments
    /// * `image` - JsonValue object representing image from API request
    ///
    /// # Panics
    /// If height or width value can't be converted to i32 (shouldn't happen)
    ///
    pub fn new(image: &JsonValue) -> SpotifyImage {
        SpotifyImage {
            url: image["url"].to_string(),
            height: match image["height"].as_i32() {
                Some(height) => height,
                None => 0, // default to 0
            },
            width: match image["width"].as_i32() {
                Some(width) => width,
                None => 0, // default to 0
            },
        }
    }
}

impl ExternalTrackIds {
    /// Takes JsonValue object representing possible external ids and formats them into ExternalTrackIds struct
    ///
    /// # Arguments
    /// * `external_ids` - JsonValue object representing possible external ids
    ///
    fn new(external_ids: &JsonValue) -> ExternalTrackIds {
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
}

impl SpotifyObject for Album {
    /// Takes JsonValue object representing album and formats it into struct for ease of use
    ///
    /// # Arguments
    /// * `raw_object` - JsonValue object representing album
    ///
    fn new(raw_object: &JsonValue) -> Album {
        let album_type = match raw_object["album_type"].as_str() {
            Some("album") => AlbumType::Album,
            Some("single") => AlbumType::Single,
            Some("compilation") => AlbumType::Compilation,
            Some(_) => AlbumType::Album, // default to album
            None => AlbumType::Album,    // default to album
        };

        let total_tracks = match raw_object["total_tracks"].as_i32() {
            Some(total_tracks) => total_tracks,
            None => 0, // default to 0
        };

        let available_markets: Vec<String> = match &raw_object["avaliable_markets"] {
            Array(markets) => markets.iter().map(|market| market.to_string()).collect(), // turn JsonValue Array type to vec of Strings
            _ => vec![], // default to empty vec
        };

        let spotify_url = &raw_object["external_urls"]["spotify"].to_string();

        let href = &raw_object["href"].to_string();

        let id = &raw_object["id"].to_string();

        let images = match &raw_object["images"] {
            Array(images) => images
                .iter()
                .map(|image| SpotifyImage::new(image))
                .collect(), // turn JsonValue Array type to vec of SpotifyImage objects
            _ => vec![], // default to empty vec
        };

        let name = &raw_object["name"].to_string();

        let release_date_precision = match raw_object["release_date_precision"].as_str() {
            Some("year") => ReleaseDatePrecision::Year,
            Some("month") => ReleaseDatePrecision::Month,
            Some("day") => ReleaseDatePrecision::Day,
            Some(_) => ReleaseDatePrecision::None, // default to none
            None => ReleaseDatePrecision::None,    // default to none
        };

        let release_date = {
            // parse string to date based on precision. Panic if unable to do so for whatever reason (hopefully this doens't happen, but it will be a problem for later me if it does)
            match &raw_object["release_date"] {
                Null => None, // default to no date
                date_string => {
                    let date_string_temp = match release_date_precision {
                        ReleaseDatePrecision::Year => {
                            Some(NaiveDate::parse_from_str(&date_string.to_string(), "%Y"))
                        }
                        ReleaseDatePrecision::Month => {
                            Some(NaiveDate::parse_from_str(&date_string.to_string(), "%Y-%m"))
                        }
                        ReleaseDatePrecision::Day => Some(NaiveDate::parse_from_str(
                            &date_string.to_string(),
                            "%Y-%m-%d",
                        )),
                        ReleaseDatePrecision::None => None, // default to no date
                    };
                    match date_string_temp {
                        Some(Ok(date)) => Some(date),
                        Some(Err(_)) => None, // default to no date
                        None => None,         // pass through none
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
            Array(artists) => Some(artists.iter().map(|artist| Artist::new(artist)).collect()), // turn JsonValue Array type to vec of Artist objects
            _ => None, // if artist array can't be found, return None
        };

        let tracks = match &raw_object["tracks"] {
            Null => None,
            _ => Some(SpotifyCollection::<Track>::new(&raw_object["tracks"])),
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
}

impl SpotifyObject for DatedAlbum {
    /// Takes JsonValue object for DatedAlbum and formats it
    ///
    /// # Arguments
    /// * `raw_object` - JsonValue object representing DatedAlbum
    ///
    fn new(raw_object: &JsonValue) -> DatedAlbum {
        let added_at = match &raw_object["added_at"] {
            Null => None, // default to no date
            date_string => Some(
                NaiveDateTime::parse_from_str(&date_string.to_string(), "%Y-%m-%dT%H:%M:%S%.fZ")
                    .unwrap(),
            ),
        };

        let album = Album::new(&raw_object["album"]);

        DatedAlbum {
            date_added: added_at,
            album,
        }
    }
}

impl SpotifyObject for Artist {
    /// Take JsonValue object representing artist from API request and turn into Artist struct for
    /// ease of use.
    ///
    /// # Arguments
    /// * `raw_object` - JsonValue object representing artist from API request
    ///
    fn new(raw_object: &JsonValue) -> Artist {
        let spotify_url = &raw_object["external_urls"]["spotify"].to_string();

        let followers = match raw_object["followers"]["total"].as_i32() {
            Some(followers) => followers,
            None => 0, // default to 0
        };

        let genres: Vec<String> = match &raw_object["genres"] {
            Array(genres) => genres.iter().map(|genre| genre.to_string()).collect(), // turn JsonValue Array type to vec of Strings
            _ => vec![], // default to empty vec
        };

        let href = &raw_object["href"].to_string();

        let id = &raw_object["id"].to_string();

        let images = match &raw_object["images"] {
            Array(images) => images
                .iter()
                .map(|image| SpotifyImage::new(image))
                .collect(), // turn JsonValue Array type to vec of SpotifyImage objects
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
}

impl SpotifyObject for Track {
    /// Format a single track in the form of a JsonValue from API request into struct for ease of use
    ///
    /// # Arguments
    /// * `raw_object` - JsonValue object representing track from API request
    ///
    fn new(raw_object: &JsonValue) -> Track {
        let album = match &raw_object["album"] {
            Null => None, // if album object doesn't exist, return None
            _ => Some(Album::new(&raw_object["album"])), // if album object exists, format it
        };

        let artists: Option<Vec<Artist>> = match &raw_object["artists"] {
            Array(artists) => Some(artists.iter().map(|artist| Artist::new(artist)).collect()), // turn JsonValue Array type to vec of Artist objects
            _ => None, // default to None
        };

        let available_markets: Vec<String> = match &raw_object["available_markets"] {
            Array(markets) => markets.iter().map(|market| market.to_string()).collect(), // turn JsonValue Array type to vec of Strings
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
            Null => ExternalTrackIds {
                isrc: None,
                ean: None,
                upc: None,
            }, // if external_ids object doesn't exist, just set all ids to None
            _ => ExternalTrackIds::new(&raw_object["external_ids"]), // if external_ids object exists, format it
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
}

impl SpotifyObject for DatedTrack {
    /// Takes JsonValue for DatedTrack and formats it
    ///
    /// # Arguments
    /// * `raw_object` - JsonValue object representing DatedTrack from API request
    ///
    fn new(raw_object: &JsonValue) -> DatedTrack {
        let added_at = match &raw_object["added_at"] {
            Null => None, // default to no date
            date_string => Some(
                NaiveDateTime::parse_from_str(&date_string.to_string(), "%Y-%m-%dT%H:%M:%S%.fZ")
                    .unwrap(),
            ),
        };

        let track = Track::new(&raw_object["track"]);

        DatedTrack {
            date_added: added_at,
            track,
        }
    }
}

impl FeatureTrack {
    /// Takes JsonValue representing audio features for a track and formats it into FeatureTrack struct
    ///
    /// # Arguments
    /// * `raw_object` - JsonValue object representing audio features for a track from API request
    ///
    pub fn new(raw_object: &JsonValue) -> FeatureTrack {
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
}

impl AnalysisTrack {
    /// Takes JsonValue representing audion analysis and formats it into AnalysisTrack struct
    ///
    /// # Arguments
    /// * `raw_object` - JsonValue object representing audio analysis for a track from API request
    ///
    pub fn new(raw_object: &JsonValue) -> Result<AnalysisTrack, SpotifyError> {
        // Check for errors before formatting. An error has occured on status code = 1
        if let Some(1) = raw_object["meta"]["status_code"].as_i32() {
            return Err(SpotifyError::FailedRequest(
                raw_object["meta"]["detailed_status"].to_string(),
            ));
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

        let time_signature_confidence =
            match raw_object["track"]["time_signature_confidence"].as_f64() {
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

        for bar in raw_object["bars"].members() {
            // loop through array and format Bar objects
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

        for beat in raw_object["beats"].members() {
            // loop through array and format Beat objects
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

        for section in raw_object["sections"].members() {
            // loop through array and format Section objects
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

        for segment in raw_object["segments"].members() {
            // loop through array and format Segment objects
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

            let pitches: Vec<f64> = segment["pitches"]
                .members()
                .map(|p| p.as_f64().unwrap())
                .collect();

            let timbre: Vec<f64> = segment["timbre"]
                .members()
                .map(|t| t.as_f64().unwrap())
                .collect();

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

        for tatum in raw_object["tatums"].members() {
            // loop through array and format Tatum objects
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
}

impl User {
    /// Takes JsonValue representing a User and returns the User Struct
    ///
    /// # Arguments
    /// * `raw_object` - JsonValue representing a User
    ///
    pub fn new(raw_object: &JsonValue) -> User {
        let country = match raw_object["country"].as_str() {
            Some(country) => Some(country.to_string()),
            None => None, // default to empty string
        };

        let display_name = match raw_object["display_name"].as_str() {
            Some("null") => None, // default to None
            Some(display_name) => Some(display_name.to_string()),
            None => None, // default to None
        };

        let spotify_url = match raw_object["external_urls"]["spotify"].as_str() {
            Some(spotify_url) => spotify_url,
            None => "", // default to empty string
        };

        let total_followers = match raw_object["followers"]["total"].as_i32() {
            Some(followers) => followers,
            None => 0, // default to 0
        };

        let href = match raw_object["href"].as_str() {
            Some(href) => href,
            None => "", // default to empty string
        };

        let id = match raw_object["id"].as_str() {
            Some(id) => id,
            None => "", // default to empty string
        };

        let images = match &raw_object["images"] {
            Array(images) => images
                .iter()
                .map(|image| SpotifyImage::new(image))
                .collect(), // turn JsonValue Array type to vec of SpotifyImage objects
            _ => vec![], // default to empty vec
        };

        let product = match raw_object["product"].as_str() {
            Some(product) => Some(product.to_string()),
            None => None, // default to empty string
        };

        let uri = match raw_object["uri"].as_str() {
            Some(uri) => uri,
            None => "", // default to empty string
        };

        User {
            country,
            display_name,
            spotify_url: spotify_url.to_string(),
            total_followers,
            href: href.to_string(),
            id: id.to_string(),
            images,
            product,
            uri: uri.to_string(),
        }
    }
}

impl SpotifyObject for Playlist {
    /// Takes JsonValue representing a Playlist and returns the Playlist Struct
    ///
    /// # Arguments
    /// * `raw_object` - JsonValue representing a Playlist
    ///
    fn new(raw_object: &JsonValue) -> Playlist {
        let collaborative = match raw_object["collaborative"].as_bool() {
            Some(collaborative) => collaborative,
            None => false, // default to false
        };

        let description = match raw_object["description"].as_str() {
            Some(description) => Some(description.to_string()),
            None => None, // default to None
        };

        let spotify_url = match raw_object["external_urls"]["spotify"].as_str() {
            Some(spotify_url) => spotify_url.to_string(),
            None => String::new(), // default to empty string
        };

        let total_followers = raw_object["followers"]["total"].as_i32().unwrap_or(0); // default to 0

        let href = match raw_object["href"].as_str() {
            Some(href) => String::from(href),
            None => String::new(), // default to empty string
        };

        let id = match raw_object["id"].as_str() {
            Some(id) => String::from(id),
            None => String::new(), // default to empty string
        };

        let images = match &raw_object["images"] {
            Array(images) => images
                .iter()
                .map(|image| SpotifyImage::new(image))
                .collect(), // turn JsonValue Array type to vec of SpotifyImage objects
            _ => vec![], // default to empty vec
        };

        let name = match raw_object["name"].as_str() {
            Some(name) => String::from(name),
            None => String::new(), // default to empty string
        };

        let owner = User::new(&raw_object["owner"]);

        let public = match raw_object["public"].as_bool() {
            Some(public) => Some(public),
            None => None, // default to none
        };

        let snapshot_id = match raw_object["snapshot_id"].as_str() {
            Some(snapshot_id) => String::from(snapshot_id),
            None => String::new(), // default to empty string
        };

        let tracks = match &raw_object["tracks"] {
            Null => None,
            tracks => Some(SpotifyCollection::<PlaylistTrack>::new(tracks)), // format tracks if they exist
        };

        let uri = match raw_object["uri"].as_str() {
            Some(uri) => String::from(uri),
            None => String::new(), // default to empty string
        };

        Playlist {
            collaborative,
            description,
            spotify_url,
            total_followers,
            href,
            id,
            images,
            name,
            owner,
            public,
            snapshot_id,
            tracks,
            uri,
        }
    }
}

impl SpotifyObject for PlaylistTrack {
    /// Takes JsonValue representing a PlaylistTrack and returns the PlaylistTrack Struct
    ///
    /// # Arguments
    /// * `raw_object` - JsonValue representing a PlaylistTrack
    ///
    fn new(raw_object: &JsonValue) -> PlaylistTrack {
        let added_at = match raw_object["added_at"].as_str() {
            Some(added_at) => {
                Some(NaiveDateTime::parse_from_str(added_at, "%Y-%m-%dT%H:%M:%S%.fZ").unwrap())
            } // parse string into NaiveDateTime
            None => None, // default to None
        };

        let added_by = User::new(&raw_object["added_by"]);

        let is_local = match raw_object["is_local"].as_bool() {
            Some(is_local) => is_local,
            None => false, // default to false
        };

        let track = Track::new(&raw_object["track"]);

        PlaylistTrack {
            added_at,
            added_by,
            is_local,
            track,
        }
    }
}

impl<T: SpotifyObject + Debug> SpotifyCollection<T> {
    /// Takes JsonValue representing a collection of spotify objects and returns SpotifyCollection of objects
    ///
    /// # Arguments
    /// * `raw_object` - JsonValue representing a collection of spotify objects
    ///
    pub fn new(raw_object: &JsonValue) -> SpotifyCollection<T> {
        let href = match raw_object["href"].as_str() {
            Some(href) => String::from(href),
            None => String::new(), // default to empty string
        };

        let total = match raw_object["total"].as_i32() {
            Some(total) => total,
            None => 0, // default to 0
        };

        let items = match &raw_object["items"] {
            Array(items) => items.iter().map(|item| T::new(item)).collect(), // turn JsonValue Array type to vec of T objects
            _ => vec![],                                                     // default to empty vec
        };

        let next = match raw_object["next"].as_str() {
            Some(next) => Some(String::from(next)),
            None => None, // default to None
        };

        let previous = match raw_object["previous"].as_str() {
            Some(previous) => Some(String::from(previous)),
            None => None, // default to None
        };

        let limit = match raw_object["limit"].as_i32() {
            Some(limit) => limit,
            None => 0, // default to 0
        };

        let offset = match raw_object["offset"].as_i32() {
            Some(offset) => offset,
            None => 0, // default to 0
        };

        SpotifyCollection {
            href,
            total,
            items,
            next,
            limit,
            offset,
            previous,
        }
    }
}

impl SpotifyObject for Category {
    /// Takes JsonValue representing a Category and returns the Category Struct
    ///
    /// # Arguments
    /// * `raw_object` - JsonValue representing a Category
    ///
    fn new(raw_object: &JsonValue) -> Category {
        let href = match raw_object["href"].as_str() {
            Some(href) => String::from(href),
            None => String::new(), // default to empty string
        };

        let icons = match &raw_object["icons"] {
            Array(icons) => icons.iter().map(|icon| SpotifyImage::new(icon)).collect(), // turn JsonValue Array type to vec of SpotifyImage objects
            _ => vec![], // default to empty vec
        };

        let id = match raw_object["id"].as_str() {
            Some(id) => String::from(id),
            None => String::new(), // default to empty string
        };

        let name = match raw_object["name"].as_str() {
            Some(name) => String::from(name),
            None => String::new(), // default to empty string
        };

        Category {
            href,
            icons,
            id,
            name,
        }
    }
}

impl Device {
    /// Takes JsonValue representing a playback device and returns the Device struct
    ///
    /// # Arguments
    /// * `raw_object` - JsonValue representing a playback device
    ///
    pub fn new(raw_object: &JsonValue) -> Device {
        let id = match raw_object["id"].as_str() {
            Some(id) => String::from(id),
            None => String::new(), // default to empty string
        };

        let is_active = match raw_object["is_active"].as_bool() {
            Some(is_active) => is_active,
            None => false, // default to false
        };

        let is_private_session = match raw_object["is_private_session"].as_bool() {
            Some(is_private_session) => is_private_session,
            None => false, // default to false
        };

        let is_restricted = match raw_object["is_restricted"].as_bool() {
            Some(is_restricted) => is_restricted,
            None => false, // default to false
        };

        let name = match raw_object["name"].as_str() {
            Some(name) => String::from(name),
            None => String::new(), // default to empty string
        };

        let device_type = match raw_object["type"].as_str() {
            Some(r#type) => String::from(r#type),
            None => String::new(), // default to empty string
        };

        let volume_percent = match raw_object["volume_percent"].as_i32() {
            Some(volume_percent) => Some(volume_percent),
            None => None, // default to 0
        };

        Device {
            id,
            is_active,
            is_private_session,
            is_restricted,
            name,
            device_type,
            volume_percent,
        }
    }
}

impl PlaybackActions {
    /// Takes JsonValue representing possible playback actions and returns PlaybackActions struct
    ///
    /// # Arguments
    /// * `raw_object` - JsonValue representing possible playback actions
    ///
    pub fn new(raw_object: &JsonValue) -> PlaybackActions {
        let interrupting_playback = match raw_object["interrupting_playback"].as_bool() {
            Some(interrupting_playback) => interrupting_playback,
            None => false, // default to false
        };

        let pausing = match raw_object["pausing"].as_bool() {
            Some(pausing) => pausing,
            None => false, // default to false
        };

        let resuming = match raw_object["resuming"].as_bool() {
            Some(resuming) => resuming,
            None => false, // default to false
        };

        let seeking = match raw_object["seeking"].as_bool() {
            Some(seeking) => seeking,
            None => false, // default to false
        };

        let skipping_next = match raw_object["skipping_next"].as_bool() {
            Some(skipping_next) => skipping_next,
            None => false, // default to false
        };

        let skipping_prev = match raw_object["skipping_prev"].as_bool() {
            Some(skipping_prev) => skipping_prev,
            None => false, // default to false
        };

        let toggling_repeat_context = match raw_object["toggling_repeat_context"].as_bool() {
            Some(toggling_repeat_context) => toggling_repeat_context,
            None => false, // default to false
        };

        let toggling_shuffle = match raw_object["toggling_shuffle"].as_bool() {
            Some(toggling_shuffle) => toggling_shuffle,
            None => false, // default to false
        };

        let toggling_repeat_track = match raw_object["toggling_repeat_track"].as_bool() {
            Some(toggling_repeat_track) => toggling_repeat_track,
            None => false, // default to false
        };

        let transferring_playback = match raw_object["transferring_playback"].as_bool() {
            Some(transferring_playback) => transferring_playback,
            None => false, // default to false
        };

        PlaybackActions {
            interrupting_playback,
            pausing,
            resuming,
            seeking,
            skipping_next,
            skipping_prev,
            toggling_repeat_context,
            toggling_shuffle,
            toggling_repeat_track,
            transferring_playback,
        }
    }
}

impl Playback {
    /// Takes JsonValue representing playback state and returns Playback struct
    ///
    /// # Arguments
    /// * `raw_object` - JsonValue representing playback state
    ///
    pub fn new(raw_object: &JsonValue) -> Playback {
        let timestamp = match raw_object["timestamp"].as_i64() {
            Some(timestamp) => Some(NaiveDateTime::from_timestamp(timestamp / 1000, 0)), // parse timestamp into NaiveDateTime (timestamp is in ms)
            None => None, // default to None
        };

        let device = match &raw_object["device"] {
            Null => None,                        // default to None
            device => Some(Device::new(device)), // turn JsonValue into Device struct
        };

        let repeat_state = match &raw_object["repeat_state"].as_str() {
            Some("off") => RepeatState::Off,
            Some("track") => RepeatState::Track,
            Some("context") => RepeatState::Context,
            _ => RepeatState::Off, // default to off
        };

        let shuffle_state = match &raw_object["shuffle_state"].as_str() {
            Some("on") => true,
            _ => false, // default to false
        };

        let progress = match raw_object["progress_ms"].as_i32() {
            Some(progress_ms) => Some(progress_ms),
            None => None, // default to 0
        };

        let is_playing = match raw_object["is_playing"].as_bool() {
            Some(is_playing) => is_playing,
            None => false, // default to false
        };

        let track = match &raw_object["item"] {
            Null => None,
            item => Some(Track::new(item)), // format item if it exists
        };

        let actions = match &raw_object["actions"] {
            Null => None,
            actions => Some(PlaybackActions::new(actions)), // format actions if they exist
        };

        Playback {
            device,
            repeat_state,
            shuffle_state,
            timestamp,
            progress,
            is_playing,
            track,
            actions,
        }
    }
}

impl SpotifyObject for PlayedTrack {
    /// Takes JsonValue representing played track and returns PlayedTrack struct
    ///
    /// # Arguments
    /// * `raw_object` - JsonValue representing played track
    ///
    fn new(raw_object: &JsonValue) -> PlayedTrack {
        let track = Track::new(&raw_object["track"]);

        let played_at = match &raw_object["played_at"].as_str() {
            Some(played_at) => {
                Some(NaiveDateTime::parse_from_str(played_at, "%Y-%m-%dT%H:%M:%S%.fZ").unwrap())
            } // parse played_at into NaiveDateTime
            None => None, // default to None
        };

        let context = SpotifyContext::new(&raw_object["context"]);

        PlayedTrack {
            track,
            played_at,
            context,
        }
    }
}
