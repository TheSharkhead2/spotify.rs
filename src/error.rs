use std::error;
use std::fmt;

// based error type on: https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/wrap_error.html

/// Relevant errors to interacting with the Spotify API
#[derive(Debug)]
pub enum Error {
    RequestError(reqwest::Error),      // general request error
    UnrecognizedStatusCode(u16),       // spotify returned an  unrecognized status code
    UnexpectedSuccessfulResponse(u16), // response returned a success, but with an unexpected status code
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::RequestError(..) => {
                write!(f, "encountered an error with an https request to the api")
            }
            Error::UnrecognizedStatusCode(c) => {
                write!(
                    f,
                    "{}",
                    format!("Spotify returned unrecognized status code {}", c)
                )
            }
            Error::UnexpectedSuccessfulResponse(c) => {
                write!(
                    f,
                    "{}",
                    format!(
                        "Spotify request was successful, but with unexpected status code {}",
                        c
                    )
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
            Error::UnexpectedSuccessfulResponse(..) => None,
        }
    }
}

/// Implements convertion from `reqwest::Error` to `Error`.
impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        Error::RequestError(err)
    }
}
