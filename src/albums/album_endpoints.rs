use reqwest::header::HeaderMap;

use crate::albums::TempSpotifyAlbum;
use crate::authentication::SpotifyAuth;
use crate::objects::BASE_API_URL;
use crate::requests::{general_request, RequestMethod, SpotifyStatus};
use crate::utils::{process_endpoint_status_code_errors, EndpointRequestError};
use crate::{AlbumId, Error, Market};

pub async fn get_album<T, S>(
    request_client: &reqwest::Client,
    spotify: SpotifyAuth,
    album_id: T,
    market: Option<S>,
) -> Result<(), Error>
where
    T: TryInto<AlbumId>,
    T::Error: Into<Error>,
    S: TryInto<Market>,
    S::Error: Into<Error>, // error conversion woes solved by: https://users.rust-lang.org/t/impl-tryinto-as-an-argument-in-a-function-complains-about-the-error-conversion/34004
{
    // convert to AlbumId to catch some errors here
    let albumid: AlbumId = album_id.try_into().map_err(Into::into)?;

    // create base url with album id
    let mut request_url = format!("{}albums/{}", BASE_API_URL, albumid.id());

    if let Some(market) = market {
        // convert to market object to avoid issues
        let market_obj: Market = market.try_into().map_err(Into::into)?;

        request_url.push_str(&format!("?market={}", market_obj.code()))
    }

    // get headers
    let headers = spotify.get_base_auth_header_map();

    // make request
    let response =
        general_request(request_client, request_url, RequestMethod::Get(headers)).await?;

    // get status code
    let status_code: SpotifyStatus = response.status().try_into()?;

    match status_code {
        SpotifyStatus::OK => {
            // all OK, process response

            let album: TempSpotifyAlbum = response.json().await?;
        }
        status_code => {
            // parse error from Spotify
            let request_error: EndpointRequestError = response.json().await?;

            // return error
            return Err(process_endpoint_status_code_errors(
                status_code,
                request_error,
            ));
        }
    };

    Ok(())
}
