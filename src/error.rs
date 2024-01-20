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
    InvalidState,                        // State returned with access code doesn't match.

    #[cfg(feature = "local_auth")]
    BrowserFailure(std::io::Error), // failed to open browser

    #[cfg(feature = "local_auth")]
    HttpServerError(std::io::Error), // error when waiting for authentication code

    #[cfg(feature = "local_auth")]
    HttpServerTimeout, // timeout waiting for user to authenticate

    #[cfg(feature = "local_auth")]
    UnexpectedAuthCode, // didn't get both a code and a state upon user authorization
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
            Error::InvalidState => {
                write!(f, "State returned with authentication code is invalid. Please try authenticating again.")
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
            Error::InvalidState => None,

            #[cfg(feature = "local_auth")]
            Error::BrowserFailure(ref e) => Some(e),

            #[cfg(feature = "local_auth")]
            Error::HttpServerError(ref e) => Some(e),

            #[cfg(feature = "local_auth")]
            Error::HttpServerTimeout => None,

            #[cfg(feature = "local_auth")]
            Error::UnexpectedAuthCode => None,
        }
    }
}

/// Implements convertion from `reqwest::Error` to `Error`.
impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        Error::RequestError(err)
    }
}
