mod albums;
pub mod authentication;
mod error;
mod objects;
mod requests;

// export just Error type
pub use error::Error;

// album exports
pub use albums::AlbumId;
