use base64::{engine::general_purpose, Engine};
use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_json::{json, Value};

use crate::{
    auth::{AuthFlow, Authorised},
    client::Body,
    error::Result,
    model::{
        playlist::{
            FeaturedPlaylists, Playlist, PlaylistItem, Playlists, SimplifiedPlaylist, SnapshotId,
        },
        Image, Page,
    },
    Nil,
};

use super::{Client, Endpoint, EndpointPrivate};

impl Endpoint for PlaylistEndpoint {}
impl Endpoint for ChangePlaylistDetailsEndpoint {}
impl Endpoint for PlaylistItemsEndpoint {}
impl Endpoint for UpdatePlaylistItemsEndpoint {}
impl Endpoint for AddPlaylistItemsEndpoint {}
impl Endpoint for RemovePlaylistItemsEndpoint {}
impl Endpoint for CurrentUserPlaylistsEndpoint {}
impl Endpoint for UserPlaylistsEndpoint {}
impl Endpoint for CreatePlaylistEndpoint<'_> {}
impl Endpoint for FeaturedPlaylistsEndpoint {}
impl Endpoint for CategoryPlaylistsEndpoint {}

pub fn playlist(id: impl Into<String>) -> PlaylistEndpoint {
    PlaylistEndpoint {
        id: id.into(),
        ..Default::default()
    }
}

pub fn change_playlist_details(id: impl Into<String>) -> ChangePlaylistDetailsEndpoint {
    ChangePlaylistDetailsEndpoint {
        id: id.into(),
        ..Default::default()
    }
}

pub fn playlist_items(id: impl Into<String>) -> PlaylistItemsEndpoint {
    PlaylistItemsEndpoint {
        id: id.into(),
        ..Default::default()
    }
}

// split into two functions: replace and reoder playlist items
// (the endpoint serves two functions)?
pub fn update_playlist_items(
    id: impl Into<String>,
    range_start: u32,
    insert_before: u32,
) -> UpdatePlaylistItemsEndpoint {
    UpdatePlaylistItemsEndpoint {
        id: id.into(),
        range_start,
        insert_before,
        ..Default::default()
    }
}

pub fn add_items_to_playlist<T: ToString>(
    id: impl Into<String>,
    item_uris: &[T],
) -> AddPlaylistItemsEndpoint {
    AddPlaylistItemsEndpoint {
        id: id.into(),
        uris: item_uris.iter().map(ToString::to_string).collect(),
        position: None,
    }
}

pub fn remove_playlist_items<T: AsRef<str>>(
    id: impl Into<String>,
    item_uris: &[T],
) -> RemovePlaylistItemsEndpoint {
    let tracks = item_uris
        .iter()
        .map(|u| json!({ "uri": u.as_ref() }))
        .collect();

    RemovePlaylistItemsEndpoint {
        id: id.into(),
        tracks,
        snapshot_id: None,
    }
}

pub fn current_user_playlists() -> CurrentUserPlaylistsEndpoint {
    CurrentUserPlaylistsEndpoint::default()
}

pub fn user_playlists(user_id: impl Into<String>) -> UserPlaylistsEndpoint {
    UserPlaylistsEndpoint {
        id: user_id.into(),
        ..Default::default()
    }
}

pub fn create_playlist<'a>(
    user_id: impl Into<String>,
    name: impl Into<String>,
) -> CreatePlaylistEndpoint<'a> {
    CreatePlaylistEndpoint {
        user_id: user_id.into(),
        name: name.into(),
        ..Default::default()
    }
}

/// **Note:** This endpoint has been deprecated by Spotify. It continues to work for
/// applications already using the extended mode in the API.
///
/// You can read more about this [here](https://developer.spotify.com/blog/2024-11-27-changes-to-the-web-api).
pub fn featured_playlists() -> FeaturedPlaylistsEndpoint {
    FeaturedPlaylistsEndpoint::default()
}

/// **Note:** This endpoint has been deprecated by Spotify. It continues to work for
/// applications already using the extended mode in the API.
///
/// You can read more about this [here](https://developer.spotify.com/blog/2024-11-27-changes-to-the-web-api).
pub fn category_playlists(category_id: impl Into<String>) -> CategoryPlaylistsEndpoint {
    CategoryPlaylistsEndpoint {
        id: category_id.into(),
        ..Default::default()
    }
}

pub async fn get_playlist_image(
    id: impl Into<String>,
    spotify: &Client<impl AuthFlow>,
) -> Result<Vec<Image>> {
    spotify
        .get::<(), _>(format!("/playlists/{}/images", id.into()), None)
        .await
}

pub async fn add_playlist_image(
    id: impl Into<String>,
    image: &[u8],
    spotify: &Client<impl AuthFlow + Authorised>,
) -> Result<Nil> {
    let encoded_image = general_purpose::STANDARD.encode(image).into_bytes();
    let body = <Body>::File(encoded_image);

    spotify
        .put(format!("/playlists/{}/images", id.into()), body)
        .await
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct PlaylistEndpoint {
    #[serde(skip)]
    pub(crate) id: String,
    pub(crate) market: Option<String>,
}

impl PlaylistEndpoint {
    #[doc = include_str!("../docs/market.md")]
    pub fn market(mut self, market: impl Into<String>) -> Self {
        self.market = Some(market.into());
        self
    }

    #[doc = include_str!("../docs/send.md")]
    pub async fn get(self, spotify: &Client<impl AuthFlow>) -> Result<Playlist> {
        spotify.get(format!("/playlists/{}", self.id), self).await
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct ChangePlaylistDetailsEndpoint {
    #[serde(skip)]
    pub(crate) id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) public: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) collaborative: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) description: Option<String>,
}

impl ChangePlaylistDetailsEndpoint {
    /// The new name for the playlist.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Whether or not to make the playlist public.
    pub fn public(mut self, public: bool) -> Self {
        self.public = Some(public);
        self
    }

    /// If true, other users will be able to modify the playlist.
    ///
    /// You can only set `collaborative` to `true` on private playlists.
    pub fn collaborative(mut self, collaborative: bool) -> Self {
        self.collaborative = Some(collaborative);
        self
    }

    /// The new description for the playlist.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    #[doc = include_str!("../docs/send.md")]
    pub async fn send(self, spotify: &Client<impl AuthFlow + Authorised>) -> Result<Nil> {
        spotify
            .put(format!("/playlists/{}", self.id), self.json())
            .await
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct PlaylistItemsEndpoint {
    #[serde(skip)]
    pub(crate) id: String,
    pub(crate) market: Option<String>,
    pub(crate) limit: Option<u32>,
    pub(crate) offset: Option<u32>,
}

impl PlaylistItemsEndpoint {
    #[doc = include_str!("../docs/market.md")]
    pub fn market(mut self, market: impl Into<String>) -> Self {
        self.market = Some(market.into());
        self
    }

    #[doc = include_str!("../docs/limit.md")]
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    #[doc = include_str!("../docs/offset.md")]
    pub fn offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }

    #[doc = include_str!("../docs/send.md")]
    pub async fn get(self, spotify: &Client<impl AuthFlow>) -> Result<Page<PlaylistItem>> {
        spotify
            .get(format!("/playlists/{}/tracks", self.id), self)
            .await
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct UpdatePlaylistItemsEndpoint {
    #[serde(skip)]
    pub(crate) id: String,
    pub(crate) range_start: u32,
    pub(crate) insert_before: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) uris: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) range_length: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) snapshot_id: Option<String>,
}

impl UpdatePlaylistItemsEndpoint {
    /// The Spotify *URIs* of the items to add (an item can be a track or episode).
    pub fn uris<T: ToString>(mut self, uris: &[T]) -> Self {
        self.uris = Some(uris.iter().map(ToString::to_string).collect());
        self
    }

    /// The amount of items to be reordered. Defaults to `1`.
    ///
    /// The range of items to be reordered begins from the range_start position,
    /// and includes the range_length subsequent items.
    ///
    /// For example, to move the items at index 9-10 to the start of the playlist,
    /// `range_start` should be 9 and `range_length` 2.
    pub fn range_length(mut self, range_length: u32) -> Self {
        self.range_length = Some(range_length);
        self
    }

    /// The playlist's snapshot ID against which to make changes.
    pub fn snapshot_id(mut self, snapshot_id: impl Into<String>) -> Self {
        self.snapshot_id = Some(snapshot_id.into());
        self
    }

    #[doc = include_str!("../docs/send.md")]
    pub async fn send(self, spotify: &Client<impl AuthFlow + Authorised>) -> Result<String> {
        spotify
            .put(format!("/playlists/{}/tracks", self.id), self.json())
            .await
            .map(|i: SnapshotId| i.snapshot_id)
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct AddPlaylistItemsEndpoint {
    #[serde(skip)]
    pub(crate) id: String,
    pub(crate) uris: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) position: Option<u32>,
}

impl AddPlaylistItemsEndpoint {
    /// The position to insert the items at, zero-based. If omitted, items will be appended to the playlist.
    pub fn position(mut self, position: u32) -> Self {
        self.position = Some(position);
        self
    }

    #[doc = include_str!("../docs/send.md")]
    pub async fn send(self, spotify: &Client<impl AuthFlow + Authorised>) -> Result<String> {
        spotify
            .post(format!("/playlists/{}/tracks", self.id), self.json())
            .await
            .map(|i: SnapshotId| i.snapshot_id)
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct RemovePlaylistItemsEndpoint {
    #[serde(skip)]
    pub(crate) id: String,
    pub(crate) tracks: Vec<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) snapshot_id: Option<String>,
}

impl RemovePlaylistItemsEndpoint {
    /// The playlist's snapshot ID against which to make changes.
    pub fn snapshot_id(mut self, snapshot_id: impl Into<String>) -> Self {
        self.snapshot_id = Some(snapshot_id.into());
        self
    }

    #[doc = include_str!("../docs/send.md")]
    pub async fn send(self, spotify: &Client<impl AuthFlow + Authorised>) -> Result<String> {
        spotify
            .delete(format!("/playlists/{}/tracks", self.id), self.json())
            .await
            .map(|i: SnapshotId| i.snapshot_id)
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct CurrentUserPlaylistsEndpoint {
    pub(crate) limit: Option<u32>,
    pub(crate) offset: Option<u32>,
}

impl CurrentUserPlaylistsEndpoint {
    #[doc = include_str!("../docs/limit.md")]
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    #[doc = include_str!("../docs/offset.md")]
    pub fn offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }

    #[doc = include_str!("../docs/send.md")]
    pub async fn get(
        self,
        spotify: &Client<impl AuthFlow + Authorised>,
    ) -> Result<Page<SimplifiedPlaylist>> {
        spotify.get("/me/playlists".to_owned(), self).await
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct UserPlaylistsEndpoint {
    #[serde(skip)]
    pub(crate) id: String,
    pub(crate) limit: Option<u32>,
    pub(crate) offset: Option<u32>,
}

impl UserPlaylistsEndpoint {
    #[doc = include_str!("../docs/limit.md")]
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    #[doc = include_str!("../docs/offset.md")]
    pub fn offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }

    #[doc = include_str!("../docs/send.md")]
    pub async fn get(self, spotify: &Client<impl AuthFlow>) -> Result<Page<SimplifiedPlaylist>> {
        spotify
            .get(format!("/users/{}/playlists", self.id), self)
            .await
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct CreatePlaylistEndpoint<'a> {
    #[serde(skip)]
    pub(crate) user_id: String,
    #[serde(skip)]
    pub(crate) tracks: Option<&'a [&'a str]>,
    pub(crate) name: String,
    pub(crate) public: Option<bool>,
    pub(crate) collaborative: Option<bool>,
    pub(crate) description: Option<String>,
}

impl<'a> CreatePlaylistEndpoint<'a> {
    /// Whether or not to make the playlist public. Defaults to `true`.
    ///
    /// Note: Setting *public* to `false` only stops the playlist from being
    /// displayed on the user's profile and from appearing in search results,
    /// it does *not* modify the access (thus anyone with the link can access
    /// the playlist), as the Spotify Web API
    /// [doesn't allow modifying access](https://developer.spotify.com/documentation/web-api/concepts/playlists).
    pub fn public(mut self, public: bool) -> Self {
        self.public = Some(public);
        self
    }

    /// If true, other users will be able to modify the playlist.
    ///
    /// You can only set `collaborative` to `true` on private playlists.
    /// Defaults to `false`.
    pub fn collaborative(mut self, collaborative: bool) -> Self {
        self.collaborative = Some(collaborative);
        self
    }

    /// The description for the new playlist.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// This will create the playlist with the given tracks in it.
    ///
    /// Note: This will make an additional API call to set the tracks, it is not
    /// part of the Spotify API parameters.
    pub fn tracks(mut self, track_uris: &'a [&str]) -> Self {
        self.tracks = Some(track_uris);
        self
    }

    #[doc = include_str!("../docs/send.md")]
    pub async fn send(self, spotify: &Client<impl AuthFlow + Authorised>) -> Result<Playlist> {
        let tracks = self.tracks;

        let mut playlist: Playlist = spotify
            .post(format!("/users/{}/playlists", self.user_id), self.json())
            .await?;

        if let Some(tracks) = tracks {
            add_items_to_playlist(&playlist.id, tracks)
                .send(spotify)
                .await?;

            let tracks = playlist_items(&playlist.id).get(spotify).await?;
            playlist.tracks = tracks;
        }

        Ok(playlist)
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct FeaturedPlaylistsEndpoint {
    pub(crate) country: Option<String>,
    pub(crate) locale: Option<String>,
    pub(crate) timestamp: Option<String>,
    pub(crate) limit: Option<u32>,
    pub(crate) offset: Option<u32>,
}

impl FeaturedPlaylistsEndpoint {
    #[doc = include_str!("../docs/country.md")]
    pub fn country(mut self, country: impl Into<String>) -> Self {
        self.country = Some(country.into());
        self
    }

    #[doc = include_str!("../docs/locale.md")]
    pub fn locale(mut self, locale: impl Into<String>) -> Self {
        self.locale = Some(locale.into());
        self
    }

    /// An [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) timestamp (`yyyy-MM-ddTHH:mm:ss`)
    pub fn timestamp(mut self, timestamp: DateTime<Utc>) -> Self {
        self.timestamp = Some(timestamp.to_rfc3339());
        self
    }

    #[doc = include_str!("../docs/limit.md")]
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    #[doc = include_str!("../docs/offset.md")]
    pub fn offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }

    #[doc = include_str!("../docs/send.md")]
    pub async fn get(self, spotify: &Client<impl AuthFlow>) -> Result<FeaturedPlaylists> {
        spotify
            .get("/browse/featured-playlists".to_owned(), self)
            .await
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct CategoryPlaylistsEndpoint {
    #[serde(skip)]
    pub(crate) id: String,
    pub(crate) country: Option<String>,
    pub(crate) limit: Option<u32>,
    pub(crate) offset: Option<u32>,
}

impl CategoryPlaylistsEndpoint {
    #[doc = include_str!("../docs/country.md")]
    pub fn country(mut self, country: impl Into<String>) -> Self {
        self.country = Some(country.into());
        self
    }

    #[doc = include_str!("../docs/limit.md")]
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    #[doc = include_str!("../docs/offset.md")]
    pub fn offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }

    #[doc = include_str!("../docs/send.md")]
    pub async fn get(self, spotify: &Client<impl AuthFlow>) -> Result<Page<SimplifiedPlaylist>> {
        spotify
            .get(format!("/browse/categories/{}/playlists", self.id), self)
            .await
            .map(|p: Playlists| p.playlists)
    }
}
