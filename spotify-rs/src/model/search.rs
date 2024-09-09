use std::str::FromStr;

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
