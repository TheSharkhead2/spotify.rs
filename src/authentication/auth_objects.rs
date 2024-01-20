use chrono::{DateTime, Utc};

/// Holds authentication data for PKCE authorization
pub struct PkceAuth {
    client_id: String,
    scope: String,
    access_token: String,
    refresh_token: String,
    expires_at: DateTime<Utc>,
}

impl PkceAuth {
    pub(super) fn new(
        client_id: String,
        scope: String,
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
    scope: String,
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
        scope: String,
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
    pub(super) fn get_auth_requirements(&self) -> (String, String) {
        (self.client_id.clone(), self.scope.clone())
    }

    /// Returns the state for comparison
    pub(super) fn get_state(&self) -> String {
        self.state.clone()
    }
}
