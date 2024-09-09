use serde::{Deserialize, Serialize};
use spotify_rs_macros::docs;

use super::*;

/// Information about the current user, which can only be obtained when
/// authorised by the user.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[docs(name = "user")]
pub struct PrivateUser {
    /// An [ISO 3661-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)
    /// code that represents the user's country, as set in the user's account.
    ///
    /// Note: this field is only available if the user is authorised with the
    /// `user-read-private` scope.
    pub country: String,
    /// The name that is displayed on the user's profile.
    pub display_name: Option<String>,
    /// The user's email address.
    ///
    /// Note: this email address is *unverified*, meaning that there is no proof
    /// that it actually belongs to the user; this field is only available if the
    /// user is authorised with the `user-read-email` scope.
    pub email: String,
    /// The user's explicit content settings.
    ///
    /// Note: This field is only available if the user is authorised with the
    /// `user-read-private` scope.
    pub explicit_content: Option<ExplicitContent>,
    pub external_urls: ExternalUrls,
    /// The followers of a user.
    pub followers: Followers,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    /// The user's Spotify subscription tier. The value `open` can be considered
    /// the same as `free`.
    ///
    /// Note: This field is only available if the user is authorised with the
    /// `user-read-private` scope.
    pub product: Option<String>,
    pub r#type: String,
    pub uri: String,
}

/// A user.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[docs]
pub struct User {
    /// The name that is displayed on the user's profile.
    pub display_name: Option<String>,
    pub external_urls: ExternalUrls,
    /// The followers of a user.
    pub followers: Followers,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub r#type: String,
    pub uri: String,
}

// Returned by the get/playlist/{id} endpoint; also called "PlaylistUserObject" in the schema
// It is missing the followers and images field from the regular User struct.
/// A user, returned usually as a playlist's owner.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[docs(name = "user")]
pub struct ReferenceUser {
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    pub r#type: String,
    pub uri: String,
    /// The name that is displayed on the user's profile.
    pub display_name: Option<String>,
}

/// A user's explicit content settings.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct ExplicitContent {
    /// Whether or not explicit content should be played.
    pub filter_enabled: bool,
    /// Whether or not the explicit content setting is locked and
    /// can't be modified by the user.
    pub filter_locked: bool,
}

/// Over what timespan the top items are calculated.
#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TimeRange {
    /// Calculated from the last ~1 year of data.
    LongTerm,
    /// Calculated from the last ~6 months of data.
    #[default]
    MediumTerm,
    /// Calculated from the last ~4 weeks of data.
    ShortTerm,
}
