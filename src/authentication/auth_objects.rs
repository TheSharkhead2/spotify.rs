use chrono::{DateTime, Utc};

/// Holds authentication data for PKCE authorization
pub struct PkceAuth {
    client_id: &'static str,
    scope: String,
    access_token: &'static str,
    refresh_token: &'static str,
    expires_at: DateTime<Utc>,
}

/// Object holding information integral to the PKCE authentication. Used as an intermediate storage between authentication steps.
pub struct PkcePreAuth {
    client_id: &'static str,
    redirect_uri: &'static str,
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
        client_id: &'static str,
        redirect_uri: &'static str,
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
    pub(super) fn get_access_token_requirements(&self) -> (&'static str, &'static str, &str) {
        (self.client_id, self.redirect_uri, &self.code_verifier[..])
    }
}
