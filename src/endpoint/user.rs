use serde::Serialize;
use serde_json::json;

use crate::{
    auth::AuthFlow,
    client::Body,
    error::Result,
    model::{
        artist::{Artist, PagedArtists},
        user::{TimeRange, UserItem, UserItemType},
        CursorPage, Page,
    },
    query_list, Nil,
};

use super::{Builder, Endpoint, Limit, PrivateEndpoint};

impl Endpoint for UserTopItemsEndpoint {}
impl Endpoint for FollowPlaylistBuilder {}
impl Endpoint for FollowedArtistsBuilder {}
impl Endpoint for FollowUserOrArtistEndpoint {}

#[derive(Clone, Debug, Default, Serialize)]
pub struct UserTopItemsEndpoint {
    #[serde(skip)]
    pub(crate) r#type: UserItemType,
    pub(crate) time_range: Option<TimeRange>,
    pub(crate) limit: Option<Limit>,
    pub(crate) offset: Option<u32>,
}

impl<F: AuthFlow> Builder<'_, F, UserTopItemsEndpoint> {
    /// The time frame of the computed affinities.
    pub fn time_range(mut self, time_range: TimeRange) -> Self {
        self.endpoint.time_range = Some(time_range);
        self
    }

    #[doc = include_str!("../docs/limit.md")]
    pub fn limit(mut self, limit: u32) -> Self {
        self.endpoint.limit = Some(Limit::new(limit));
        self
    }

    #[doc = include_str!("../docs/offset.md")]
    pub fn offset(mut self, offset: u32) -> Self {
        self.endpoint.offset = Some(offset);
        self
    }

    #[doc = include_str!("../docs/send.md")]
    pub async fn get(self) -> Result<Page<UserItem>> {
        self.spotify
            .get(format!("/me/top/{}", self.endpoint.r#type), self.endpoint)
            .await
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct FollowPlaylistBuilder {
    #[serde(skip)]
    pub(crate) id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) public: Option<bool>,
}

impl<F: AuthFlow> Builder<'_, F, FollowPlaylistBuilder> {
    /// If set to `true`, the playlist will be included in the user's
    /// public playlists. Defaults to `true`.
    pub fn public(mut self, public: bool) -> Self {
        self.endpoint.public = Some(public);
        self
    }

    #[doc = include_str!("../docs/send.md")]
    pub async fn send(self) -> Result<Nil> {
        self.spotify
            .put(
                format!("/playlists/{}/followers", self.endpoint.id),
                self.endpoint.json(),
            )
            .await
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct FollowedArtistsBuilder {
    pub(crate) r#type: String,
    pub(crate) after: Option<String>,
    pub(crate) limit: Option<Limit>,
}

impl<F: AuthFlow> Builder<'_, F, FollowedArtistsBuilder> {
    /// The last artist ID retrieved from the previous request.
    pub fn after(mut self, artist_id: &str) -> Self {
        self.endpoint.after = Some(artist_id.to_owned());
        self
    }

    #[doc = include_str!("../docs/limit.md")]
    pub fn limit(mut self, limit: u32) -> Self {
        self.endpoint.limit = Some(Limit::new(limit));
        self
    }

    #[doc = include_str!("../docs/send.md")]
    pub async fn get(self) -> Result<CursorPage<Artist>> {
        self.spotify
            .get("/me/following".to_owned(), self.endpoint)
            .await
            .map(|a: PagedArtists| a.artists)
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct FollowUserOrArtistEndpoint {
    pub(crate) r#type: String,
    #[serde(skip)]
    pub(crate) ids: Vec<String>,
}

impl<F: AuthFlow> Builder<'_, F, FollowUserOrArtistEndpoint> {
    #[doc = include_str!("../docs/send.md")]
    pub async fn follow(self) -> Result<Nil> {
        self.spotify
            .put(
                format!("/me/following?type={}", self.endpoint.r#type),
                Body::Json(json!({ "ids": self.endpoint.ids })),
            )
            .await
    }

    #[doc = include_str!("../docs/send.md")]
    pub async fn unfollow(self) -> Result<Nil> {
        self.spotify
            .delete(
                format!("/me/following?type={}", self.endpoint.r#type),
                Body::Json(json!({ "ids": self.endpoint.ids })),
            )
            .await
    }

    #[doc = include_str!("../docs/send.md")]
    pub async fn check(self) -> Result<Vec<bool>> {
        self.spotify
            .get(
                "/me/following/contains".to_owned(),
                [
                    ("type", self.endpoint.r#type),
                    ("ids", query_list(&self.endpoint.ids)),
                ],
            )
            .await
    }
}
