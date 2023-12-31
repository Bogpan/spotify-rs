use std::{collections::HashMap, marker::PhantomData};

use serde::Serialize;
use strum::IntoStaticStr;

use crate::{
    auth::{AuthFlow, Verifier},
    error::Result,
    model::{
        recommendation::Recommendations,
        track::{SavedTrack, Track, Tracks},
        Page,
    },
    query_list,
};

use super::{Builder, Endpoint, Limit};

impl Endpoint for TrackEndpoint {}
impl Endpoint for TracksEndpoint {}
impl Endpoint for SavedTracksEndpoint {}
impl<S: SeedType> Endpoint for RecommendationsEndpoint<S> {}

pub trait SeedType {}
impl SeedType for SeedArtists {}
impl SeedType for SeedGenres {}
impl SeedType for SeedTracks {}

pub enum SeedArtists {}
pub enum SeedGenres {}
pub enum SeedTracks {}

#[derive(Clone, Debug)]
pub enum Seed<'a, T: AsRef<str>, S: SeedType> {
    Artists(&'a [T], PhantomData<S>),
    Genres(&'a [T], PhantomData<S>),
    Tracks(&'a [T], PhantomData<S>),
}

impl<'a, T: AsRef<str> + Clone> Seed<'a, T, SeedArtists> {
    pub fn artists(ids: &'a [T]) -> Self {
        Self::Artists(ids, PhantomData)
    }
}

impl<'a, T: AsRef<str> + Clone> Seed<'a, T, SeedGenres> {
    pub fn genres(genres: &'a [T]) -> Self {
        Self::Genres(genres, PhantomData)
    }
}

impl<'a, T: AsRef<str> + Clone> Seed<'a, T, SeedTracks> {
    pub fn tracks(ids: &'a [T]) -> Self {
        Self::Tracks(ids, PhantomData)
    }
}

#[derive(Clone, Copy, Debug, Serialize, IntoStaticStr)]
#[serde(untagged)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Feature {
    MinAcousticness(f32),
    MaxAcousticness(f32),
    TargetAcousticness(f32),
    MinDanceability(f32),
    MaxDanceability(f32),
    TargetDanceability(f32),
    MinDurationMs(u32),
    MaxDurationMs(u32),
    TargetDurationMs(u32),
    MinEnergy(f32),
    MaxEnergy(f32),
    TargetEnergy(f32),
    MinInstrumentalness(f32),
    MaxInstrumentalness(f32),
    TargetInstrumentalness(f32),
    MinKey(u32),
    MaxKey(u32),
    TargetKey(u32),
    MinLiveness(f32),
    MaxLiveness(f32),
    TargetLiveness(f32),
    MinLoudness(f32),
    MaxLoudness(f32),
    TargetLoudness(f32),
    MinMode(u32),
    MaxMode(u32),
    TargetMode(u32),
    MinPopularity(u32),
    MaxPopularity(u32),
    TargetPopularity(u32),
    MinSpeechiness(f32),
    MaxSpeechiness(f32),
    TargetSpeechiness(f32),
    MinTempo(f32),
    MaxTempo(f32),
    TargetTempo(f32),
    MinTimeSignature(u32),
    MaxTimeSignature(u32),
    TargetTimeSignature(u32),
    MinValence(f32),
    MaxValence(f32),
    TargetValence(f32),
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct TrackEndpoint {
    #[serde(skip)]
    pub(crate) id: String,
    pub(crate) market: Option<String>,
}

impl<F: AuthFlow, V: Verifier> Builder<'_, F, V, TrackEndpoint> {
    #[doc = include_str!("../docs/market.md")]
    pub fn market(mut self, market: impl Into<String>) -> Self {
        self.endpoint.market = Some(market.into());
        self
    }

    #[doc = include_str!("../docs/send.md")]
    pub async fn get(self) -> Result<Track> {
        self.spotify
            .get(format!("/tracks/{}", self.endpoint.id), self.endpoint)
            .await
    }
}
#[derive(Clone, Debug, Default, Serialize)]
pub struct TracksEndpoint {
    pub(crate) ids: String,
    pub(crate) market: Option<String>,
}

impl<F: AuthFlow, V: Verifier> Builder<'_, F, V, TracksEndpoint> {
    #[doc = include_str!("../docs/market.md")]
    pub fn market(mut self, market: impl Into<String>) -> Self {
        self.endpoint.market = Some(market.into());
        self
    }

    #[doc = include_str!("../docs/send.md")]
    pub async fn get(self) -> Result<Vec<Track>> {
        self.spotify
            .get("/tracks".to_owned(), self.endpoint)
            .await
            .map(|t: Tracks| t.tracks)
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct SavedTracksEndpoint {
    pub(crate) market: Option<String>,
    pub(crate) limit: Option<Limit>,
    pub(crate) offset: Option<u32>,
}

impl<F: AuthFlow, V: Verifier> Builder<'_, F, V, SavedTracksEndpoint> {
    #[doc = include_str!("../docs/market.md")]
    pub fn market(mut self, market: impl Into<String>) -> Self {
        self.endpoint.market = Some(market.into());
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
    pub async fn get(self) -> Result<Page<SavedTrack>> {
        self.spotify
            .get("/me/tracks".to_owned(), self.endpoint)
            .await
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct RecommendationsEndpoint<S: SeedType> {
    pub(crate) seed_artists: Option<String>,
    pub(crate) seed_genres: Option<String>,
    pub(crate) seed_tracks: Option<String>,
    pub(crate) limit: Option<Limit<1, 100>>,
    pub(crate) market: Option<String>,
    #[serde(flatten)]
    pub(crate) features: Option<HashMap<&'static str, Feature>>,
    #[serde(skip)]
    pub(crate) marker: PhantomData<S>,
}

impl<F: AuthFlow, V: Verifier> Builder<'_, F, V, RecommendationsEndpoint<SeedArtists>> {
    /// Up to 5 Spotify genre IDs used for seeding the recommendations.
    pub fn seed_genres<T: AsRef<str>>(mut self, genres: &[T]) -> Self {
        self.endpoint.seed_genres = Some(query_list(genres));
        self
    }

    /// Up to 5 Spotify track IDs used for seeding the recommendations.
    pub fn seed_tracks<T: AsRef<str>>(mut self, track_ids: &[T]) -> Self {
        self.endpoint.seed_tracks = Some(query_list(track_ids));
        self
    }
}

impl<F: AuthFlow, V: Verifier> Builder<'_, F, V, RecommendationsEndpoint<SeedGenres>> {
    /// Up to 5 Spotify artist IDs used for seeding the recommendations.
    pub fn seed_artists<T: AsRef<str>>(mut self, artist_ids: &[T]) -> Self {
        self.endpoint.seed_genres = Some(query_list(artist_ids));
        self
    }

    /// Up to 5 Spotify track IDs used for seeding the recommendations.
    pub fn seed_tracks<T: AsRef<str>>(mut self, track_ids: &[T]) -> Self {
        self.endpoint.seed_tracks = Some(query_list(track_ids));
        self
    }
}

impl<F: AuthFlow, V: Verifier> Builder<'_, F, V, RecommendationsEndpoint<SeedTracks>> {
    /// Up to 5 Spotify genre IDs used for seeding the recommendations.
    pub fn seed_genres<T: AsRef<str>>(mut self, genres: &[T]) -> Self {
        self.endpoint.seed_genres = Some(query_list(genres));
        self
    }

    /// Up to 5 Spotify artist IDs used for seeding the recommendations.
    pub fn seed_artists<T: AsRef<str>>(mut self, artist_ids: &[T]) -> Self {
        self.endpoint.seed_genres = Some(query_list(artist_ids));
        self
    }
}

impl<F: AuthFlow, V: Verifier, S: SeedType> Builder<'_, F, V, RecommendationsEndpoint<S>> {
    #[doc = include_str!("../docs/limit.md")]
    pub fn limit(mut self, limit: u32) -> Self {
        self.endpoint.limit = Some(Limit::new(limit));
        self
    }

    #[doc = include_str!("../docs/market.md")]
    pub fn market(mut self, market: impl Into<String>) -> Self {
        self.endpoint.market = Some(market.into());
        self
    }

    /// A list of [`Feature`]s. Read more about the available features
    /// [here](https://developer.spotify.com/documentation/web-api/reference/get-recommendations).
    pub fn features(mut self, features: &[Feature]) -> Self {
        let features: HashMap<&'static str, Feature> = features
            .iter()
            .map(|f| (From::<Feature>::from(*f), *f))
            .collect();
        dbg!(&features);
        self.endpoint.features = Some(features);
        self
    }

    #[doc = include_str!("../docs/send.md")]
    pub async fn get(self) -> Result<Recommendations> {
        self.spotify
            .get("/recommendations".to_owned(), self.endpoint)
            .await
    }
}
