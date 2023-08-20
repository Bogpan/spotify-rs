use std::fmt::Display;

use serde::{Deserialize, Serialize};

use super::{artist::Artist, track::Track, *};

#[derive(Clone, Debug, Deserialize)]
pub struct PrivateUser {
    pub country: String,
    pub display_name: Option<String>,
    pub email: String,
    /// The user's explicit content settings. This field is only available when the current user has granted access to the user-read-private scope.
    pub explicit_content: Option<ExplicitContent>,
    pub external_urls: ExternalUrls,
    pub followers: Followers,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    /// The user's Spotify subscription level: "premium", "free", etc. (The subscription level "open" can be considered the same as "free".) This field is only available when the current user has granted access to the user-read-private scope.
    pub product: Option<String>,
    pub r#type: String,
    pub uri: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct User {
    pub display_name: Option<String>,
    pub external_urls: ExternalUrls,
    pub followers: Followers,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub r#type: String,
    pub uri: String,
}

// Returned by the get/playlist/{id} endpoint; also called "PlaylistUserObject" in the schema
#[derive(Clone, Debug, Deserialize)]
pub struct ReferenceUser {
    pub external_urls: ExternalUrls,
    pub followers: Option<Followers>,
    pub href: String,
    pub id: String,
    pub r#type: String,
    pub uri: String,
    pub display_name: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ExplicitContent {
    pub filter_enabled: bool,
    pub filter_locked: bool,
}

#[derive(Clone, Debug, Default)]
pub enum UserItemType {
    #[default]
    Artists,
    Tracks,
}

impl Display for UserItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            UserItemType::Artists => "artists",
            UserItemType::Tracks => "tracks",
        };

        write!(f, "{s}")
    }
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TimeRange {
    LongTerm,
    #[default]
    MediumTerm,
    ShortTerm,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum UserItem {
    Artist(Box<Artist>),
    Track(Box<Track>),
}
