use serde::{Deserialize, Deserializer};

pub mod album;
pub mod artist;
pub mod audio;
pub mod audiobook;
pub mod category;
pub mod market;
pub mod player;
pub mod playlist;
pub mod recommendation;
pub mod search;
pub mod show;
pub mod track;
pub mod user;

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Page<T: Clone> {
    pub href: String,
    pub limit: u32,
    pub next: Option<String>,
    pub offset: u32,
    pub previous: Option<String>,
    pub total: u32,
    /// A list of the items, which includes `null` values.
    /// To get only the `Some` values, use [`filtered_items`](Self::filtered_items).
    pub items: Vec<Option<T>>,
}

impl<T: Clone> Page<T> {
    /// Get a list of only the `Some` values from a Page's items.
    pub fn filtered_items(&self) -> Vec<T> {
        self.items.clone().into_iter().flatten().collect()
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct CursorPage<T: Clone> {
    pub href: String,
    pub limit: u32,
    pub next: Option<String>,
    pub cursors: Cursor,
    pub total: Option<u32>,
    pub items: Vec<Option<T>>,
}

impl<T: Clone> CursorPage<T> {
    /// Get a list of only the `Some` values from a Cursor Page's items.
    pub fn filtered_items(&self) -> Vec<T> {
        self.items.clone().into_iter().flatten().collect()
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Cursor {
    pub after: Option<String>,
    pub before: Option<String>,
}

/// An image used in various contexts.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Image {
    /// The URL of the image.
    pub url: String,
    /// The height in pixels.
    pub height: Option<u32>,
    /// The width in pixels.
    pub width: Option<u32>,
}

/// A copyright statement.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Copyright {
    /// The copyright text.
    pub text: String,
    /// The copyright type.
    pub r#type: CopyrightType,
}

/// A content restriction.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Restriction {
    /// The reason for the restriction.
    pub reason: RestrictionReason,
}

/// Contains known external IDs for content.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct ExternalIds {
    /// The [International Standard Recording Code](https://en.wikipedia.org/wiki/International_Standard_Recording_Code)
    /// for the content.
    pub isrc: Option<String>,
    /// The [International Article Number](https://en.wikipedia.org/wiki/International_Article_Number)
    /// for the content.
    pub ean: Option<String>,
    /// The [Universal Product Code](https://en.wikipedia.org/wiki/Universal_Product_Code)
    /// for the content.
    pub upc: Option<String>,
}

/// Contains external URLs for content. Currently, it seems that only Spotify
/// URLs are included here.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct ExternalUrls {
    /// The [Spotify URL](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids)
    /// for the content.
    pub spotify: String,
}

/// Information about the followers of an artist, playlist or user.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Followers {
    /// This will always be set to null, as the Web API does not support it at the moment.
    pub href: Option<String>,
    /// The total amount of followers.
    pub total: u32,
}

/// The user's latest position in a chapter or episode.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct ResumePoint {
    /// Whether or not the chapter or episode has fully been played by the user.
    pub fully_played: bool,
    /// The user's latest position in miliseconds.
    pub resume_position_ms: u32,
}

/// The reason for restriction on content.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum RestrictionReason {
    /// A restriction set because of the market of a user.
    Market,
    /// A restriction set because of the user's subscription type.
    Product,
    /// A restriction set because the content is explicit, and the user settings
    /// are set so that explicit conent can't be played.
    Explicit,
    #[serde(other)]
    /// Any other type of restriction, as more may be added in the future.
    Unknown,
}

/// The copyright type for a piece of content:
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub enum CopyrightType {
    #[serde(rename = "C")]
    /// The copyright.
    Copyright,
    #[serde(rename = "P")]
    /// The sound recording (performance) copyright.
    Performance,
}

/// The precision with which a date is known.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DatePrecision {
    /// The date is known at the year level.
    Year,
    /// The date is known at the month level.
    Month,
    /// The date is known at the day level.
    Day,
}

/// An item that can be played.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum PlayableItem {
    /// A Spotify track (song).
    Track(track::Track),
    /// An episode of a show.
    Episode(show::Episode),
}

// A function to convert a "null" JSON value to the default of given type,
// to make the API slightly nicer to use for people.
fn null_to_default<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Default + Deserialize<'de>,
    D: Deserializer<'de>,
{
    Ok(Option::deserialize(deserializer)?.unwrap_or_default())
}
