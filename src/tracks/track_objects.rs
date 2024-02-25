use serde::Deserialize;

use crate::artists::TempSimplifiedArtistObject;
use crate::objects::{TempExternalUrls, TempRestriction};

/// Object representing a collection of tracks to Deseralize into
#[derive(Deserialize, Debug)]
pub(crate) struct TempTracks {
    href: String,
    limit: i32,
    next: Option<String>,
    offset: i32,
    previous: Option<String>,
    total: i32,
}

/// Object representing track linked from to Deserialize into
#[derive(Deserialize, Debug)]
pub(crate) struct TempTrackLinkedFrom {
    external_urls: TempExternalUrls,
    href: String,
    id: String,

    #[serde(alias = "type")]
    _type: String,

    uri: String,
}

/// Object representing a simplified track to Deserialize into
#[derive(Deserialize, Debug)]
pub(crate) struct TempSimplifiedTrackObject {
    artists: Vec<TempSimplifiedArtistObject>,
    external_urls: TempExternalUrls,
    href: String,
    id: String,
    is_playable: Option<bool>,
    linked_from: Option<TempTrackLinkedFrom>,
    restrictions: TempRestriction,
    name: String,
    preview_url: Option<String>,
    track_number: i32,

    #[serde(alias = "type")]
    _type: String,

    uri: String,
    is_local: bool,
}
