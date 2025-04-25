use serde::Serialize;
use serde_json::json;

use crate::{
    auth::{AuthFlow, Authorised},
    client::Body,
    error::Result,
    model::{
        artist::{Artist, PagedArtists},
        track::Track,
        user::{PrivateUser, TimeRange, User},
        CursorPage, Page,
    },
    query_list, Nil,
};

use super::{Client, Endpoint, EndpointPrivate};

pub async fn get_current_user_profile(
    spotify: &Client<impl AuthFlow + Authorised>,
) -> Result<PrivateUser> {
    spotify.get::<(), _>("/me".to_owned(), None).await
}

pub fn current_user_top_artists() -> UserTopItemsEndpoint<ArtistsMarker> {
    UserTopItemsEndpoint::default()
}

pub fn current_user_top_tracks() -> UserTopItemsEndpoint<TracksMarker> {
    UserTopItemsEndpoint::default()
}

pub async fn get_user(id: impl Into<String>, spotify: &Client<impl AuthFlow>) -> Result<User> {
    spotify
        .get::<(), _>(format!("/users/{}", id.into()), None)
        .await
}

pub fn follow_playlist(id: impl Into<String>) -> FollowPlaylistEndpoint {
    FollowPlaylistEndpoint {
        id: id.into(),
        public: None,
    }
}

pub async fn unfollow_playlist(
    id: impl Into<String>,
    spotify: &Client<impl AuthFlow + Authorised>,
) -> Result<Nil> {
    spotify
        .delete::<(), _>(format!("/playlists/{}/followers", id.into()), None)
        .await
}

pub fn followed_artists() -> FollowedArtistsEndpoint {
    // Currently only the "artist" type is supported, so it's hardcoded.
    FollowedArtistsEndpoint {
        r#type: "artist".to_owned(),
        ..Default::default()
    }
}

pub async fn follow_artists<T: AsRef<str>>(
    ids: &[T],
    spotify: &Client<impl AuthFlow + Authorised>,
) -> Result<Nil> {
    let ids: Vec<String> = ids.iter().map(|i| i.as_ref().to_owned()).collect();

    spotify
        .put(
            "/me/following?type=artist".to_owned(),
            Body::Json(json!({ "ids": ids })),
        )
        .await
}

pub async fn unfollow_artists<T: AsRef<str>>(
    ids: &[T],
    spotify: &Client<impl AuthFlow + Authorised>,
) -> Result<Nil> {
    let ids: Vec<String> = ids.iter().map(|i| i.as_ref().to_owned()).collect();

    spotify
        .delete(
            "/me/following?type=artist".to_owned(),
            Body::Json(json!({ "ids": ids })),
        )
        .await
}

pub async fn check_if_user_follows_artists<T: AsRef<str>>(
    ids: &[T],
    spotify: &Client<impl AuthFlow + Authorised>,
) -> Result<Vec<bool>> {
    spotify
        .get::<(), _>(
            format!("/me/following/contains?type=artist&ids={}", query_list(ids)),
            None,
        )
        .await
}

pub async fn follow_users<T: AsRef<str>>(
    ids: &[T],
    spotify: &Client<impl AuthFlow + Authorised>,
) -> Result<Nil> {
    let ids: Vec<String> = ids.iter().map(|i| i.as_ref().to_owned()).collect();

    spotify
        .put(
            "/me/following?type=user".to_owned(),
            Body::Json(json!({ "ids": ids })),
        )
        .await
}

pub async fn unfollow_users<T: AsRef<str>>(
    ids: &[T],
    spotify: &Client<impl AuthFlow + Authorised>,
) -> Result<Nil> {
    let ids: Vec<String> = ids.iter().map(|i| i.as_ref().to_owned()).collect();

    spotify
        .delete(
            "/me/following?type=user".to_owned(),
            Body::Json(json!({ "ids": ids })),
        )
        .await
}

pub async fn check_if_user_follows_users<T: AsRef<str>>(
    ids: &[T],
    spotify: &Client<impl AuthFlow + Authorised>,
) -> Result<Vec<bool>> {
    spotify
        .get::<(), _>(
            format!("/me/following/contains?type=user&ids={}", query_list(ids)),
            None,
        )
        .await
}

pub async fn check_if_current_user_follow_playlist(
    playlist_id: impl Into<String>,
    spotify: &Client<impl AuthFlow + Authorised>,
) -> Result<Vec<bool>> {
    spotify
        .get::<(), _>(
            format!("/playlists/{}/followers/contains", playlist_id.into()),
            None,
        )
        .await
}

pub trait ItemType: private::Sealed {}
impl ItemType for ArtistsMarker {}
impl ItemType for TracksMarker {}

#[derive(Clone, Copy, Debug, Default)]
pub struct ArtistsMarker;
#[derive(Clone, Copy, Debug, Default)]
pub struct TracksMarker;

mod private {
    pub trait Sealed {}

    impl Sealed for super::ArtistsMarker {}
    impl Sealed for super::TracksMarker {}
}

impl<I: ItemType> Endpoint for UserTopItemsEndpoint<I> {}
impl Endpoint for FollowPlaylistEndpoint {}
impl Endpoint for FollowedArtistsEndpoint {}

#[derive(Clone, Debug, Default, Serialize)]
pub struct UserTopItemsEndpoint<ItemType> {
    pub(crate) time_range: Option<TimeRange>,
    pub(crate) limit: Option<u32>,
    pub(crate) offset: Option<u32>,
    #[serde(skip)]
    marker: std::marker::PhantomData<ItemType>,
}

impl<I: ItemType> UserTopItemsEndpoint<I> {
    /// The time frame of the computed affinities.
    pub fn time_range(mut self, time_range: TimeRange) -> Self {
        self.time_range = Some(time_range);
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
}

impl UserTopItemsEndpoint<ArtistsMarker> {
    #[doc = include_str!("../docs/send.md")]
    pub async fn get(self, spotify: &Client<impl AuthFlow + Authorised>) -> Result<Page<Artist>> {
        spotify.get("/me/top/artists".to_owned(), self).await
    }
}

impl UserTopItemsEndpoint<TracksMarker> {
    #[doc = include_str!("../docs/send.md")]
    pub async fn get(self, spotify: &Client<impl AuthFlow + Authorised>) -> Result<Page<Track>> {
        spotify.get("/me/top/tracks".to_owned(), self).await
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct FollowPlaylistEndpoint {
    #[serde(skip)]
    pub(crate) id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) public: Option<bool>,
}

impl FollowPlaylistEndpoint {
    /// If set to `true`, the playlist will be included in the user's
    /// public playlists. Defaults to `true`.
    pub fn public(mut self, public: bool) -> Self {
        self.public = Some(public);
        self
    }

    #[doc = include_str!("../docs/send.md")]
    pub async fn send(self, spotify: &Client<impl AuthFlow + Authorised>) -> Result<Nil> {
        spotify
            .put(format!("/playlists/{}/followers", self.id), self.json())
            .await
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct FollowedArtistsEndpoint {
    pub(crate) r#type: String,
    pub(crate) after: Option<String>,
    pub(crate) limit: Option<u32>,
}

impl FollowedArtistsEndpoint {
    /// The last artist ID retrieved from the previous request.
    pub fn after(mut self, artist_id: impl Into<String>) -> Self {
        self.after = Some(artist_id.into());
        self
    }

    #[doc = include_str!("../docs/limit.md")]
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    #[doc = include_str!("../docs/send.md")]
    pub async fn get(
        self,
        spotify: &Client<impl AuthFlow + Authorised>,
    ) -> Result<CursorPage<Artist, Self>> {
        spotify
            .get("/me/following".to_owned(), self)
            .await
            .map(|a: PagedArtists| a.artists)
    }
}
