use serde::Deserialize;

use crate::objects::TempExternalUrls;

/// Temporary object representing a SimplifiedArtistObject to Deserialize into
#[derive(Deserialize, Debug)]
pub(crate) struct TempSimplifiedArtistObject {
    external_urls: TempExternalUrls,
    href: String,
    id: String,
    name: String,

    #[serde(alias = "type")]
    _type: String,

    uri: String,
}
