use serde::Deserialize;
use spotify_rs_macros::docs;

use super::*;

/// An audiobook.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[docs]
pub struct Audiobook {
    /// The author(s) of the audiobook.
    pub authors: Vec<Author>,
    #[serde(default)]
    pub available_markets: Vec<String>,
    pub copyrights: Vec<Copyright>,
    /// A text description of the audiobook.
    pub description: String,
    /// An description of the audiobook that may contain HTML tags.
    pub html_description: String,
    /// The edition of the audiobook.
    pub edition: String,
    pub explicit: bool,
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub languages: Vec<String>,
    pub media_type: String,
    pub name: String,
    /// The narrator(s) of the audiobook.
    pub narrators: Vec<Narrator>,
    pub publisher: String,
    pub r#type: String,
    pub uri: String,
    /// The amount of chapters the audiobook contains.
    pub total_chapters: u32,
    /// The chapters of the audiobook.
    pub chapters: Page<SimplifiedChapter>,
}

/// A simplified audiobook, missing some details, that is usually obtained
/// through endpoints not specific to audiobooks. The `href` field may be
/// used to get a full audiobook.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[docs(name = "audiobook")]
pub struct SimplifiedAudiobook {
    /// The author(s) of the audiobook.
    pub authors: Vec<Author>,
    #[serde(default)]
    pub available_markets: Vec<String>,
    pub copyrights: Vec<Copyright>,
    /// A text description of the audiobook.
    pub description: String,
    /// An description of the audiobook that may contain HTML tags.
    pub html_description: String,
    /// The edition of the audiobook.
    pub edition: String,
    /// Whether or not the audiobook has explicit content.
    /// `false` can also mean *unknown*.
    pub explicit: bool,
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    /// A list of [ISO 639](https://en.wikipedia.org/wiki/ISO_639) codes for the
    /// languages spoken in the audiobook.
    pub languages: Vec<String>,
    /// The type of the media of the audiobook.
    pub media_type: String,
    pub name: String,
    /// The narrator(s) of the audiobook.
    pub narrators: Vec<Narrator>,
    /// The publisher of the audiobook.
    pub publisher: String,
    pub r#type: String,
    pub uri: String,
    /// The amount of chapters the audiobook contains.
    pub total_chapters: Option<u32>,
}

// Used only to deserialize JSON responses with arrays that are named objects.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub(crate) struct Audiobooks {
    pub(crate) audiobooks: Vec<Option<Audiobook>>,
}

/// An audiobook chapter.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[docs]
pub struct Chapter {
    /// The URL for a 30 second MP3 preview of the chapter.
    ///
    /// **Note:** This attribute has been deprecated by Spotify. It continues to work for
    /// applications already using the extended mode in the API.
    ///
    /// You can read more about this [here](https://developer.spotify.com/blog/2024-11-27-changes-to-the-web-api).
    pub audio_preview_url: Option<String>,
    #[serde(default)]
    pub available_markets: Vec<String>,
    /// The number of the chapter in the audiobook it belongs to.
    pub chapter_number: u32,
    /// A text description of the audiobook.
    pub description: String,
    /// An description of the audiobook that may contain HTML tags.
    pub html_description: String,
    pub duration_ms: u32,
    /// Whether or not the audiobook has explicit content.
    /// `false` can also mean *unknown*.
    pub explicit: bool,
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub is_playable: Option<bool>,
    /// A list of [ISO 639](https://en.wikipedia.org/wiki/ISO_639) codes for the
    /// languages spoken in the audiobook.
    pub languages: Vec<String>,
    pub name: String,
    pub release_date: String,
    pub release_date_precision: DatePrecision,
    pub resume_point: Option<ResumePoint>,
    pub r#type: String,
    pub uri: String,
    /// Included in the response when a content restriction is applied.
    pub restrictions: Option<Restriction>,
    /// The audiobook to which the chapter belongs.
    pub audiobook: SimplifiedAudiobook,
}

/// A simplified chapter, missing some details, that is usually obtained
/// through endpoints not specific to chapters. The `href` field may be
/// used to get a full chapter.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[docs(name = "chapter")]
pub struct SimplifiedChapter {
    /// The URL for a 30 second MP3 preview of the chapter.
    ///
    /// **Note:** This attribute has been deprecated by Spotify. It continues to work for
    /// applications already using the extended mode in the API.
    ///
    /// You can read more about this [here](https://developer.spotify.com/blog/2024-11-27-changes-to-the-web-api).
    pub audio_preview_url: Option<String>,
    #[serde(default)]
    pub available_markets: Vec<String>,
    /// The number of the chapter in the audiobook it belongs to.
    pub chapter_number: u32,
    pub description: String,
    pub html_description: String,
    pub duration_ms: u32,
    pub explicit: bool,
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub is_playable: Option<bool>,
    pub languages: Vec<String>,
    pub name: String,
    pub release_date: String,
    pub release_date_precision: DatePrecision,
    pub resume_point: Option<ResumePoint>,
    pub r#type: String,
    pub uri: String,
    /// Included in the response when a content restriction is applied.
    pub restrictions: Option<Restriction>,
}

// Used only to deserialize JSON responses with arrays that are named objects.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub(crate) struct Chapters {
    pub(crate) chapters: Vec<Option<Chapter>>,
}

// Even though there's no point to these containers (and other types in the
// Spotify API), this library tries to adhere as closely as possible to the API.
/// An author of an audiobook.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Author {
    /// The name of the author.
    pub name: String,
}

/// A narrator of an audiobook
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Narrator {
    /// The name of the narrator.
    pub name: String,
}

impl Audiobook {
    /// Get the names of the author(s) of the audiobook.
    pub fn author_names(&self) -> Vec<String> {
        self.authors.iter().map(|a| a.name.clone()).collect()
    }

    /// Get the names of the narrator(s) of the audiobook.
    pub fn narrator_names(&self) -> Vec<String> {
        self.narrators.iter().map(|n| n.name.clone()).collect()
    }
}

impl SimplifiedAudiobook {
    /// Get the names of the author(s) of the audiobook.
    pub fn author_names(&self) -> Vec<String> {
        self.authors.iter().map(|a| a.name.clone()).collect()
    }

    /// Get the names of the narrator(s) of the audiobook.
    pub fn narrator_names(&self) -> Vec<String> {
        self.narrators.iter().map(|n| n.name.clone()).collect()
    }
}
