use std::{fmt::Display, str::FromStr};

use serde::Deserialize;

use super::{
    album::SimplifiedAlbum,
    artist::Artist,
    audiobook::SimplifiedAudiobook,
    playlist::SimplifiedPlaylist,
    show::{SimplifiedEpisode, SimplifiedShow},
    track::Track,
    Page,
};

/// Represents a search query builder.
#[derive(Clone, Debug, Default)]
pub struct SearchQuery {
    query: String,
    album: Option<String>,
    artist: Option<String>,
    track: Option<String>,
    year: Option<String>,
    irsc: Option<String>,
    genre: Option<String>,
    upc: Option<String>,
    hipster: bool,
    new: bool,
}

impl SearchQuery {
    pub fn from_query(query: impl Into<String>) -> Self {
        Self {
            query: query.into(),
            ..Default::default()
        }
    }

    pub fn album(mut self, album: impl Into<String>) -> Self {
        self.album = Some(album.into());
        self
    }

    pub fn artist(mut self, artist: impl Into<String>) -> Self {
        self.artist = Some(artist.into());
        self
    }

    pub fn track(mut self, track: impl Into<String>) -> Self {
        self.track = Some(track.into());
        self
    }

    pub fn year(mut self, year: u32) -> Self {
        self.year = Some(year.to_string());
        self
    }

    pub fn years(mut self, start_year: u32, end_year: u32) -> Self {
        self.year = Some(format!("{start_year}-{end_year}"));
        self
    }

    pub fn irsc(mut self, irsc: impl Into<String>) -> Self {
        self.irsc = Some(irsc.into());
        self
    }

    pub fn genre(mut self, genre: impl Into<String>) -> Self {
        self.genre = Some(genre.into());
        self
    }

    pub fn upc(mut self, upc: impl Into<String>) -> Self {
        self.upc = Some(upc.into());
        self
    }

    pub fn hipster(mut self, hipster: bool) -> Self {
        self.hipster = hipster;
        self
    }

    pub fn new(mut self, new: bool) -> Self {
        self.new = new;
        self
    }
}

impl<T> From<T> for SearchQuery
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        Self::from_query(value)
    }
}

// To format the query.
impl Display for SearchQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.query)?;

        if let Some(album) = &self.album {
            write!(f, " album:{album}")?;
        }

        if let Some(artist) = &self.artist {
            write!(f, " artist:{artist}")?;
        }

        if let Some(track) = &self.track {
            write!(f, " track:{track}")?;
        }

        if let Some(year) = &self.year {
            write!(f, " year:{year}")?;
        }

        if let Some(irsc) = &self.irsc {
            write!(f, " irsc:{irsc}")?;
        }

        if let Some(genre) = &self.genre {
            write!(f, " genre:{genre}")?;
        }

        if let Some(upc) = &self.upc {
            write!(f, " upc:{upc}")?;
        }

        if self.hipster {
            write!(f, " tag:hipster")?;
        }

        if self.new {
            write!(f, " tag:new")?;
        }

        Ok(())
    }
}

/// The results of a search.
///
/// Note: audiobooks are only available within the US, Canada, the UK, Ireland,
/// New Zealand and Australia.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct SearchResults {
    /// The track results.
    pub tracks: Option<Page<Track>>,
    /// The artist results.
    pub artists: Option<Page<Artist>>,
    /// The album results.
    pub albums: Option<Page<SimplifiedAlbum>>,
    /// The playlist results.
    pub playlists: Option<Page<SimplifiedPlaylist>>,
    /// The show results.
    pub shows: Option<Page<SimplifiedShow>>,
    /// The episode results.
    pub episodes: Option<Page<SimplifiedEpisode>>,
    /// The audiobook results.
    pub audiobooks: Option<Page<SimplifiedAudiobook>>,
}

/// An item type to search for.
///
/// You can either use [all](Self::all()) to get a list of all types of items,
/// or construct a list yourself to use in your search.
#[derive(Clone, Debug)]
pub enum Item {
    /// Album type.
    Album,
    /// Artist type.
    Artist,
    /// Playlist type.
    Playlist,
    /// Track type.
    Track,
    /// Show type.
    Show,
    /// Episode type.
    Episode,
    /// Audiobook type.
    Audiobook,
}

impl Item {
    /// Returns a list of all the types of item to use in a search.
    pub fn all() -> &'static [Self; 7] {
        &[
            Self::Album,
            Self::Artist,
            Self::Playlist,
            Self::Track,
            Self::Show,
            Self::Episode,
            Self::Audiobook,
        ]
    }
}

impl FromStr for Item {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "album" => Ok(Self::Album),
            "artist" => Ok(Self::Artist),
            "playlist" => Ok(Self::Playlist),
            "track" => Ok(Self::Track),
            "show" => Ok(Self::Show),
            "episode" => Ok(Self::Episode),
            "audiobook" => Ok(Self::Audiobook),
            _ => Err(crate::Error::Parse { description: format!("Failed to parse search item: {s} is not a valid item. Expected one of: *album, artist, playlist, track, show, episode, audiobook*.") }),
        }
    }
}

impl AsRef<str> for Item {
    fn as_ref(&self) -> &str {
        match self {
            Item::Album => "album",
            Item::Artist => "artist",
            Item::Playlist => "playlist",
            Item::Track => "track",
            Item::Show => "show",
            Item::Episode => "episode",
            Item::Audiobook => "audiobook",
        }
    }
}
