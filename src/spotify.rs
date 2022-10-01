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

/// Struct to represent dated track 
pub struct DatedTrack {
    pub track: Track, // The track
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

/// Struct to represent several DatedTracks
pub struct DatedTracks {
    pub href: String, // A link to the Web API endpoint returning the full result of the request
    pub tracks: Vec<DatedTrack>, // The requested data
    pub limit: i32, // The maximum number of items in the response (as set in the query or by default)
    pub next: Option<String>, // URL to next page of items, None if none 
    pub offset: i32, // The offset of the items returned (as set in the query or by default)
    pub previous: Option<String>, // URL to previous page of items, None if none
    pub total: i32, // The total number of items available to return
}

/// Implements Debug trait for DatedTracks struct
impl fmt::Debug for DatedTracks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DatedTracks")
            .field("tracks", &self.tracks)
            .field("limit", &self.limit)
            .field("offset", &self.offset)
            .field("total", &self.total)
            .finish()
    }
}

/// Struct to represent a Track's features
pub struct FeatureTrack {
    pub acousticness: f64, // A confidence measure from 0.0 to 1.0 of whether the track is acoustic. 1.0 represents high confidence the track is acoustic.
    pub analysis_url: String, // An HTTP URL to access the full audio analysis of this track.
    pub danceability: f64, // Danceability describes how suitable a track is for dancing based on a combination of musical elements including tempo, rhythm stability, beat strength, and overall regularity. A value of 0.0 is least danceable and 1.0 is most danceable.
    pub duration: i32, // The duration of the track in milliseconds.
    pub energy: f64, // Energy is a measure from 0.0 to 1.0 and represents a perceptual measure of intensity and activity. Typically, energetic tracks feel fast, loud, and noisy. For example, death metal has high energy, while a Bach prelude scores low on the scale. Perceptual features contributing to this attribute include dynamic range, perceived loudness, timbre, onset rate, and general entropy.
    pub id: String, // The Spotify ID for the track.
    pub instrumentalness: f64, // Predicts whether a track contains no vocals. “Ooh” and “aah” sounds are treated as instrumental in this context. Rap or spoken word tracks are clearly “vocal”. The closer the instrumentalness value is to 1.0, the greater likelihood the track contains no vocal content. Values above 0.5 are intended to represent instrumental tracks, but confidence is higher as the value approaches 1.0.
    pub key: i32, // The key the track is in. Integers map to pitches using standard Pitch Class notation. E.g. 0 = C, 1 = C♯/D♭, 2 = D, and so on.
    pub liveness: f64, // Detects the presence of an audience in the recording. Higher liveness values represent an increased probability that the track was performed live. A value above 0.8 provides strong likelihood that the track is live.
    pub loudness: f64, // The overall loudness of a track in decibels (dB). Loudness values are averaged across the entire track and are useful for comparing relative loudness of tracks. Loudness is the quality of a sound that is the primary psychological correlate of physical strength (amplitude). Values typical range between -60 and 0 db.
    pub mode: i32, // Mode indicates the modality (major or minor) of a track, the type of scale from which its melodic content is derived. Major is represented by 1 and minor is 0.
    pub speechiness: f64, // The presence of spoken words in the track. Values above 0.66 are probably entirely spoken word, 0.33 to 0.66 may contain both speech and music (rap music), and values below 0.33 most likely represent non-speech or other music 
    pub tempo: f64, // The overall estimated tempo of a track in beats per minute (BPM). In musical terminology, tempo is the speed or pace of a given piece and derives directly from the average beat duration.
    pub time_signature: i32, // An estimated overall time signature of a track. The time signature (meter) is a notational convention to specify how many beats are in each bar (or measure).
    pub track_href: String, // A link to the Web API endpoint providing full details of the track.
    pub uri: String, // The Spotify URI for the track.
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
    pub start: f64, // The starting point (in seconds) of the bar
    pub duration: f64, // The duration (in seconds) of the bar
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
    pub start: f64, // The starting point (in seconds) of the beat
    pub duration: f64, // The duration (in seconds) of the beat
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
    pub start: f64, // The starting point (in seconds) of the section
    pub duration: f64, // The duration (in seconds) of the section
    pub confidence: f64, // The confidence, from 0.0 to 1.0, of the reliability of the section
    pub loudness: f64, // The overall loudness of the section in decibels (dB). Loudness values are useful for comparing relative loudness of sections within tracks.
    pub tempo: f64, // The overall estimated tempo of the section in beats per minute (BPM). 
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
    pub start: f64, // The starting point (in seconds) of the segment
    pub duration: f64, // The duration (in seconds) of the segment
    pub confidence: f64, // The confidence, from 0.0 to 1.0, of the reliability of the segment
    pub loudness_start: f64, // The onset loudness of the segment in decibels (dB). Can be used with ```loudness_max``` and ```loudness_max_time``` to describe the "attack" of the track 
    pub loudness_max: f64, // The peak loudness of the segment in decibels (dB). Can be used with ```loudness_start``` and ```loudness_max_time``` to describe the "attack" of the track 
    pub loudness_max_time: f64, // The segment-relative offset of the segment peak loudness in seconds. Can be used with ```loudness_start``` and ```loudness_max``` to describe the "attack" of the track 
    pub loudness_end: f64, /// The offset loudness of the segment in decibels (dB). This value should be equal to ```loudness_start``` of the next segment 
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
    pub start: f64, // The starting point (in seconds) of the tatum
    pub duration: f64, // The duration (in seconds) of the tatum
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
    pub platform: String, // The platform used to read the track's audio data 
    pub detailed_status: String, // A detailed status code for the track, may contain information as to why fields are missing 
    pub timestamp: i64, // The Unix timestamp as to when this track was analyzed 
    pub analysis_time: f64, // The length of time in seconds it took to analyze the track
    pub input_process: String, // The process used to read the track's audio data

    pub num_samples: i32, // The number of samples in the track
    pub duration: f64, // The length of the track in seconds
    pub analysis_sample_rate: i32, // The sample rate of the track
    pub analysis_channels: i32, // The number of channels in the track used for analysis 
    pub end_fade_in: f64, // The time, in seconds, the fade-in period of the song begins. If the track has no fade-in, this value is 0.
    pub start_fade_out: f64, // The time, in seconds, the fade-out period of the song begins. If the track has no fade-out, this value is the max song length.
    pub loudness: f64, // The overall loudness of the track in decibels (dB). Averaged values across the entire track, useful for comparing relative loudness of different tracks 
    pub tempo: f64, // average tempo of the track in beats per minute 
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

    pub bars: Vec<Bar>, // A list of bars in the track
    pub beats: Vec<Beat>, // A list of beats in the track
    pub sections: Vec<Section>, // A list of sections in the track
    pub segments: Vec<Segment>, // A list of segments in the track
    pub tatums: Vec<Tatum>, // A list of tatums in the track
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

/// Error object for Spotify struct
pub enum SpotifyError {
    AccessTokenExpired,
    RequestError(String),
    InsufficientScope(String),
    FailedRequest(String),
    // Unknown,
}

/// Implemntation of formatting such that SpotfiyError can be printed
impl fmt::Display for SpotifyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SpotifyError::AccessTokenExpired => write!(f, "Access token expired, please refresh"),
            SpotifyError::RequestError(e) => write!(f, "Request error: {}", e),
            SpotifyError::InsufficientScope(scopes) => write!(f, "Insufficient scope. Need: {:?}", scopes),
            SpotifyError::FailedRequest(e) => write!(f, "Failed request: {}", e),
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
            SpotifyError::FailedRequest(e) => write!(f, "Failed request: {}", e),
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
