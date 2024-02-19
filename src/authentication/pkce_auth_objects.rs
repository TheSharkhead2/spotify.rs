use crate::authentication::{
    auth_objects::RefreshAccess,
    pkce_auth::{refresh_token, PkceAccessTokenReponse},
    Scope,
};
use crate::Error;
use chrono::{DateTime, Duration, Utc};

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

impl RefreshAccess for PkceAuth {
    async fn refresh(&self, request_client: &reqwest::Client) -> Result<Self, Error> {
        let response =
            refresh_token(request_client, &self.refresh_token[..], &self.client_id[..]).await?;

        let expires_at = Utc::now() + Duration::seconds(response.expires_in); // get DateTime object for when token will expire

        Ok(PkceAuth::new(
            self.client_id.clone(),
            self.scope,
            response.access_token,
            response.refresh_token,
            expires_at,
        ))
    }

    fn is_expired(&self) -> bool {
        self.expires_at < Utc::now()
    }

    fn is_valid(&self) -> bool {
        !self.is_expired()
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
