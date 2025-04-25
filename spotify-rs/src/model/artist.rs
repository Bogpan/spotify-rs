use serde::Deserialize;
use spotify_rs_macros::docs;

use crate::endpoint::user::FollowedArtistsEndpoint;

use super::*;

/// An artist.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[docs]
pub struct Artist {
    pub external_urls: ExternalUrls,
    /// Information about the followers of the artist.
    pub followers: Followers,
    /// A list of genres with which the artist is associated. May be empty.
    pub genres: Vec<String>,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub name: String,
    /// The popularity of the artist, represented as a number between 1 - 100.
    pub popularity: u32,
    pub r#type: String,
    pub uri: String,
}

/// A simplified artist, missing some details, that is usually obtained through
/// endpoints not specific to artists. The `href` field may be used to get a\
/// full artist.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[docs(name = "artist")]
pub struct SimplifiedArtist {
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    pub name: String,
    pub r#type: String,
    pub uri: String,
}

// Used only to deserialize JSON responses with arrays that are named objects.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub(crate) struct Artists {
    pub(crate) artists: Vec<Artist>,
}

// Used only to deserialize JSON responses with arrays that are named objects.
#[derive(Clone, Debug, Deserialize)]
pub(crate) struct PagedArtists {
    pub(crate) artists: CursorPage<Artist, FollowedArtistsEndpoint>,
}
