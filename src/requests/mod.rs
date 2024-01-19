mod request_errors;
mod srequest;

use reqwest::header::HeaderMap;
use serde_json::Value;

/// Enum to store the type of request method and the headers + body to that request
#[derive(Clone)]
pub(crate) enum RequestMethod {
    Get(HeaderMap),
    Post(HeaderMap, Option<Value>),
    Put(HeaderMap, Option<Value>),
    Delete(HeaderMap, Option<Value>),
}

// export `general_request`
pub(crate) use request_errors::SpotifyStatus;
pub(crate) use srequest::general_request;
