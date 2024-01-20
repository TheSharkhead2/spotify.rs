use crate::Error;

use serde::Deserialize;

/// Authentication error object
#[derive(Deserialize)]
pub(super) struct SpotifyAuthenticationError {
    error: String,
    error_description: String,
}

/// Processing on authentication errors for (potentially) more clarity
pub(super) fn process_auth_error(error: SpotifyAuthenticationError) -> Error {
    // TODO: More processing to give more specific errors

    Error::AuthenticationError(error.error, error.error_description)
}
