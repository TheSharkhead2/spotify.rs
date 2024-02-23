use crate::authentication::SpotifyAuth;
use crate::objects::BASE_API_URL;
use crate::requests::{general_request, RequestMethod, SpotifyStatus};
use crate::{AlbumId, Error, Market};
use reqwest::header::HeaderMap;

async fn get_album(
    request_client: &reqwest::Client,
    spotify: SpotifyAuth,
    album_id: impl Into<AlbumId>,
    market: Option<impl Into<Market>>,
) -> Result<(), Error> {
    // convert to AlbumId to catch some errors here
    let albumid: AlbumId = album_id.try_into()?;

    // create base url with album id
    let mut request_url = format!("{}albums/{}", BASE_API_URL, albumid.id());

    if let Some(market) = market {
        // convert to market object to avoid issues
        let market_obj: Market = market.try_into()?;

        request_url.push_str(&format!("?market={}", market_obj.code()))
    }

    // make request
    let response = general_request(
        request_client,
        request_url,
        RequestMethod::Get(HeaderMap::new()),
    )
    .await?;

    // get status code
    let status_code: SpotifyStatus = response.status().try_into()?;

    Ok(())
}
