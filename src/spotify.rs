use chrono::{DateTime, Utc, Duration};
use dotenv;

use crate::authorization::{generate_verifier, get_authorization_code, get_access_token};

pub struct Spotify {
    pub client_id: String,
    pub scope: String,
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
}
