use crate::authentication::SpotifyAuth;
use crate::AlbumId;

async fn get_album(
    request_client: &reqwest::Client,
    spotify: SpotifyAuth,
    album_id: impl Into<AlbumId>,
) {
}
