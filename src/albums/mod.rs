mod album_endpoints;
mod album_objects;

pub(crate) use album_objects::TempSpotifyAlbum;

pub use album_endpoints::get_album;
pub use album_objects::AlbumId;
