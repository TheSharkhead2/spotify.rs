use dotenv;
use http;

fn generate_verifier() {
    
}

/// Object that holds information relevant to PKCE authorization
pub struct ApplicationDetails {
    pub client_id: String,
    redirect_uri: String,
    scope: String,
    access_token: Option<String>,
    refresh_token: Option<String>,
    expires_at: Option<u32>,
}

impl ApplicationDetails {
    pub fn new(redirect_uri: String, scope: String) -> ApplicationDetails {
        let client_id = dotenv::var("CLIENT_ID").unwrap(); // grab client_id from .env

        ApplicationDetails {
            client_id: client_id,
            redirect_uri: redirect_uri,
            scope: scope,
            access_token: None,
            refresh_token: None,
            expires_at: None,
        }
    }
}
