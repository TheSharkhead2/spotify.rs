use std::convert::Infallible;
use std::error;
use std::fmt;

// based error type on: https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/wrap_error.html

/// Relevant errors to interacting with the Spotify API
#[derive(Debug)]
pub enum Error {
    RequestError(reqwest::Error),        // general request error
    UnrecognizedStatusCode(u16),         // spotify returned an  unrecognized status code
    UnexpectedStatusCode(u16),           // response returned status code that doesn't make sense
    AuthenticationError(String, String), // Arbitrary authentication error
    ReauthenticateUser(String),          // User is no longer authenticated, reauthenticate
    BadOAuthRequest(String),             // Bad OAuth request
    ExceededRateLimits(String),          // App has exceeded rate limits
    InvalidState,                        // State returned with access code doesn't match.
    MalformedUri(String),                // URI provided is invalid
    InvalidUriType(String, String),      // provided URI that should be of a certain type (first string) but is actually of the second type
    InvalidUrl(url::ParseError),         // the Spotify URL passed wasn't recognized as a URL
    MalformedUrl(String),                // The Spotify URL provided was invalid
    InvalidUrlType(String, String),      // The Spotify URL supplied is for the wrong type of object
    InvalidId(String),                   // The provided Spotify ID cannot be interpreted
    InvalidMarket(String),               // The supplied ISO 3166-1 alpha-2 market code

    #[cfg(feature = "local_auth")]
    BrowserFailure(std::io::Error), // failed to open browser

    #[cfg(feature = "local_auth")]
    HttpServerError(std::io::Error), // error when waiting for authentication code

    #[cfg(feature = "local_auth")]
    HttpServerTimeout, // timeout waiting for user to authenticate

    #[cfg(feature = "local_auth")]
    UnexpectedAuthCode, // didn't get both a code and a state upon user authorization

    Infallible, // Shouldn't exist
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::RequestError(..) => {
                write!(f, "Encountered an error with an https request to the api")
            }
            Error::UnrecognizedStatusCode(c) => {
                write!(
                    f,
                    "{}",
                    format!("Spotify returned unrecognized status code {}", c)
                )
            }
            Error::UnexpectedStatusCode(c) => {
                write!(
                    f,
                    "{}",
                    format!("Spotify request returned with unexpected status code {}", c)
                )
            }
            Error::AuthenticationError(error, error_description) => {
                write!(
                    f,
                    "{}",
                    format!("Encountered an authentication error with Spotify API: {}. Specifically: {}", error, error_description)
                )
            }

            Error::ReauthenticateUser(error) => {
                write!(
                    f, 
                    "{}",
                    format!("The user is no longer authenticated, reauthenticate. Specifically: {}", error)
                )
            }

            Error::BadOAuthRequest(error) => {
                write!(
                    f,
                    "{}",
                    format!("Bad OAuth Request: {}", error)
                )
            }

            Error::ExceededRateLimits(error) => {
                write!(
                    f,
                    "{}",
                    format!("App has exceeded rate limits: {}", error)
                )
            }
            
            Error::InvalidState => {
                write!(f, "State returned with authentication code is invalid. Please try authenticating again.")
            }

            Error::MalformedUri(uri) => {
                write!(
                    f,
                    "{}",
                    format!("The Spotify URI provided is invalid: {}", uri)
                )
            }

            Error::InvalidUriType(expected, got) => {
                write!(
                    f,
                    "{}",
                    format!("The Spotify URI provided was expected to be of type '{}' but got one of type '{}'", expected, got)
                )
            }

            Error::InvalidUrl(..) => {
                write!(f, "Unable to parse the given Spotify URL")
            }

            Error::MalformedUrl(url) => {
                write!(
                    f,
                    "{}",
                    format!("The Spotfiy URL provided is invalid: {}", url)
                )
            }

            Error::InvalidUrlType(expected, got) => {
                write!(
                    f, 
                    "{}",
                    format!("The Spotify URL provided was expected to be of type '{}' but got one of type '{}'", expected, got)
                )
            }

            Error::InvalidId(id) => {
                write!(
                    f, 
                    "{}",
                    format!("The Spotify ID provided could bot be interpreted: '{}'", id)
                )
            }

            Error::InvalidMarket(market) => {
                write!(
                    f, 
                    "{}",
                    format!("The provided market code '{}' is not a valid ISO 3166-1 alpha-2 country code", market)
                )
            }

            #[cfg(feature = "local_auth")]
            Error::BrowserFailure(..) => {
                write!(f, "Encountered error opening the browser.")
            }

            #[cfg(feature = "local_auth")]
            Error::HttpServerError(..) => {
                write!(f, "Failed to listen for authentication code.")
            }

            #[cfg(feature = "local_auth")]
            Error::HttpServerTimeout => {
                write!(f, "Timeout reached. User didn't authenticate")
            }

            #[cfg(feature = "local_auth")]
            Error::UnexpectedAuthCode => {
                write!(f, "Didn't get a code and a state when user authenticated with the API. Could another request have been sent to the port?")
            }

            Error::Infallible => {
                write!(
                    f, 
                    "{}",
                    "An impossible error has occured"
                )
            }
        }

    }
}

/// Implements source tracing for `Error`
impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Error::RequestError(ref e) => Some(e),
            Error::UnrecognizedStatusCode(..) => None,
            Error::UnexpectedStatusCode(..) => None,
            Error::AuthenticationError(..) => None,
            Error::ReauthenticateUser(..) => None,
            Error::BadOAuthRequest(..) => None,
            Error::ExceededRateLimits(..) => None,
            Error::InvalidState => None,
            Error::MalformedUri(..) => None,
            Error::InvalidUriType(..) => None,
            Error::InvalidUrl(ref e) => Some(e),
            Error::MalformedUrl(..) => None,
            Error::InvalidUrlType(..) => None,
            Error::InvalidId(..) => None,
            Error::InvalidMarket(..) => None,

            #[cfg(feature = "local_auth")]
            Error::BrowserFailure(ref e) => Some(e),

            #[cfg(feature = "local_auth")]
            Error::HttpServerError(ref e) => Some(e),

            #[cfg(feature = "local_auth")]
            Error::HttpServerTimeout => None,

            #[cfg(feature = "local_auth")]
            Error::UnexpectedAuthCode => None,

            Error::Infallible => None,
        }
    }
}

/// Implements convertion from `reqwest::Error` to `Error`.
impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        Error::RequestError(err)
    }
}

/// Implements convertion from `url::ParseError` to `Error`.
impl From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Error {
        Error::InvalidUrl(err)
    }
}

/// Shouldn't ever actually happen, but for types to work
impl From<Infallible> for Error {
    fn from(_: Infallible) -> Self {
        Error::Infallible
    }
}
