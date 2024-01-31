use chrono::{DateTime, Utc};

/// Enum representing all the possible scopes. Used for constructing a `Scope` struct.
pub enum Scopes {
    UGC_IMAGE_UPLOAD,
    USER_READ_PLAYBACK_STATE,
    USER_MODIFY_PLAYBACK_STATE,
    USER_READ_CURRENTLY_PLAYING,
    APP_REMOTE_CONTROL,
    STREAMING,
    PLAYLIST_READ_PRIVATE,
    PLAYLIST_READ_COLLABORATIVE,
    PLAYLIST_MODIFY_PRIVATE,
    PLAYLIST_MODIFY_PUBLIC,
    USER_FOLLOW_MODIFY,
    USER_FOLLOW_READ,
    USER_READ_PLAYBACK_POSITION,
    USER_TOP_READ,
    USER_READ_RECENTLY_PLAYED,
    USER_LIBRARY_MODIFY,
    USER_LIBRARY_READ,
    USER_READ_EMAIL,
    USER_READ_PRIVATE,
    USER_SOA_LINK,
    USER_SOA_UNLINK,
    USER_MANAGE_ENTITLEMENTS,
    USER_MANAGE_PARTNER,
    USER_CREATE_PARTNER,
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
                Scopes::UGC_IMAGE_UPLOAD => {
                    return_scope.ugc_image_upload = true;
                }
                Scopes::USER_READ_PLAYBACK_STATE => {
                    return_scope.user_read_playback_state = true;
                }
                Scopes::USER_MODIFY_PLAYBACK_STATE => {
                    return_scope.user_modify_playback_state = true;
                }
                Scopes::USER_READ_CURRENTLY_PLAYING => {
                    return_scope.user_read_currently_playing = true;
                }
                Scopes::APP_REMOTE_CONTROL => {
                    return_scope.app_remote_control = true;
                }
                Scopes::STREAMING => {
                    return_scope.streaming = true;
                }
                Scopes::PLAYLIST_READ_PRIVATE => {
                    return_scope.playlist_read_private = true;
                }
                Scopes::PLAYLIST_READ_COLLABORATIVE => {
                    return_scope.playlist_read_collaborative = true;
                }
                Scopes::PLAYLIST_MODIFY_PRIVATE => {
                    return_scope.playlist_modify_private = true;
                }
                Scopes::PLAYLIST_MODIFY_PUBLIC => {
                    return_scope.playlist_modify_public = true;
                }
                Scopes::USER_FOLLOW_MODIFY => {
                    return_scope.user_follow_modify = true;
                }
                Scopes::USER_FOLLOW_READ => {
                    return_scope.user_follow_read = true;
                }
                Scopes::USER_READ_PLAYBACK_POSITION => {
                    return_scope.user_read_playback_position = true;
                }
                Scopes::USER_TOP_READ => {
                    return_scope.user_top_read = true;
                }
                Scopes::USER_READ_RECENTLY_PLAYED => {
                    return_scope.user_read_recently_played = true;
                }
                Scopes::USER_LIBRARY_MODIFY => {
                    return_scope.user_library_modify = true;
                }
                Scopes::USER_LIBRARY_READ => {
                    return_scope.user_library_read = true;
                }
                Scopes::USER_READ_EMAIL => {
                    return_scope.user_read_email = true;
                }
                Scopes::USER_READ_PRIVATE => {
                    return_scope.user_read_private = true;
                }
                Scopes::USER_SOA_LINK => {
                    return_scope.user_soa_link = true;
                }
                Scopes::USER_SOA_UNLINK => {
                    return_scope.user_soa_unlink = true;
                }
                Scopes::USER_MANAGE_ENTITLEMENTS => {
                    return_scope.user_manage_entitlements = true;
                }
                Scopes::USER_MANAGE_PARTNER => {
                    return_scope.user_manage_partner = true;
                }
                Scopes::USER_CREATE_PARTNER => {
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

/// Holds authentication data for PKCE authorization
pub struct PkceAuth {
    client_id: String,
    scope: Scope,
    access_token: String,
    refresh_token: String,
    expires_at: DateTime<Utc>,
}

impl PkceAuth {
    pub(super) fn new(
        client_id: String,
        scope: Scope,
        access_token: String,
        refresh_token: String,
        expires_at: DateTime<Utc>,
    ) -> PkceAuth {
        PkceAuth {
            client_id,
            scope,
            access_token,
            refresh_token,
            expires_at,
        }
    }
}

/// Object holding information integral to the PKCE authentication. Used as an intermediate storage between authentication steps.
pub struct PkcePreAuth {
    client_id: String,
    redirect_uri: String,
    scope: Scope,
    state: String,
    code_verifier: String,
}

/// Authentication object for the Spotify API, keeping track of authentication type in enum.
pub enum SpotifyAuth {
    PKCE(PkceAuth),
}

impl PkcePreAuth {
    /// Creates new object
    pub(super) fn new(
        client_id: String,
        redirect_uri: String,
        scope: Scope,
        state: String,
        code_verifier: String,
    ) -> PkcePreAuth {
        PkcePreAuth {
            client_id,
            redirect_uri,
            scope,
            state,
            code_verifier,
        }
    }

    /// Returns the values necessary for getting the access token (in `access_token()`). In particular, will return: `(client_id, redirect_uri, code_verifier)`
    pub(super) fn get_access_token_requirements(&self) -> (String, String, &str) {
        (
            self.client_id.clone(),
            self.redirect_uri.clone(),
            &self.code_verifier[..],
        )
    }

    /// Returns client id and scope for what is required when authenticating with pkce
    pub(super) fn get_auth_requirements(&self) -> (String, Scope) {
        (self.client_id.clone(), self.scope)
    }

    /// Returns the state for comparison
    pub(super) fn get_state(&self) -> String {
        self.state.clone()
    }
}
