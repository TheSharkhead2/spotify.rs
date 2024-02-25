use serde::Deserialize;

use crate::{requests::SpotifyStatus, Error};

/// Struct representing error when making a request to an endpoint
#[derive(Deserialize)]
pub(crate) struct EndpointRequestError {
    pub status: i64,
    pub message: String,
}

/// Processes a (bad) status code from a request given to an endpoint into an error
pub(crate) fn process_endpoint_status_code_errors(
    status_code: SpotifyStatus,
    error: EndpointRequestError,
) -> Error {
    // process status code to error
    match status_code {
        SpotifyStatus::Unauthorized => Error::ReauthenticateUser(error.message),
        SpotifyStatus::Forbidden => Error::BadOAuthRequest(error.message),
        SpotifyStatus::TooManyRequests => Error::ExceededRateLimits(error.message),
        _ => Error::UnrecognizedStatusCode(status_code.into()),
    }
}
