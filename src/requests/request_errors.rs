use reqwest::{Response, StatusCode};

/// The type of status code from the Spotify API.
/// Information about each status code is here: https://developer.spotify.com/documentation/web-api/concepts/api-calls#response-schema
pub(crate) enum SpotifyStatus {
    // not really errors
    OK,          // 200
    Created,     // 201
    Accepted,    // 202
    NoContent,   // 204
    NotModified, // 304

    // really errors
    BadRequest,          // 400
    Unauthorized,        // 401
    Forbidden,           // 403
    NotFound,            // 404
    TooManyRequests,     // 429
    InternalServerError, // 500
    BadGateway,          // 502
    ServiceUnavailable,  // 503
}

/// Implementing type conversion from `reqwest::StatusCode` to `SpotifyResponseError` as the Spotify API only
/// uses a subset of the possible status codes.
impl TryFrom<StatusCode> for SpotifyStatus {
    type Error = crate::Error;
    fn try_from(status_code: StatusCode) -> Result<SpotifyStatus, Self::Error> {
        match status_code {
            // match reqwest status code representation with SpotifyStatus representation
            StatusCode::OK => Ok(SpotifyStatus::OK),
            StatusCode::CREATED => Ok(SpotifyStatus::Created),
            StatusCode::ACCEPTED => Ok(SpotifyStatus::Accepted),
            StatusCode::NO_CONTENT => Ok(SpotifyStatus::NoContent),
            StatusCode::NOT_MODIFIED => Ok(SpotifyStatus::NotModified),
            StatusCode::BAD_REQUEST => Ok(SpotifyStatus::BadRequest),
            StatusCode::UNAUTHORIZED => Ok(SpotifyStatus::Unauthorized),
            StatusCode::FORBIDDEN => Ok(SpotifyStatus::Forbidden),
            StatusCode::NOT_FOUND => Ok(SpotifyStatus::NotFound),
            StatusCode::TOO_MANY_REQUESTS => Ok(SpotifyStatus::TooManyRequests),
            StatusCode::INTERNAL_SERVER_ERROR => Ok(SpotifyStatus::InternalServerError),
            StatusCode::BAD_GATEWAY => Ok(SpotifyStatus::BadGateway),
            StatusCode::SERVICE_UNAVAILABLE => Ok(SpotifyStatus::ServiceUnavailable),

            // not a recongized status code
            status => Err(crate::Error::UnrecognizedStatusCode(status.as_u16())),
        }
    }
}

impl SpotifyStatus {
    /// returns whether this particular status should be considered an error
    fn is_error(&self) -> bool {
        match self {
            &SpotifyStatus::OK => false,
            &SpotifyStatus::Created => false,
            &SpotifyStatus::Accepted => false,
            &SpotifyStatus::NoContent => false,
            &SpotifyStatus::NotModified => false, // could be either way
            _ => true,
        }
    }
}
