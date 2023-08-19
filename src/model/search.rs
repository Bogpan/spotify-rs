use std::fmt::Display;

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

#[derive(Clone, Debug, Deserialize)]
pub struct SearchResults {
    pub tracks: Option<Page<Track>>,
    pub artists: Option<Page<Artist>>,
    pub albums: Option<Page<SimplifiedAlbum>>,
    pub playlists: Option<Page<SimplifiedPlaylist>>,
    pub shows: Option<Page<SimplifiedShow>>,
    pub episodes: Option<Page<SimplifiedEpisode>>,
    pub audiobooks: Option<Page<SimplifiedAudiobook>>,
}

#[derive(Clone, Debug)]
pub enum Item {
    Album,
    Artist,
    Playlist,
    Track,
    Show,
    Episode,
    Audiobook,
}

impl Item {
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
