use chrono::{DateTime, Duration, NaiveDate, NaiveDateTime, Utc};
use dotenv;
use json::JsonValue;
use std::fmt::{self, Debug};
use std::fs;
use std::sync::RwLock;

use crate::authorization::{
    generate_verifier, get_access_token, get_authorization_code, refresh_access_token,
};

/// Trait to represent single Spotify objects (i.e. Track, Artist, Album, etc.)
pub trait SpotifyObject {
    fn new(raw_object: &JsonValue) -> Self; // must implement new method
}

/// Struct to represent Spotify images (album art, etc.)
pub struct SpotifyImage {
    pub url: String,
    pub height: i32,
    pub width: i32,
}

/// Implements the Debug trait for SpotifyImage
impl Debug for SpotifyImage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SpotifyImage")
            .field("url", &self.url)
            .field("height", &self.height)
            .field("width", &self.width)
            .finish()
    }
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

/// Enum to represent different spotify contexts
pub enum SpotifyContext {
    Album(String),
    Playlist(String),
    Artist(String),
}

impl SpotifyContext {
    /// Function to grab spotify URI from SpotifyContext enum
    pub fn uri(&self) -> String {
        match self {
            SpotifyContext::Album(id) => format!("spotify:album:{}", id),
            SpotifyContext::Playlist(id) => format!("spotify:playlist:{}", id),
            SpotifyContext::Artist(id) => format!("spotify:artist:{}", id),
        }
    }

    /// Function to turn JsonValue general context into SpotifyContext enum
    ///
    /// # Arguments
    /// * `context` - JsonValue representing information on the context returned by Spotify API
    ///
    pub fn new(context: &JsonValue) -> Option<SpotifyContext> {
        let context_type = match context["type"].as_str() {
            Some(t) => t,
            None => return None, // if unable to find type of context, return None
        };

        let context_uri = match context["uri"].as_str() {
            Some(uri) => uri,
            None => return None, // if unable to find uri of context, return None
        };

        let context_id = context_uri.split(':').last().unwrap(); // this technically can panic, but it shouldn't if the uri exists

        match context_type {
            "album" => Some(SpotifyContext::Album(context_id.to_string())),
            "playlist" => Some(SpotifyContext::Playlist(context_id.to_string())),
            "artist" => Some(SpotifyContext::Artist(context_id.to_string())),
            _ => None,
        }
    }
}

/// Implements Debug trait for SpotifyContext
impl Debug for SpotifyContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SpotifyContext::Album(id) => f
                .debug_struct("SpotifyContext")
                .field("type", &"album")
                .field("id", &id)
                .finish(),
            SpotifyContext::Playlist(id) => f
                .debug_struct("SpotifyContext")
                .field("type", &"playlist")
                .field("id", &id)
                .finish(),
            SpotifyContext::Artist(id) => f
                .debug_struct("SpotifyContext")
                .field("type", &"artist")
                .field("id", &id)
                .finish(),
        }
    }
}

/// Struct to hold general collection of Spotify objects
pub struct SpotifyCollection<T: SpotifyObject + Debug> {
    pub href: String,
    pub items: Vec<T>,
    pub limit: i32,
    pub next: Option<String>,
    pub offset: i32,
    pub previous: Option<String>,
    pub total: i32,
}

/// Implements Debug trait for SpotifyCollection
impl<T: SpotifyObject + Debug> fmt::Debug for SpotifyCollection<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SpotifyCollection")
            .field("items", &self.items)
            .field("limit", &self.limit)
            .field("next", &self.next)
            .field("offset", &self.offset)
            .field("previous", &self.previous)
            .field("total", &self.total)
            .finish()
    }
}

/// Struct to represent Spotify category
pub struct Category {
    pub href: String,
    pub icons: Vec<SpotifyImage>,
    pub id: String,
    pub name: String,
}

/// Implements Debug trait for Category
impl fmt::Debug for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Category")
            .field("id", &self.id)
            .field("name", &self.name)
            .finish()
    }
}

/// Struct to represent Album
pub struct Album {
    pub album_type: AlbumType, // Type of album: album, single, compilation
    pub total_tracks: i32,     // The number of tracks in album
    pub available_markets: Vec<String>, // The markets in which the album is available: ISO 3166-1 alpha-2 country codes (Note: considered in market if at least 1 song is in that market)
    pub spotify_url: String,            // The Spotify URL for the album
    pub href: String, // A link to the Web API endpoint providing full details of the album
    pub id: String,   // The Spotify ID for the album
    pub images: Vec<SpotifyImage>, // The cover art for the album in various sizes, widest first
    pub name: String, // The name of the album. In case of an album takedown, the value may be an empty string
    pub release_date: Option<NaiveDate>, // The date the album was first released
    pub release_date_precision: ReleaseDatePrecision, // The precision with which release_date value is known: year, month, or day
    pub restriction_reason: RestrictionReason, // The reason for an album being restricted. Albums may be restricted if the content is not available in a given market, to the user's subscription type, or when the user's account is set to not play explicit content.
    pub uri: String,                           // The Spotify URI for the album
    pub artists: Option<Vec<Artist>>,          // The artists of the album. Can be None
    pub tracks: Option<SpotifyCollection<Track>>, // The tracks of the album. Can be None
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
    pub album: Album,                      // The album
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

/// Struct to represent Artist
pub struct Artist {
    pub spotify_url: String,       // The Spotify URL for the artist
    pub total_followers: i32,      // The total number of followers
    pub genres: Vec<String>, // A list of the genres the artist is associated with. If not yet classified, the array is empty.
    pub href: String,        // A link to the Web API endpoint providing full details of the artist
    pub id: String,          // The Spotify ID for the artist
    pub images: Vec<SpotifyImage>, // Images of the artist in various sizes, widest first
    pub name: String,        // The name of the artist
    pub popularity: i32, // The popularity of the artist. The value will be between 0 and 100, with 100 being the most popular. The artist's popularity is calculated from the popularity of all the artist's tracks
    pub uri: String,     // The Spotify URI for the artist
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
    pub album: Option<Album>,           // The album on which the track appears.
    pub artists: Option<Vec<Artist>>,   // The artists who performed the track.
    pub available_markets: Vec<String>, // A list of the countries in which the track can be played, identified by their ISO 3166-1 alpha-2 code.
    pub disc_number: i32, // The disc number (usually 1 unless the album consists of more than one disc)
    pub duration: i32,    // The track length in milliseconds
    pub explicit: bool, // Whether or not the track has explicit lyrics ( true = yes it does; false = no it does not OR unknown)
    pub external_ids: ExternalTrackIds, // Known external IDs for the track
    pub spotify_url: String, // The Spotify URL for the track
    pub href: String,   // A link to the Web API endpoint providing full details of the track
    pub id: String,     // The Spotify ID for the track
    pub restriction_reason: RestrictionReason, // The reason for the track being restricted. If a track is restricted, the reason is usually market or explicit.
    pub name: String,                          // The name of the track
    pub popularity: i32, // The popularity of the track. The value will be between 0 and 100, with 100 being the most popular.
    pub preview_url: Option<String>, // A URL to a 30 second preview (MP3 format) of the track.
    pub track_number: i32, // The number of the track. If an album has several discs, the track number is the number on the specified disc.
    pub uri: String,       // The Spotify URI for the track
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

/// Struct to represent dated track
pub struct DatedTrack {
    pub track: Track,                      // The track
    pub date_added: Option<NaiveDateTime>, // The date the track was added to the user's library
}

/// Implements Debug trait for DatedTrack struct
impl fmt::Debug for DatedTrack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DatedTrack")
            .field("track", &self.track)
            .field("date_added", &self.date_added)
            .finish()
    }
}

/// Struct to represent a Track's features
pub struct FeatureTrack {
    pub acousticness: f64, // A confidence measure from 0.0 to 1.0 of whether the track is acoustic. 1.0 represents high confidence the track is acoustic.
    pub analysis_url: String, // An HTTP URL to access the full audio analysis of this track.
    pub danceability: f64, // Danceability describes how suitable a track is for dancing based on a combination of musical elements including tempo, rhythm stability, beat strength, and overall regularity. A value of 0.0 is least danceable and 1.0 is most danceable.
    pub duration: i32,     // The duration of the track in milliseconds.
    pub energy: f64, // Energy is a measure from 0.0 to 1.0 and represents a perceptual measure of intensity and activity. Typically, energetic tracks feel fast, loud, and noisy. For example, death metal has high energy, while a Bach prelude scores low on the scale. Perceptual features contributing to this attribute include dynamic range, perceived loudness, timbre, onset rate, and general entropy.
    pub id: String,  // The Spotify ID for the track.
    pub instrumentalness: f64, // Predicts whether a track contains no vocals. “Ooh” and “aah” sounds are treated as instrumental in this context. Rap or spoken word tracks are clearly “vocal”. The closer the instrumentalness value is to 1.0, the greater likelihood the track contains no vocal content. Values above 0.5 are intended to represent instrumental tracks, but confidence is higher as the value approaches 1.0.
    pub key: i32, // The key the track is in. Integers map to pitches using standard Pitch Class notation. E.g. 0 = C, 1 = C♯/D♭, 2 = D, and so on.
    pub liveness: f64, // Detects the presence of an audience in the recording. Higher liveness values represent an increased probability that the track was performed live. A value above 0.8 provides strong likelihood that the track is live.
    pub loudness: f64, // The overall loudness of a track in decibels (dB). Loudness values are averaged across the entire track and are useful for comparing relative loudness of tracks. Loudness is the quality of a sound that is the primary psychological correlate of physical strength (amplitude). Values typical range between -60 and 0 db.
    pub mode: i32, // Mode indicates the modality (major or minor) of a track, the type of scale from which its melodic content is derived. Major is represented by 1 and minor is 0.
    pub speechiness: f64, // The presence of spoken words in the track. Values above 0.66 are probably entirely spoken word, 0.33 to 0.66 may contain both speech and music (rap music), and values below 0.33 most likely represent non-speech or other music
    pub tempo: f64, // The overall estimated tempo of a track in beats per minute (BPM). In musical terminology, tempo is the speed or pace of a given piece and derives directly from the average beat duration.
    pub time_signature: i32, // An estimated overall time signature of a track. The time signature (meter) is a notational convention to specify how many beats are in each bar (or measure).
    pub track_href: String,  // A link to the Web API endpoint providing full details of the track.
    pub uri: String,         // The Spotify URI for the track.
    pub valence: f64, // A measure from 0.0 to 1.0 describing the musical positiveness conveyed by a track. Tracks with high valence sound more positive (e.g. happy, cheerful, euphoric), while tracks with low valence sound more negative (e.g. sad, depressed, angry).
}

/// Implements Debug trait for FeatureTrack struct
impl fmt::Debug for FeatureTrack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FeatureTrack")
            .field("acousticness", &self.acousticness)
            .field("danceability", &self.danceability)
            .field("duration", &self.duration)
            .field("energy", &self.energy)
            .field("id", &self.id)
            .field("instrumentalness", &self.instrumentalness)
            .field("key", &self.key)
            .field("liveness", &self.liveness)
            .field("loudness", &self.loudness)
            .field("mode", &self.mode)
            .field("speechiness", &self.speechiness)
            .field("tempo", &self.tempo)
            .field("time_signature", &self.time_signature)
            .field("valence", &self.valence)
            .finish()
    }
}

/// Struct representing bars in a track for audio analysis
/// [Bar](https://developer.spotify.com/documentation/web-api/reference/#/operations/get-audio-analysis): A bar (or measure) is a segment of time defined as a given number of beats.
pub struct Bar {
    pub start: f64,      // The starting point (in seconds) of the bar
    pub duration: f64,   // The duration (in seconds) of the bar
    pub confidence: f64, // The confidence, from 0.0 to 1.0, of the reliability of the bar
}

/// Implements Debug trait for Bar struct
impl fmt::Debug for Bar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Bar")
            .field("start", &self.start)
            .field("duration", &self.duration)
            .finish()
    }
}

/// Struct representing beats in a track for audio analysis
/// [Beat](https://developer.spotify.com/documentation/web-api/reference/#/operations/get-audio-analysis): A beat is the basic time unit of a piece of music; for example, each tick of a metronome. Beats are typically multiples of tatums.
pub struct Beat {
    pub start: f64,      // The starting point (in seconds) of the beat
    pub duration: f64,   // The duration (in seconds) of the beat
    pub confidence: f64, // The confidence, from 0.0 to 1.0, of the reliability of the beat
}

/// Implements Debug trait for Beat struct
impl fmt::Debug for Beat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Beat")
            .field("start", &self.start)
            .field("duration", &self.duration)
            .finish()
    }
}

/// Struct representing sections in a track for audio analysis
/// [Section](https://developer.spotify.com/documentation/web-api/reference/#/operations/get-audio-analysis): Sections are defined by large variations in rhythm or timbre, e.g. chorus, verse, bridge, guitar solo, etc. Each section contains its own descriptions of tempo, key, mode, time_signature, and loudness.
pub struct Section {
    pub start: f64,                     // The starting point (in seconds) of the section
    pub duration: f64,                  // The duration (in seconds) of the section
    pub confidence: f64, // The confidence, from 0.0 to 1.0, of the reliability of the section
    pub loudness: f64, // The overall loudness of the section in decibels (dB). Loudness values are useful for comparing relative loudness of sections within tracks.
    pub tempo: f64,    // The overall estimated tempo of the section in beats per minute (BPM).
    pub tempo_confidence: f64, // The confidence, from 0.0 to 1.0, of the reliability of the tempo
    pub key: i32, // The estimated overall key of the section. The values in this field ranging from 0 to 11 mapping to pitches using standard Pitch Class notation (E.g. 0 = C, 1 = C♯/D♭, 2 = D, and so on).
    pub key_confidence: f64, // The confidence, from 0.0 to 1.0, of the reliability of the key
    pub mode: i32, // The estimated overall modality (major or minor) of the section. The major key (e.g. C major) is represented by 1 and the minor key (e.g. A minor) is 0.
    pub mode_confidence: f64, // The confidence, from 0.0 to 1.0, of the reliability of the mode
    pub time_signature: i32, // An estimated overall time signature of the section. The time signature (meter) is a notational convention to specify how many beats are in each bar (or measure).
    pub time_signature_confidence: f64, // The confidence, from 0.0 to 1.0, of the reliability of the time_signature
}

/// Implements Debug trait for Section struct
impl fmt::Debug for Section {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Section")
            .field("start", &self.start)
            .field("duration", &self.duration)
            .field("loudness", &self.loudness)
            .field("tempo", &self.tempo)
            .field("key", &self.key)
            .field("mode", &self.mode)
            .field("time_signature", &self.time_signature)
            .finish()
    }
}

/// Struct representing segments in a track for audio analysis
/// [Segment](https://developer.spotify.com/documentation/web-api/reference/#/operations/get-audio-analysis): Each segment contains a roughly consistent sound throughout its duration.
pub struct Segment {
    pub start: f64,             // The starting point (in seconds) of the segment
    pub duration: f64,          // The duration (in seconds) of the segment
    pub confidence: f64, // The confidence, from 0.0 to 1.0, of the reliability of the segment
    pub loudness_start: f64, // The onset loudness of the segment in decibels (dB). Can be used with ```loudness_max``` and ```loudness_max_time``` to describe the "attack" of the track
    pub loudness_max: f64, // The peak loudness of the segment in decibels (dB). Can be used with ```loudness_start``` and ```loudness_max_time``` to describe the "attack" of the track
    pub loudness_max_time: f64, // The segment-relative offset of the segment peak loudness in seconds. Can be used with ```loudness_start``` and ```loudness_max``` to describe the "attack" of the track
    pub loudness_end: f64,
    /// The offset loudness of the segment in decibels (dB). This value should be equal to ```loudness_start``` of the next segment
    pub pitches: Vec<f64>, // A normalized vector of 12 pitch values (values ranging from 0 to 1). These describe the relative dominance of every pitch on the chromatic scale. Noisy sounds are most demonstrated by many values near 1, whereas pure tones will have one value at 1 and the others near 0.
    pub timbre: Vec<f64>, // A 12-vector of unbounded values where each value represents coefficient values for 12 basis functions that can describe the timbre of a segment. Each basis function describes a certain quality of the sound, together they represent "sound color" and can distinguish between instruments.
}

/// Implements Debug trait for Segment struct
impl fmt::Debug for Segment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Segment")
            .field("start", &self.start)
            .field("duration", &self.duration)
            .field("loudness_start", &self.loudness_start)
            .field("loudness_max", &self.loudness_max)
            .field("loudness_max_time", &self.loudness_max_time)
            .field("loudness_end", &self.loudness_end)
            .field("pitches", &self.pitches)
            .field("timbre", &self.timbre)
            .finish()
    }
}

/// Struct representing tatums in a track for audio analysis
/// [Tatum](https://developer.spotify.com/documentation/web-api/reference/#/operations/get-audio-analysis): A tatum represents the lowest regular pulse train that a listener intuitively infers from the timing of percieved musical events (segments).
pub struct Tatum {
    pub start: f64,      // The starting point (in seconds) of the tatum
    pub duration: f64,   // The duration (in seconds) of the tatum
    pub confidence: f64, // The confidence, from 0.0 to 1.0, of the reliability of the tatum
}

/// Implements Debug trait for Tatum struct
impl fmt::Debug for Tatum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Tatum")
            .field("start", &self.start)
            .field("duration", &self.duration)
            .finish()
    }
}

/// Struct to represent a Track's audio analysis
pub struct AnalysisTrack {
    pub analyzer_version: String, // The version of the audio analysis engine
    pub platform: String,         // The platform used to read the track's audio data
    pub detailed_status: String, // A detailed status code for the track, may contain information as to why fields are missing
    pub timestamp: i64,          // The Unix timestamp as to when this track was analyzed
    pub analysis_time: f64,      // The length of time in seconds it took to analyze the track
    pub input_process: String,   // The process used to read the track's audio data

    pub num_samples: i32,               // The number of samples in the track
    pub duration: f64,                  // The length of the track in seconds
    pub analysis_sample_rate: i32,      // The sample rate of the track
    pub analysis_channels: i32,         // The number of channels in the track used for analysis
    pub end_fade_in: f64, // The time, in seconds, the fade-in period of the song begins. If the track has no fade-in, this value is 0.
    pub start_fade_out: f64, // The time, in seconds, the fade-out period of the song begins. If the track has no fade-out, this value is the max song length.
    pub loudness: f64, // The overall loudness of the track in decibels (dB). Averaged values across the entire track, useful for comparing relative loudness of different tracks
    pub tempo: f64,    // average tempo of the track in beats per minute
    pub tempo_confidence: f64, // value from 0 to 1 representing how reliable the tempo value is.
    pub time_signature: i32, // The estimated time signature of the track where values take on 3 to 7 and represent 3/4 and 7/4 time signatures respectively.
    pub time_signature_confidence: f64, // value from 0 to 1 representing how reliable the time signature value is.
    pub key: i32, // The estimated overall key of the track. Integers map to pitches using standard Pitch Class notation. E.g. 0 = C, 1 = C♯/D♭, 2 = D, and so on. Between 1 and 11 and set to -1 if no key is detected
    pub key_confidence: f64, // value from 0 to 1 representing how reliable the key value is.
    pub mode: i32, // The estimated overall modality of the track. 0 = minor, 1 = major.
    pub mode_confidence: f64, // value from 0 to 1 representing how reliable the mode value is.
    pub code_string: String, // An Echo Nest Musical Fingerprint codestring for the track
    pub code_version: String, // The version of the Echo Nest Musical Fingerprint codestring
    pub echoprint_string: String, // An echoprintstring for the track
    pub echoprint_version: String, // The version of echoprintstring
    pub synch_string: String, //  synchstring for the track
    pub synch_version: String, // The version of the =ynchstring
    pub rhythm_string: String, // Rhythmstring for the track
    pub rhythm_version: String, // The version of the rhythmstring

    pub bars: Vec<Bar>,         // A list of bars in the track
    pub beats: Vec<Beat>,       // A list of beats in the track
    pub sections: Vec<Section>, // A list of sections in the track
    pub segments: Vec<Segment>, // A list of segments in the track
    pub tatums: Vec<Tatum>,     // A list of tatums in the track
}

/// Implements the Debug trait for AnalysisTrack struct
impl fmt::Debug for AnalysisTrack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AnalysisTrack")
            .field("duration", &self.duration)
            .field("end_of_fade_in", &self.end_fade_in)
            .field("start_fade_out", &self.start_fade_out)
            .field("loudness", &self.loudness)
            .field("tempo", &self.tempo)
            .field("time_signature", &self.time_signature)
            .field("key", &self.key)
            .field("mode", &self.mode)
            .field("bars", &self.bars)
            .field("beats", &self.beats)
            .field("sections", &self.sections)
            .field("segments", &self.segments)
            .field("tatums", &self.tatums)
            .finish()
    }
}

/// Struct to represent User
pub struct User {
    pub country: Option<String>, // The country of the user, ISO 3166-1 alpha-2 country code.
    pub display_name: Option<String>, // The name displayed on the user's profile.
    pub spotify_url: String,     // Spotify url for the user
    pub total_followers: i32,    // Total number of followers
    pub href: String,            // A link to the Web API endpoint for this user
    pub id: String,              // The Spotify user ID for the user
    pub images: Vec<SpotifyImage>, // The user's profile image
    pub product: Option<String>, // The user's Spotify subscription level: "premium", "free", etc. (The subscription level "open" can be considered the same as "free".)
    pub uri: String,             // The Spotify URI for the user
}

/// Implements Debug trait for User struct
impl fmt::Debug for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("User")
            .field("display_name", &self.display_name)
            .field("id", &self.id)
            .field("total_followers", &self.total_followers)
            .finish()
    }
}

/// Struct to represent a Spotify Playlist
pub struct Playlist {
    pub collaborative: bool, // true if the owner allows other users to modify the playlist
    pub description: Option<String>, // The playlist description. Only returned for modified, verified playlists, otherwise null
    pub spotify_url: String,         // Spotify url for the playlist
    pub total_followers: i32,        // The total number of followers
    pub href: String, // A link to the Web API endpoint providing full details of the playlist
    pub id: String,   // The Spotify ID for the playlist
    pub images: Vec<SpotifyImage>, // The playlist cover image in different sizes
    pub name: String, // The name of the playlist
    pub owner: User,  // The user who owns the playlist
    pub public: Option<bool>, // true if the playlist is public
    pub snapshot_id: String, // The version identifier for the current playlist. Can be supplied in other requests to target a specific playlist version
    pub tracks: Option<SpotifyCollection<PlaylistTrack>>, // The tracks of the playlist
    pub uri: String,         // The Spotify URI for the playlist
}

/// Implements Debug trait for Playlist struct
impl fmt::Debug for Playlist {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Playlist")
            .field("name", &self.name)
            .field("description", &self.description)
            .field("id", &self.id)
            .field("total_followers", &self.total_followers)
            .field("owner", &self.owner)
            .field("tracks", &self.tracks)
            .finish()
    }
}

/// Struct to represent track in playlist
pub struct PlaylistTrack {
    pub added_at: Option<NaiveDateTime>, // The date and time the track was added.
    pub added_by: User,                  // The Spotify user who added the track.
    pub is_local: bool,                  // Whether this track is a local file or not.
    pub track: Track,                    // The track.
}

/// Implements Debug trait for PlaylistTrack struct
impl fmt::Debug for PlaylistTrack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PlaylistTrack")
            .field("added_at", &self.added_at)
            .field("added_by", &self.added_by)
            .field("track", &self.track)
            .finish()
    }
}

/// Struct to represent track played by user (ie in recently played)
pub struct PlayedTrack {
    pub track: Track,                     // The track the user listened to.
    pub played_at: Option<NaiveDateTime>, // The date and time the track was played.
    pub context: Option<SpotifyContext>,  // The context the track was played from.
}

/// Implements Debug trait for PlayedTrack struct
impl fmt::Debug for PlayedTrack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PlayedTrack")
            .field("track", &self.track)
            .field("played_at", &self.played_at)
            .field("context", &self.context)
            .finish()
    }
}

/// Enum to represent possibilities for time range of user top tracks and artists
pub enum TimeRange {
    ShortTerm,
    MediumTerm,
    LongTerm,
}

/// Implements Debug trait for TimeRange enum
impl fmt::Debug for TimeRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TimeRange::ShortTerm => write!(f, "ShortTerm"),
            TimeRange::MediumTerm => write!(f, "MediumTerm"),
            TimeRange::LongTerm => write!(f, "LongTerm"),
        }
    }
}

/// Enum representing repeat state of user playback
pub enum RepeatState {
    Track,   // track is repeating
    Context, // context is repeating
    Off,     // no repeat
}

/// Implements debug trait for RepeatState
impl fmt::Debug for RepeatState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RepeatState::Track => write!(f, "Track"),
            RepeatState::Context => write!(f, "Context"),
            RepeatState::Off => write!(f, "Off"),
        }
    }
}

impl RepeatState {
    /// Converts RepeatState to string
    pub fn to_string(&self) -> String {
        match self {
            RepeatState::Track => String::from("track"),
            RepeatState::Context => String::from("context"),
            RepeatState::Off => String::from("off"),
        }
    }
}

/// Struct to represent a playback device
pub struct Device {
    pub id: String,                  // The device ID.
    pub is_active: bool,             // If this device is the currently active device.
    pub is_private_session: bool,    // If this device is currently in a private session.
    pub is_restricted: bool,         // If playback on this device is currently restricted.
    pub name: String,                // A reasonable human name for the device.
    pub device_type: String, // The type of device. Such as: "computer", "smartphone", "speaker"
    pub volume_percent: Option<i32>, // The current volume in percent. Between 0 and 100.
}

/// Implements Debug trait for Device struct
impl fmt::Debug for Device {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Device")
            .field("id", &self.id)
            .field("is active", &self.is_active)
            .field("name", &self.name)
            .field("device_type", &self.device_type)
            .field("volume_percent", &self.volume_percent)
            .finish()
    }
}

/// Struct representing allowed actions for a given playback state
pub struct PlaybackActions {
    pub interrupting_playback: bool, // If true, the user can go to the next track.
    pub pausing: bool,               // If true, the user can pause the playback.
    pub resuming: bool,              // If true, the user can resume the playback.
    pub seeking: bool, // If true, the user can seek to a specific position in the currently playing track.
    pub skipping_next: bool, // If true, the user can skip to the next track.
    pub skipping_prev: bool, // If true, the user can skip to the previous track.
    pub toggling_repeat_context: bool, // If true, the user can toggle repeat mode for the context.
    pub toggling_repeat_track: bool, // If true, the user can toggle repeat mode for the track.
    pub toggling_shuffle: bool, // If true, the user can toggle shuffle mode.
    pub transferring_playback: bool, // If true, the user can transfer playback to a different device.
}

/// Implements Debug trait for PlaybackActions struct
impl fmt::Debug for PlaybackActions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PlaybackActions")
            .field("interrupting_playback", &self.interrupting_playback)
            .field("pausing", &self.pausing)
            .field("resuming", &self.resuming)
            .field("seeking", &self.seeking)
            .field("skipping_next", &self.skipping_next)
            .field("skipping_prev", &self.skipping_prev)
            .field("toggling_repeat_context", &self.toggling_repeat_context)
            .field("toggling_repeat_track", &self.toggling_repeat_track)
            .field("toggling_shuffle", &self.toggling_shuffle)
            .field("transferring_playback", &self.transferring_playback)
            .finish()
    }
}

/// Struct representing playback state
pub struct Playback {
    pub device: Option<Device>, // Information on the device the user is playing on
    pub repeat_state: RepeatState, // The repeat state of the user's playback.
    pub shuffle_state: bool,    // The shuffle state of the user's playback.
    pub timestamp: Option<NaiveDateTime>, // The timestamp when data was fetched
    pub progress: Option<i32>,  // The progress into the currently playing track.
    pub is_playing: bool,       // If something is currently playing.
    pub track: Option<Track>,   // The track that is currently playing
    pub actions: Option<PlaybackActions>, // The allowed actions for the current playback state
}

/// Implements debug trait for Playback struct
impl fmt::Debug for Playback {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Playback")
            .field("device", &self.device)
            .field("repeat_state", &self.repeat_state)
            .field("shuffle_state", &self.shuffle_state)
            .field("timestamp", &self.timestamp)
            .field("progress", &self.progress)
            .field("is_playing", &self.is_playing)
            .field("track", &self.track)
            .finish()
    }
}

/// Error object for Spotify struct
pub enum SpotifyError {
    RequestError(String),
    InsufficientScope(String),
    FailedRequest(String),
    BadOrExpiredToken(String),
    RateLimitExceeded(String),
    BadRequest(String),
    InvalidRequest(String),
    AuthenticationError(String),
    NotAuthenticated,
    FileError(String),
    NoFile,
    GeneralError(String),
    Unauthorized(String),
    // Unknown,
}

/// Implemntation of formatting such that SpotfiyError can be printed
impl fmt::Debug for SpotifyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SpotifyError::RequestError(e) => write!(f, "Request error: {}", e),
            SpotifyError::InsufficientScope(scopes) => {
                write!(f, "Insufficient scope. Need: {:?}", scopes)
            }
            SpotifyError::FailedRequest(e) => write!(f, "Failed request: {}", e),
            SpotifyError::BadOrExpiredToken(e) => write!(f, "Bad or expired token: {}", e),
            SpotifyError::RateLimitExceeded(e) => write!(f, "Rate limited: {}", e),
            SpotifyError::BadRequest(e) => write!(f, "Bad request: {}", e),
            SpotifyError::InvalidRequest(e) => write!(f, "Invalid request: {}", e),
            SpotifyError::AuthenticationError(e) => write!(f, "Authentication error: {}", e),
            SpotifyError::NotAuthenticated => write!(f, "Not authenticated"),
            SpotifyError::FileError(e) => write!(f, "File error: {}", e),
            SpotifyError::NoFile => write!(f, "No file present"),
            SpotifyError::GeneralError(e) => write!(f, "General error: {}", e),
            SpotifyError::Unauthorized(e) => write!(f, "Unauthorized: {}", e),
            // SpotifyError::Unknown => write!(f, "Unknown error"),
        }
    }
}

/// An authenticated instance of the Spotify API client. Can be used to make requests in the given scope.
pub struct Spotify {
    client_id: RwLock<Option<String>>,
    scope: RwLock<Option<String>>,
    access_token: RwLock<Option<String>>,
    refresh_token: RwLock<Option<String>>,
    expires_at: RwLock<Option<DateTime<Utc>>>,
}

impl Spotify {
    /// Creates a blank Spotify object 
    /// 
    pub fn new() -> Spotify {
        Spotify { 
            client_id: RwLock::new(None), 
            scope: RwLock::new(None), 
            access_token: RwLock::new(None), 
            refresh_token: RwLock::new(None), 
            expires_at: RwLock::new(None) 
        }
    }

    /// Creates a new Spotify object by authenticating with the Spotify API using the PKCE codeflow.
    /// Grabs `client_id` from `.env` file.
    ///
    /// # Arguments
    /// * `localhost_port` - The localhost port fort the redirect uri. Note: currently there is only support for localhost redirect uris.
    /// * `scope` - The scope of the Spotify API. See <https://developer.spotify.com/documentation/general/guides/authorization/scopes/> for more information.
    ///
    pub fn authenticate(&self, localhost_port: String, scope: String) -> Result<(), SpotifyError> {
        let client_id = dotenv::var("CLIENT_ID").unwrap(); // grab client_id from .env

        let (code_verifier, code_challenge) = generate_verifier(); // generate code verifier and code challenge

        let redirect_uri = format!("http://localhost:{}/callback", &localhost_port); // redirect uri for authorization code endpoint

        let auth_code_result = get_authorization_code(
            &client_id,
            &localhost_port,
            &redirect_uri,
            &scope,
            &code_challenge,
        );

        let (access_token, refresh_token, expires_in) = match auth_code_result {
            Ok(auth_code) => {
                get_access_token(&auth_code, &client_id, &code_verifier, &redirect_uri).unwrap()
                // get access token (be lazy with error handling and just panic if request is bad)
            }
            Err(e) => return Err(SpotifyError::AuthenticationError(e.to_string())),
        };

        let expires_at = Utc::now() + Duration::seconds(expires_in); // get time when access token expires

        // update all of the Spotify object's fields
        let mut self_client_id = self.client_id.write().unwrap();
        *self_client_id = Some(client_id);
        let mut self_scope = self.scope.write().unwrap();
        *self_scope = Some(scope);
        let mut self_access_token = self.access_token.write().unwrap();
        *self_access_token = Some(access_token);
        let mut self_refresh_token = self.refresh_token.write().unwrap();
        *self_refresh_token = Some(refresh_token);
        let mut self_expires_at = self.expires_at.write().unwrap();
        *self_expires_at = Some(expires_at);

        Ok(())
    }

    /// Checks to see if required scope is present in current scope
    ///
    /// # Arguments
    /// * `scope` - A string slice that holds required scope
    ///
    pub fn check_scope(&self, scope: &str) -> Result<(), SpotifyError> {
        let current_scope = &*self.scope.read().unwrap(); // get current scope
        let scopes: Vec<&str> = match current_scope {
            Some(scope) => scope.split_whitespace().collect(), // collect scopes into vector
            None => Vec::new(), // if scope isn't set, then assume no scope
        };
        let required_scopes: Vec<&str> = scope.split_whitespace().collect();

        let missing_scopes: Vec<&str> = required_scopes
            .iter()
            .copied()
            .filter(|s| !scopes.contains(s))
            .collect();

        if missing_scopes.len() > 0 {
            return Err(SpotifyError::InsufficientScope(missing_scopes.join(" ")));
        }

        Ok(())
    }

    /// Returns the access token. If the token is expired, it will be refreshed.
    pub fn access_token(&self) -> Result<String, SpotifyError> {
        if self.access_token.read().unwrap().is_none() { // don't proceed if access toekn is not set
            return Err(SpotifyError::NotAuthenticated);
        };

        match *self.expires_at.read().unwrap() {
            Some(expires_at) => {
                // if access token is expired, refresh it
                if Utc::now() > expires_at {
                    let (access_token, expires_at, refresh_token) = self.refresh()?;
                    let mut self_access_token = self.access_token.write().unwrap();
                    *self_access_token = Some(access_token);
                    let mut self_expires_at = self.expires_at.write().unwrap();
                    *self_expires_at = Some(expires_at);
                    let mut self_refresh_token = self.refresh_token.write().unwrap(); 
                    *self_refresh_token = Some(refresh_token);
                }
                return Ok((*self.access_token.read().unwrap()).as_ref().unwrap().to_string()); // return access token. Can assume that the access token is set because already returned error if not
            },
            None => return Err(SpotifyError::NotAuthenticated),
        };
        
    }

    /// Refreshes the access token and returns the new access token and the time it expires
    fn refresh(&self) -> Result<(String, DateTime<Utc>, String), SpotifyError> {
        if self.refresh_token.read().unwrap().is_none() || self.client_id.read().unwrap().is_none() { // if client id or refresh token is not set, return error
            return Err(SpotifyError::NotAuthenticated);
        }
        let (access_token, expires_in, refresh_token) =
            match refresh_access_token(&self.refresh_token.read().unwrap().as_ref().unwrap(), &self.client_id.read().unwrap().as_ref().unwrap()) { // can unwrap because they are set
                Ok((access_token, expires_in, refresh_token)) => (access_token, expires_in, refresh_token),
                Err(e) => panic!("{:?}", e), // on error panic
            };

        let expires_at = Utc::now() + Duration::seconds(expires_in); // get time when access token expires

        // return access token and time when access token expires
        Ok((access_token, expires_at, refresh_token))
    }

    /// Saves necessary authorization information to file for later use 
    /// 
    /// # Arguments 
    /// * `file_name` - The name of the file to save the authorization information to
    /// 
    pub fn save_to_file(&self, file_name: &str) -> Result<(), SpotifyError> {
        if self.client_id.read().unwrap().is_none() || self.scope.read().unwrap().is_none() || self.access_token.read().unwrap().is_none() || self.refresh_token.read().unwrap().is_none() || self.expires_at.read().unwrap().is_none() { // if client_id, scope, access token, refresh token, or expires at is not set, return error
            return Err(SpotifyError::NotAuthenticated);
        }

        let data = format!("{}\n{}\n{}", self.client_id.read().unwrap().as_ref().unwrap(), self.scope.read().unwrap().as_ref().unwrap(), self.refresh_token.read().unwrap().as_ref().unwrap()); // format data to be saved to file

        match fs::write(file_name, data) { // write data to file
            Ok(_) => Ok(()),
            Err(e) => Err(SpotifyError::FileError(e.to_string())),
        }
    }

    /// Creates a new autheticated object from file 
    /// 
    /// # Arguments
    /// * `file_name` - The name of the file to load the authorization information from
    /// 
    /// # Panics
    /// Panics if the file doesn't contain the necessary information
    /// 
    pub fn new_from_file(file_name: &str) -> Result<Spotify, SpotifyError> {
        let data = match fs::read_to_string(file_name) {
            Ok(data) => data,
            Err(_) => return Err(SpotifyError::NoFile), // assume no file 
        };
        let mut lines = data.lines(); // get lines from data

        let client_id = lines.next().unwrap().to_string(); // get client id
        let scope = lines.next().unwrap().to_string(); // get scope
        let refresh_token = lines.next().unwrap().to_string(); // get refresh token

        let (access_token, expires_in, new_refresh_token) = refresh_access_token(&refresh_token, &client_id).unwrap(); // refresh access token. Panics if request is bad
        let expires_at = Utc::now() + Duration::seconds(expires_in); // get time when access token expires

        // return Spotify object
        Ok(Spotify {
            client_id: RwLock::new(Some(client_id)),
            scope: RwLock::new(Some(scope)),
            access_token: RwLock::new(Some(access_token)),
            refresh_token: RwLock::new(Some(new_refresh_token)),
            expires_at: RwLock::new(Some(expires_at)),
        })
    }

    /// Authorizes a blank Spotify object from a file
    /// 
    /// # Arguments
    /// * `file_name` - The name of the file to load the authorization information from
    /// 
    /// # Panics
    /// Panics if the file doesn't contain the necessary information
    ///
    pub fn authenticate_from_file(&self, file_name: &str) -> Result<(), SpotifyError> {
        let data = match fs::read_to_string(file_name) {
            Ok(data) => data,
            Err(_) => return Err(SpotifyError::NoFile), // assume no file 
        };

        let mut lines = data.lines(); // get lines from data

        let client_id = lines.next().unwrap().to_string(); // get client id
        let scope = lines.next().unwrap().to_string(); // get scope
        let refresh_token = lines.next().unwrap().to_string(); // get refresh token

        let (access_token, expires_in, new_refresh_token) = refresh_access_token(&refresh_token, &client_id).unwrap(); // refresh access token. Panics if request is bad
        let expires_at = Utc::now() + Duration::seconds(expires_in); // get time when access token expires
        
        // set client id, scope, access token, refresh token, and expires at
        let mut self_client_id = self.client_id.write().unwrap();
        *self_client_id = Some(client_id);
        let mut self_scope = self.scope.write().unwrap();
        *self_scope = Some(scope);
        let mut self_access_token = self.access_token.write().unwrap();
        *self_access_token = Some(access_token);
        let mut self_refresh_token = self.refresh_token.write().unwrap();
        *self_refresh_token = Some(new_refresh_token);
        let mut self_expires_at = self.expires_at.write().unwrap();
        *self_expires_at = Some(expires_at);

        Ok(())
    }

    /// Returns true if the API is authenticated 
    /// 
    pub fn is_authenticated(&self) -> bool {
        match self.access_token.read() {
            Ok(access_token) => access_token.is_some(),
            Err(_) => false,
        }
    }
}
