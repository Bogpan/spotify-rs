use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_json::Value;

use crate::{
    auth::AuthFlow,
    model::{
        playlist::{
            FeaturedPlaylists, Playlist, PlaylistTrack, Playlists, SimplifiedPlaylist, SnapshotId,
        },
        Page,
    },
    Nil, Result,
};

use super::{Builder, Endpoint, Limit, PrivateEndpoint};

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

#[derive(Clone, Debug, Default, Serialize)]
pub struct PlaylistEndpoint {
    #[serde(skip)]
    pub(crate) id: String,
    pub(crate) market: Option<String>,
}

impl<F: AuthFlow> Builder<'_, F, PlaylistEndpoint> {
    pub fn market(mut self, market: &str) -> Self {
        self.endpoint.market = Some(market.to_owned());
        self
    }

    pub async fn get(self) -> Result<Playlist> {
        self.spotify
            .get(format!("/playlists/{}", self.endpoint.id), self.endpoint)
            .await
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

impl<F: AuthFlow> Builder<'_, F, ChangePlaylistDetailsEndpoint> {
    pub fn name(mut self, name: &str) -> Self {
        self.endpoint.name = Some(name.to_owned());
        self
    }

    pub fn public(mut self, public: bool) -> Self {
        self.endpoint.public = Some(public);
        self
    }

    pub fn collaborative(mut self, collaborative: bool) -> Self {
        self.endpoint.collaborative = Some(collaborative);
        self
    }

    pub fn description(mut self, description: &str) -> Self {
        self.endpoint.description = Some(description.to_owned());
        self
    }

    pub async fn change(self) -> Result<Nil> {
        self.spotify
            .put(
                format!("/playlists/{}", self.endpoint.id),
                self.endpoint.json(),
            )
            .await
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct PlaylistItemsEndpoint {
    #[serde(skip)]
    pub(crate) id: String,
    pub(crate) market: Option<String>,
    pub(crate) limit: Option<Limit>,
    pub(crate) offset: Option<u32>,
}

impl<F: AuthFlow> Builder<'_, F, PlaylistItemsEndpoint> {
    pub fn market(mut self, market: &str) -> Self {
        self.endpoint.market = Some(market.to_owned());
        self
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.endpoint.limit = Some(Limit::new(limit));
        self
    }

    pub fn offset(mut self, offset: u32) -> Self {
        self.endpoint.offset = Some(offset);
        self
    }

    pub async fn get(self) -> Result<Page<PlaylistTrack>> {
        self.spotify
            .get(
                format!("/playlists/{}/tracks", self.endpoint.id),
                self.endpoint,
            )
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

impl<F: AuthFlow> Builder<'_, F, UpdatePlaylistItemsEndpoint> {
    pub fn uris<T: ToString>(mut self, uris: &[T]) -> Self {
        self.endpoint.uris = Some(uris.iter().map(ToString::to_string).collect());
        self
    }

    pub fn range_length(mut self, range_length: u32) -> Self {
        self.endpoint.range_length = Some(range_length);
        self
    }

    pub fn snapshot_id(mut self, snapshot_id: &str) -> Self {
        self.endpoint.snapshot_id = Some(snapshot_id.to_owned());
        self
    }

    pub async fn update(self) -> Result<String> {
        self.spotify
            .put(
                format!("/playlists/{}/tracks", self.endpoint.id),
                self.endpoint.json(),
            )
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

impl<F: AuthFlow> Builder<'_, F, AddPlaylistItemsEndpoint> {
    pub fn position(mut self, position: u32) -> Self {
        self.endpoint.position = Some(position);
        self
    }

    pub async fn add(self) -> Result<String> {
        self.spotify
            .post(
                format!("/playlists/{}/tracks", self.endpoint.id),
                self.endpoint.json(),
            )
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

impl<F: AuthFlow> Builder<'_, F, RemovePlaylistItemsEndpoint> {
    pub fn snapshot_id(mut self, snapshot_id: &str) -> Self {
        self.endpoint.snapshot_id = Some(snapshot_id.to_owned());
        self
    }

    pub async fn remove(self) -> Result<String> {
        self.spotify
            .delete(
                format!("/playlists/{}/tracks", self.endpoint.id),
                self.endpoint.json(),
            )
            .await
            .map(|i: SnapshotId| i.snapshot_id)
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct CurrentUserPlaylistsEndpoint {
    pub(crate) limit: Option<Limit>,
    pub(crate) offset: Option<u32>,
}

impl<F: AuthFlow> Builder<'_, F, CurrentUserPlaylistsEndpoint> {
    pub fn limit(mut self, limit: u32) -> Self {
        self.endpoint.limit = Some(Limit::new(limit));
        self
    }

    pub fn offset(mut self, offset: u32) -> Self {
        self.endpoint.offset = Some(offset);
        self
    }

    pub async fn get(self) -> Result<Page<SimplifiedPlaylist>> {
        self.spotify
            .get("/me/playlists".to_owned(), self.endpoint)
            .await
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct UserPlaylistsEndpoint {
    #[serde(skip)]
    pub(crate) id: String,
    pub(crate) limit: Option<Limit>,
    pub(crate) offset: Option<u32>,
}

impl<F: AuthFlow> Builder<'_, F, UserPlaylistsEndpoint> {
    pub fn limit(mut self, limit: u32) -> Self {
        self.endpoint.limit = Some(Limit::new(limit));
        self
    }

    pub fn offset(mut self, offset: u32) -> Self {
        self.endpoint.offset = Some(offset);
        self
    }

    pub async fn get(self) -> Result<Page<SimplifiedPlaylist>> {
        self.spotify
            .get(
                format!("/users/{}/playlists", self.endpoint.id),
                self.endpoint,
            )
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

impl<'a, F: AuthFlow> Builder<'_, F, CreatePlaylistEndpoint<'a>> {
    pub fn public(mut self, public: bool) -> Self {
        self.endpoint.public = Some(public);
        self
    }

    pub fn collaborative(mut self, collaborative: bool) -> Self {
        self.endpoint.collaborative = Some(collaborative);
        self
    }

    pub fn description(mut self, description: &str) -> Self {
        self.endpoint.description = Some(description.to_owned());
        self
    }

    pub fn tracks(mut self, track_uris: &'a [&str]) -> Self {
        self.endpoint.tracks = Some(track_uris);
        self
    }

    pub async fn create(self) -> Result<Playlist> {
        let tracks = self.endpoint.tracks;

        let mut playlist: Playlist = self
            .spotify
            .post(
                format!("/users/{}/playlists", self.endpoint.user_id),
                self.endpoint.json(),
            )
            .await?;

        if let Some(tracks) = tracks {
            self.spotify
                .add_items_to_playlist(&playlist.id, tracks)
                .add()
                .await?;

            let tracks = self.spotify.playlist_items(&playlist.id).get().await?;
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
    pub(crate) limit: Option<Limit>,
    pub(crate) offset: Option<u32>,
}

impl<F: AuthFlow> Builder<'_, F, FeaturedPlaylistsEndpoint> {
    pub fn country(mut self, country: &str) -> Self {
        self.endpoint.country = Some(country.to_owned());
        self
    }

    pub fn locale(mut self, locale: &str) -> Self {
        self.endpoint.locale = Some(locale.to_owned());
        self
    }

    pub fn timestamp(mut self, timestamp: DateTime<Utc>) -> Self {
        self.endpoint.timestamp = Some(timestamp.to_rfc3339());
        self
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.endpoint.limit = Some(Limit::new(limit));
        self
    }

    pub fn offset(mut self, offset: u32) -> Self {
        self.endpoint.offset = Some(offset);
        self
    }

    pub async fn get(self) -> Result<FeaturedPlaylists> {
        self.spotify
            .get("/browse/featured-playlists".to_owned(), self.endpoint)
            .await
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct CategoryPlaylistsEndpoint {
    #[serde(skip)]
    pub(crate) id: String,
    pub(crate) country: Option<String>,
    pub(crate) limit: Option<Limit>,
    pub(crate) offset: Option<u32>,
}

impl<F: AuthFlow> Builder<'_, F, CategoryPlaylistsEndpoint> {
    pub fn country(mut self, country: &str) -> Self {
        self.endpoint.country = Some(country.to_owned());
        self
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.endpoint.limit = Some(Limit::new(limit));
        self
    }

    pub fn offset(mut self, offset: u32) -> Self {
        self.endpoint.offset = Some(offset);
        self
    }

    pub async fn get(self) -> Result<Page<SimplifiedPlaylist>> {
        self.spotify
            .get(
                format!("/browse/categories/{}/playlists", self.endpoint.id),
                self.endpoint,
            )
            .await
            .map(|p: Playlists| p.playlists)
    }
}
