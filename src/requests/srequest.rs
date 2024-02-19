use crate::requests::RequestMethod;

/// Makes a general request related to the Spotify API, returning Future that must be awaited.
pub(crate) async fn general_request(
    request_client: &reqwest::Client,
    request_url: String,
    request_method: RequestMethod,
) -> Result<reqwest::Response, reqwest::Error> {
    match request_method {
        RequestMethod::Get(headers) => {
            request_client
                .get(request_url)
                .headers(headers)
                .send()
                .await
        }
        RequestMethod::Post(headers, Some(body)) => {
            request_client
                .post(request_url)
                .headers(headers)
                .json(&body)
                .send()
                .await
        }
        RequestMethod::Post(headers, None) => {
            request_client
                .post(request_url)
                .headers(headers)
                .send()
                .await
        }
        RequestMethod::Put(headers, Some(body)) => {
            request_client
                .put(request_url)
                .headers(headers)
                .json(&body)
                .send()
                .await
        }
        RequestMethod::Put(headers, None) => {
            request_client
                .put(request_url)
                .headers(headers)
                .send()
                .await
        }
        RequestMethod::Delete(headers, Some(body)) => {
            request_client
                .delete(request_url)
                .headers(headers)
                .json(&body)
                .send()
                .await
        }
        RequestMethod::Delete(headers, None) => {
            request_client
                .delete(request_url)
                .headers(headers)
                .send()
                .await
        }
    }
}
