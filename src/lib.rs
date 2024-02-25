mod albums;
mod artists;
pub mod authentication;
mod error;
mod objects;
mod requests;
mod tracks;
mod utils;

// export just Error type
pub use error::Error;

// album exports
pub use albums::get_album;
pub use albums::AlbumId;

// other object exports
pub use objects::Market;
