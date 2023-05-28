//! spotifyrs aims to be a relatively easy to use wrapper for the Spotify API. It is still currently under development (see [issues and roadmap](https://github.com/TheSharkhead2/spotify.rs#issues-and-roadmap)); however, it can be used for most purposes in its current state.
//! This wrapper allows you to interact with the Spotify API using simple method calls and consistent objects that give you a consistent way to access the data returned by the API.
//!
//! # Authorization codeflow
//! Note: Currently, the only supported authorization method is the PKCE extension for OAuth2.0, ideal for client-side applications.
//!
//! The first step in the authentication process is generating a url through which your user can grant you authentication through their account. This can be done through the following code
//! ```
//! let scope = String::from("user-modify-playback-state");
//! let redirect_uri = "www.YOUR_URL.com/callback";
//! let client_id = dotenv::var("CLIENT_ID").unwrap();
//!
//! let (auth_url, state, code_verifier) = requesturl_authorization_code(
//!     &client_id[..],
//!     redirect_uri,
//!     &scope[..],
//! );
//! ```
//!
//! You should then have the user redirected to `auth_url` where if they accept the prompt, they will be redirected to the `redirect_uri`. This page will have a query containing a `code` and a `state` value. The `state` should be compared with the `state` returned above, and if they don't match, the authentication should be halted.
//!
//! Take the `code` value in this query and use it as the `auth_url` variable below in order to get the Spotify client:
//! ```
//! let spotify = Spotify::new_from_auth_code(
//!     auth_code,
//!     &client_id[..],
//!     scope,
//!     code_verifier,
//!     redirect_uri,
//! ).await;
//! ```
//!
//! # Automated Local Authentication
//! In order to make use of this, the `local_auth` feature needs to be enabled.
//!
//! First, create a .env file in the root of your project with the `CLIENT_ID` variable:
//! ```dotenv
//! CLIENT_ID=your_client_id
//! ```
//! Then, you can simply create a new `Spotify` object with the `authenticate` method, which will be used to make all requests to the API. You need to pass in the localhost port you want to use for the redirect URI and the scope you need:
//! ```
//! use spotifyrs::Spotify;
//!
//! let spotify = Spotify::new(); // create blank object
//! spotify.authenticate(String::from("8080"), String::from("user-read-private user-read-email")).await.unwrap();
//! ```
//! This will open a browser window and prompt the user to authorize your application. Once they do, they will be redirected to an html page confirming the authorization. You can then use the `Spotify` object to make requests to the API.
//! This can return an error if the user cancels the request. Be warned, this method will also not automatically timeout and will indefinitely hang waiting for the user to authorize if the user closes the browser.
//!
//! The Spotify object will handle refreshing your access token when it expires upon making a request. This ensures that you never have to check if your token is expired.
//!
//! # Examples
//! We can get information on a specific artist:
//! ```
//! let artist: Artist = spotify.get_artist("59sBwR0jPSTrbMtuTkRPN5").await.unwrap();
//!
//! assert_eq!(artist.name, "Wild Rivers");
//! ```
//!
//! Alternatively, we can get the tracks in the current user's queue:
//! ```
//! let (currently_playing, queue) = spotify.get_users_queue().await.unwrap();
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
    ExternalTrackIds, FeatureTrack, Playback, PlaybackActions, PlayedTrack, Playlist,
    PlaylistTrack, ReleaseDatePrecision, RepeatState, RestrictionReason, Section, Segment, Spotify,
    SpotifyCollection, SpotifyContext, SpotifyError, SpotifyImage, SpotifyObject, Tatum, TimeRange,
    Track, User,
}; // re-export relevant structs and enums

pub use authorization::requesturl_authorization_code; // export for authorization code flow

// export if manual authentication feature is active
#[cfg(feature = "local_auth")]
mod local_authentication;
