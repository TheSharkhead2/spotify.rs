use serde::Deserialize;

mod auth_objects;
mod pkce_auth;

/// Struct representing JSON error data returned on authentication/authorization error
#[derive(Deserialize)]
struct AuthorizationJsonError {
    error: String,
    error_description: String,
}
