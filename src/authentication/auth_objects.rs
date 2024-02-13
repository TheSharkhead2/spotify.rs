use crate::authentication::PkceAuth;
use crate::Error;

/// Enum representing all the possible scopes. Used for constructing a `Scope` struct.
pub enum Scopes {
    UgcImageUpload,
    UserReadPlaybackState,
    UserModifyPlaybackState,
    UserReadCurrentlyPlaying,
    AppRemoteControl,
    Streaming,
    PlaylistReadPrivate,
    PlaylistReadCollaborative,
    PlaylistModifyPrivate,
    PlaylistModifyPublic,
    UserFollowModify,
    UserFollowRead,
    UserReadPlaybackPosition,
    UserTopRead,
    UserReadRecentlyPlayed,
    UserLibraryModify,
    UserLibraryRead,
    UserReadEmail,
    UserReadPrivate,
    UserSoaLink,
    UserSoaUnlink,
    UserManageEntitlements,
    UserManagePartner,
    UserCreatePartner,
}

/// Object for holding the scope of authentication
/// See offical documentation: https://developer.spotify.com/documentation/web-api/concepts/scopes#ugc-image-upload
#[derive(Clone, Copy)]
pub struct Scope {
    ugc_image_upload: bool,
    user_read_playback_state: bool,
    user_modify_playback_state: bool,
    user_read_currently_playing: bool,
    app_remote_control: bool, // I believe only required for android/ios enpoints
    streaming: bool,
    playlist_read_private: bool,
    playlist_read_collaborative: bool,
    playlist_modify_private: bool,
    playlist_modify_public: bool,
    user_follow_modify: bool,
    user_follow_read: bool,
    user_read_playback_position: bool,
    user_top_read: bool,
    user_read_recently_played: bool,
    user_library_modify: bool,
    user_library_read: bool,
    user_read_email: bool,
    user_read_private: bool,
    user_soa_link: bool,
    user_soa_unlink: bool,
    user_manage_entitlements: bool,
    user_manage_partner: bool,
    user_create_partner: bool,
}

impl Scope {
    /// Create a Scope object specifying no scopes
    fn none() -> Scope {
        Scope {
            ugc_image_upload: false,
            user_read_playback_state: false,
            user_modify_playback_state: false,
            user_read_currently_playing: false,
            app_remote_control: false,
            streaming: false,
            playlist_read_private: false,
            playlist_read_collaborative: false,
            playlist_modify_private: false,
            playlist_modify_public: false,
            user_follow_modify: false,
            user_follow_read: false,
            user_read_playback_position: false,
            user_top_read: false,
            user_read_recently_played: false,
            user_library_modify: false,
            user_library_read: false,
            user_read_email: false,
            user_read_private: false,
            user_soa_link: false,
            user_soa_unlink: false,
            user_manage_entitlements: false,
            user_manage_partner: false,
            user_create_partner: false,
        }
    }

    /// Create a new Scope object
    pub fn new(scopes: Vec<Scopes>) -> Scope {
        // start with no scopes
        let mut return_scope = Scope::none();

        // add each one requested
        for scope in scopes {
            match scope {
                Scopes::UgcImageUpload => {
                    return_scope.ugc_image_upload = true;
                }
                Scopes::UserReadPlaybackState => {
                    return_scope.user_read_playback_state = true;
                }
                Scopes::UserModifyPlaybackState => {
                    return_scope.user_modify_playback_state = true;
                }
                Scopes::UserReadCurrentlyPlaying => {
                    return_scope.user_read_currently_playing = true;
                }
                Scopes::AppRemoteControl => {
                    return_scope.app_remote_control = true;
                }
                Scopes::Streaming => {
                    return_scope.streaming = true;
                }
                Scopes::PlaylistReadPrivate => {
                    return_scope.playlist_read_private = true;
                }
                Scopes::PlaylistReadCollaborative => {
                    return_scope.playlist_read_collaborative = true;
                }
                Scopes::PlaylistModifyPrivate => {
                    return_scope.playlist_modify_private = true;
                }
                Scopes::PlaylistModifyPublic => {
                    return_scope.playlist_modify_public = true;
                }
                Scopes::UserFollowModify => {
                    return_scope.user_follow_modify = true;
                }
                Scopes::UserFollowRead => {
                    return_scope.user_follow_read = true;
                }
                Scopes::UserReadPlaybackPosition => {
                    return_scope.user_read_playback_position = true;
                }
                Scopes::UserTopRead => {
                    return_scope.user_top_read = true;
                }
                Scopes::UserReadRecentlyPlayed => {
                    return_scope.user_read_recently_played = true;
                }
                Scopes::UserLibraryModify => {
                    return_scope.user_library_modify = true;
                }
                Scopes::UserLibraryRead => {
                    return_scope.user_library_read = true;
                }
                Scopes::UserReadEmail => {
                    return_scope.user_read_email = true;
                }
                Scopes::UserReadPrivate => {
                    return_scope.user_read_private = true;
                }
                Scopes::UserSoaLink => {
                    return_scope.user_soa_link = true;
                }
                Scopes::UserSoaUnlink => {
                    return_scope.user_soa_unlink = true;
                }
                Scopes::UserManageEntitlements => {
                    return_scope.user_manage_entitlements = true;
                }
                Scopes::UserManagePartner => {
                    return_scope.user_manage_partner = true;
                }
                Scopes::UserCreatePartner => {
                    return_scope.user_create_partner = true;
                }
            }
        }
        return_scope
    }
}

impl Into<String> for Scope {
    fn into(self) -> String {
        // Spotify expects a string of scopes separated by spaces
        let mut scopes: Vec<&str> = Vec::new();

        // rather long and repetative way of getting all our scope strings
        if self.ugc_image_upload {
            scopes.push("ugc-image-upload");
        }
        if self.user_read_playback_state {
            scopes.push("user-read-playback-state");
        }
        if self.user_modify_playback_state {
            scopes.push("user-modify-playback-state");
        }
        if self.user_read_currently_playing {
            scopes.push("user-read-currently-playing");
        }
        if self.app_remote_control {
            scopes.push("app-remote-control");
        }
        if self.streaming {
            scopes.push("streaming");
        }
        if self.playlist_read_private {
            scopes.push("playlist-read-private");
        }
        if self.playlist_read_collaborative {
            scopes.push("playlist-read-collaborative");
        }
        if self.playlist_modify_private {
            scopes.push("playlist-modify-private");
        }
        if self.playlist_modify_public {
            scopes.push("playlist-modify-public");
        }
        if self.user_follow_modify {
            scopes.push("user-follow-modify");
        }
        if self.user_follow_read {
            scopes.push("user-follow-read");
        }
        if self.user_read_playback_position {
            scopes.push("user-read-playback-position");
        }
        if self.user_top_read {
            scopes.push("user-top-read");
        }
        if self.user_read_recently_played {
            scopes.push("user-read-recently-played");
        }
        if self.user_library_modify {
            scopes.push("user-library-modify");
        }
        if self.user_library_read {
            scopes.push("user-library-read");
        }
        if self.user_read_email {
            scopes.push("user-read-email");
        }
        if self.user_read_private {
            scopes.push("user-read-private");
        }
        if self.user_soa_link {
            scopes.push("user-soa-link");
        }
        if self.user_soa_unlink {
            scopes.push("user-soa-unlink");
        }
        if self.user_manage_entitlements {
            scopes.push("user-manage-entitlements");
        }
        if self.user_manage_partner {
            scopes.push("user-manage-partner");
        }
        if self.user_create_partner {
            scopes.push("user-create-partner");
        }

        // join all scopes with space
        scopes.join(" ")
    }
}

/// Trait describing how authenticated objects refresh.
pub trait RefreshAccess
where
    Self: Sized,
{
    /// Returns new authenticated object. For use when it has expired. Doesn't do anything if access is still valid.
    async fn refresh(&self, request_client: reqwest::Client) -> Result<Self, Error>;

    /// Returns true if access to the API has expired.
    fn is_expired(&self) -> bool;

    /// Returns true if access to the API is still valid.
    fn is_valid(&self) -> bool;
}

/// Authentication object for the Spotify API, keeping track of authentication type in enum.
pub enum SpotifyAuth {
    PKCE(PkceAuth),
}
