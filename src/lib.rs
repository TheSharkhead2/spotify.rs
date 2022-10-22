//! spotifyrs aims to be a relatively easy to use wrapper for the Spotify API. It is still currently under development (see [issues and roadmap](https://github.com/TheSharkhead2/spotify.rs#issues-and-roadmap)); however, it can be used for most purposes in its current state.
//! This wrapper allows you to interact with the Spotify API using simple method calls and consistent objects that give you a consistent way to access the data returned by the API.
//!
//! # Authorization codeflow
//! Note: Currently, the only supported authorization method is the PKCE extension for OAuth2.0, ideal for client-side applications.
//! Because of this, we also currently only support localhost redirect URIs.
//!
//! First, create a .env file in the root of your project with the `CLIENT_ID` variable:
//! ```dotenv
//! CLIENT_ID=your_client_id
//! ```
//! Then, you can simply create a new `Spotify` object with the `authenticate` method, which will be used to make all requests to the API. You need to pass in the localhost port you want to use for the redirect URI and the scope you need:
//! ```ignore
//! use spotifyrs::Spotify;
//!
//! let mut spotify = Spotify::new(); // create blank object
//! spotify.authenticate(String::from("8080"), String::from("user-read-private user-read-email")).unwrap(); // authenticate it
//! ```
//! This will open a browser window and prompt the user to authorize your application. Once they do, they will be redirected to an html page confirming the authorization. You can then use the `Spotify` object to make requests to the API.
//! This can return an error if the user cancels the request. Be warned, this method will also not automatically timeout and will indefinitely hang waiting for the user to authorize if the user closes the browser.
//!
//! The Spotify object will handle refreshing your access token when it expires upon making a request. This ensures that you never have to check if your token is expired.
//!
//! # Examples
//! We can get information on a specific artist:
//! ```ignore
//! let artist: Artist = spotify.get_artist("59sBwR0jPSTrbMtuTkRPN5").unwrap();
//!
//! assert_eq!(artist.name, "Wild Rivers");
//! ```
//!
//! Alternatively, we can get the tracks in the current user's queue:
//! ```ignore
//! let (currently_playing, queue) = spotify.get_users_queue().unwrap();
//! ```
//!
//! See the [Spotify struct](struct.Spotify.html) for a full list of supported endpoints.
//!

mod albums;
mod artists;
mod authorization;
mod categories;
mod genres;
mod markets;
mod object_formatting;
mod player;
mod playlist;
mod spotify;
mod srequest;
mod tracks;
mod users;

pub use spotify::{
    Album, AlbumType, AnalysisTrack, Artist, Bar, Beat, Category, DatedAlbum, DatedTrack, Device,
    ExternalTrackIds, FeatureTrack, Playback, PlaybackActions, Playlist, PlaylistTrack,
    ReleaseDatePrecision, RepeatState, RestrictionReason, Section, Segment, Spotify,
    SpotifyCollection, SpotifyContext, SpotifyError, SpotifyImage, SpotifyObject, Tatum, TimeRange,
    Track, User,
}; // re-export relevant structs and enums
