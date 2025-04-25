use std::{collections::HashMap, fmt::Debug, marker::PhantomData};

use serde::{ser::SerializeMap, Serialize};
use strum::IntoStaticStr;

use crate::{
    auth::{AuthFlow, Authorised},
    body_list,
    error::Result,
    model::{
        audio::{AudioAnalysis, AudioFeatures, AudioFeaturesList, Mode},
        recommendation::Recommendations,
        track::{SavedTrack, Track, Tracks},
        Page,
    },
    query_list, Nil,
};

use super::{Client, Endpoint};

pub fn track(id: impl Into<String>) -> TrackEndpoint {
    TrackEndpoint {
        id: id.into(),
        market: None,
    }
}

pub fn tracks<T: AsRef<str>>(ids: &[T]) -> TracksEndpoint {
    TracksEndpoint {
        ids: query_list(ids),
        market: None,
    }
}

pub fn saved_tracks() -> SavedTracksEndpoint {
    SavedTracksEndpoint::default()
}

pub async fn save_tracks<T: AsRef<str>>(
    ids: &[T],
    spotify: &Client<impl AuthFlow + Authorised>,
) -> Result<Nil> {
    spotify
        .put("/me/tracks".to_owned(), body_list("ids", ids))
        .await
}

pub async fn remove_saved_tracks<T: AsRef<str>>(
    ids: &[T],
    spotify: &Client<impl AuthFlow + Authorised>,
) -> Result<Nil> {
    spotify
        .delete("/me/tracks".to_owned(), body_list("ids", ids))
        .await
}

pub async fn check_saved_tracks<T: AsRef<str>>(
    ids: &[T],
    spotify: &Client<impl AuthFlow + Authorised>,
) -> Result<Vec<bool>> {
    spotify
        .get("/me/tracks/contains".to_owned(), [("ids", query_list(ids))])
        .await
}

/// **Note:** This endpoint has been deprecated by Spotify. It continues to work for
/// applications already using the extended mode in the API.
///
/// You can read more about this [here](https://developer.spotify.com/blog/2024-11-27-changes-to-the-web-api).
pub async fn get_track_audio_features(
    id: impl Into<String>,
    spotify: &Client<impl AuthFlow>,
) -> Result<AudioFeatures> {
    spotify
        .get::<(), _>(format!("/audio-features/{}", id.into()), None)
        .await
}

/// **Note:** This endpoint has been deprecated by Spotify. It continues to work for
/// applications already using the extended mode in the API.
///
/// You can read more about this [here](https://developer.spotify.com/blog/2024-11-27-changes-to-the-web-api).
pub async fn get_tracks_audio_features<T: AsRef<str>>(
    ids: &[T],
    spotify: &Client<impl AuthFlow>,
) -> Result<Vec<Option<AudioFeatures>>> {
    spotify
        .get("/audio-features".to_owned(), [("ids", query_list(ids))])
        .await
        .map(|a: AudioFeaturesList| a.audio_features)
}

pub async fn get_track_audio_analysis(
    id: impl Into<String>,
    spotify: &Client<impl AuthFlow>,
) -> Result<AudioAnalysis> {
    spotify
        .get::<(), _>(format!("/audio-analysis/{}", id.into()), None)
        .await
}

/// Get recommendations based on given seeds. You must specify at least one
/// seed (whether that be a seed artist, track or genre). More seed types can
/// be used optionally via the builder.
///
/// **Note:** This endpoint has been deprecated by Spotify. It continues to work for
/// applications already using the extended mode in the API.
///
/// You can read more about this [here](https://developer.spotify.com/blog/2024-11-27-changes-to-the-web-api).
#[doc = include_str!("../docs/seed_limit.md")]
pub fn recommendations<S: SeedType, T: AsRef<str>>(seed: Seed<T, S>) -> RecommendationsEndpoint<S> {
    let (seed_artists, seed_genres, seed_tracks) = match seed {
        Seed::Artists(ids, _) => (Some(query_list(ids)), None, None),
        Seed::Genres(genres, _) => (None, Some(query_list(genres)), None),
        Seed::Tracks(ids, _) => (None, None, Some(query_list(ids))),
    };

    RecommendationsEndpoint {
        seed_artists,
        seed_genres,
        seed_tracks,
        limit: None,
        market: None,
        features: None,
        marker: std::marker::PhantomData,
    }
}

impl Endpoint for TrackEndpoint {}
impl Endpoint for TracksEndpoint {}
impl Endpoint for SavedTracksEndpoint {}
impl<S: SeedType> Endpoint for RecommendationsEndpoint<S> {}

pub trait SeedType: Debug {}
impl SeedType for SeedArtists {}
impl SeedType for SeedGenres {}
impl SeedType for SeedTracks {}

#[derive(Clone, Copy, Debug)]
pub enum SeedArtists {}
#[derive(Clone, Copy, Debug)]
pub enum SeedGenres {}
#[derive(Clone, Copy, Debug)]
pub enum SeedTracks {}

#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum Seed<'a, T: AsRef<str>, S: SeedType> {
    Artists(&'a [T], PhantomData<S>),
    Genres(&'a [T], PhantomData<S>),
    Tracks(&'a [T], PhantomData<S>),
}

impl<'a, T: AsRef<str> + Clone> Seed<'a, T, SeedArtists> {
    #[doc = include_str!("../docs/seed_limit.md")]
    pub fn artists(ids: &'a [T]) -> Self {
        Self::Artists(ids, PhantomData)
    }
}

impl<'a, T: AsRef<str> + Clone> Seed<'a, T, SeedGenres> {
    #[doc = include_str!("../docs/seed_limit.md")]
    pub fn genres(genres: &'a [T]) -> Self {
        Self::Genres(genres, PhantomData)
    }
}

impl<'a, T: AsRef<str> + Clone> Seed<'a, T, SeedTracks> {
    #[doc = include_str!("../docs/seed_limit.md")]
    pub fn tracks(ids: &'a [T]) -> Self {
        Self::Tracks(ids, PhantomData)
    }
}

// #[derive(Clone, Copy, Debug, Serialize, IntoStaticStr)]
// #[serde(untagged)]
// #[serde(rename_all = "snake_case")]
// #[strum(serialize_all = "snake_case")]
// pub enum Feature {
//     MinAcousticness(f32),
//     MaxAcousticness(f32),
//     TargetAcousticness(f32),
//     MinDanceability(f32),
//     MaxDanceability(f32),
//     TargetDanceability(f32),
//     MinDurationMs(u32),
//     MaxDurationMs(u32),
//     TargetDurationMs(u32),
//     MinEnergy(f32),
//     MaxEnergy(f32),
//     TargetEnergy(f32),
//     MinInstrumentalness(f32),
//     MaxInstrumentalness(f32),
//     TargetInstrumentalness(f32),
//     MinKey(u32),
//     MaxKey(u32),
//     TargetKey(u32),
//     MinLiveness(f32),
//     MaxLiveness(f32),
//     TargetLiveness(f32),
//     MinLoudness(f32),
//     MaxLoudness(f32),
//     TargetLoudness(f32),
//     MinMode(u32),
//     MaxMode(u32),
//     TargetMode(u32),
//     MinPopularity(u32),
//     MaxPopularity(u32),
//     TargetPopularity(u32),
//     MinSpeechiness(f32),
//     MaxSpeechiness(f32),
//     TargetSpeechiness(f32),
//     MinTempo(f32),
//     MaxTempo(f32),
//     TargetTempo(f32),
//     MinTimeSignature(u32),
//     MaxTimeSignature(u32),
//     TargetTimeSignature(u32),
//     MinValence(f32),
//     MaxValence(f32),
//     TargetValence(f32),
// }

/// Represents what feature exactly is set.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum FeatureKind {
    Acousticness,
    Danceability,
    DurationMs,
    Energy,
    Instrumentalness,
    Key,
    Liveness,
    Loudness,
    Mode,
    Popularity,
    Speechiness,
    Tempo,
    TimeSignature,
    Valence,
}

/// Represents the value of a feature. They can be either floats or values.
///
/// The value can be directly specified (e.g. 0, 100, 0.5, Mode::Major etc.), as this type
/// implements `From` for various type.
///
/// **Note:** You *must* use the right values yourself for each option. They can either take
/// integers from 0 - 100, floats from 0 - 1, or special types (like [`Mode`](crate::model::audio::Mode))
///
/// In order to know what values each feature takes, you can use
/// [this](https://developer.spotify.com/documentation/web-api/reference/get-recommendations)
/// page.
///
/// # Example
///
/// ```rs
/// let recommendations = recommendations(Seed::artists(&["59XQUEHhy5830QsAsmhe2M"]))
/// .features(&[
///     Feature::min(FeatureKind::Energy, 0.5),
///     Feature::max(FeatureKind::Popularity, 64),
///     Feature::target(FeatureKind::Mode, Mode::Major),
/// ])
/// .get(spotify)
/// .await?;
/// ```
#[derive(Clone, Copy, Debug, Serialize)]
#[serde(untagged)]
pub enum FeatureValue {
    Float(f32),
    Int(u32),
}

impl From<Vec<Feature>> for Features {
    fn from(value: Vec<Feature>) -> Self {
        Self(value)
    }
}

impl<const N: usize> From<[Feature; N]> for Features {
    fn from(value: [Feature; N]) -> Self {
        Self(value.to_vec())
    }
}

impl From<&[Feature]> for Features {
    fn from(value: &[Feature]) -> Self {
        Self(value.to_vec())
    }
}

/// Represesnts a list of features. You can use an array or vector instead of it,
/// as this type implements the `From` trait.
#[derive(Clone, Debug, Default)]
pub struct Features(Vec<Feature>);

impl Serialize for Features {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = HashMap::new();

        for element in &self.0 {
            let kind: &'static str = element.kind.into();

            if let Some(target) = element.target {
                map.insert(format!("target_{kind}"), target);
            }

            if let Some(min) = element.min {
                map.insert(format!("min_{kind}"), min);
            }

            if let Some(max) = element.max {
                map.insert(format!("max_{kind}"), max);
            }
        }

        let mut serialized_map = serializer.serialize_map(Some(map.len()))?;

        for (k, v) in map {
            serialized_map.serialize_entry(&k, &v)?
        }

        serialized_map.end()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Feature {
    kind: FeatureKind,
    target: Option<FeatureValue>,
    min: Option<FeatureValue>,
    max: Option<FeatureValue>,
}

impl From<u32> for FeatureValue {
    fn from(value: u32) -> Self {
        Self::Int(value)
    }
}

impl From<f32> for FeatureValue {
    fn from(value: f32) -> Self {
        Self::Float(value)
    }
}

impl From<Mode> for FeatureValue {
    fn from(value: Mode) -> Self {
        Self::Int(value as u32)
    }
}

impl Feature {
    pub fn new<T: Into<FeatureValue>>(
        kind: FeatureKind,
        target: Option<T>,
        min: Option<T>,
        max: Option<T>,
    ) -> Self {
        Self {
            kind,
            target: target.map(Into::into),
            min: min.map(Into::into),
            max: max.map(Into::into),
        }
    }

    pub fn target<T: Into<FeatureValue>>(kind: FeatureKind, target: T) -> Self {
        Self {
            kind,
            target: Some(target.into()),
            min: None,
            max: None,
        }
    }

    pub fn min<T: Into<FeatureValue>>(kind: FeatureKind, min: T) -> Self {
        Self {
            kind,
            target: None,
            min: Some(min.into()),
            max: None,
        }
    }

    pub fn max<T: Into<FeatureValue>>(kind: FeatureKind, max: T) -> Self {
        Self {
            kind,
            target: None,
            min: None,
            max: Some(max.into()),
        }
    }

    pub fn exact<T: Into<FeatureValue>>(kind: FeatureKind, value: T) -> Self {
        let value = value.into();

        Self {
            kind,
            target: Some(value),
            min: Some(value),
            max: Some(value),
        }
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct TrackEndpoint {
    #[serde(skip)]
    pub(crate) id: String,
    pub(crate) market: Option<String>,
}

impl TrackEndpoint {
    #[doc = include_str!("../docs/market.md")]
    pub fn market(mut self, market: impl Into<String>) -> Self {
        self.market = Some(market.into());
        self
    }

    #[doc = include_str!("../docs/send.md")]
    pub async fn get(self, spotify: &Client<impl AuthFlow>) -> Result<Track> {
        spotify.get(format!("/tracks/{}", self.id), self).await
    }
}
#[derive(Clone, Debug, Default, Serialize)]
pub struct TracksEndpoint {
    pub(crate) ids: String,
    pub(crate) market: Option<String>,
}

impl TracksEndpoint {
    #[doc = include_str!("../docs/market.md")]
    pub fn market(mut self, market: impl Into<String>) -> Self {
        self.market = Some(market.into());
        self
    }

    #[doc = include_str!("../docs/send.md")]
    pub async fn get(self, spotify: &Client<impl AuthFlow>) -> Result<Vec<Track>> {
        spotify
            .get("/tracks".to_owned(), self)
            .await
            .map(|t: Tracks| t.tracks)
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct SavedTracksEndpoint {
    pub(crate) market: Option<String>,
    pub(crate) limit: Option<u32>,
    pub(crate) offset: Option<u32>,
}

impl SavedTracksEndpoint {
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
    pub async fn get(
        self,
        spotify: &Client<impl AuthFlow + Authorised>,
    ) -> Result<Page<SavedTrack>> {
        spotify.get("/me/tracks".to_owned(), self).await
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct RecommendationsEndpoint<S: SeedType> {
    pub(crate) seed_artists: Option<String>,
    pub(crate) seed_genres: Option<String>,
    pub(crate) seed_tracks: Option<String>,
    pub(crate) limit: Option<u32>,
    pub(crate) market: Option<String>,
    #[serde(flatten)]
    pub(crate) features: Option<Features>,
    #[serde(skip)]
    pub(crate) marker: PhantomData<S>,
}

impl RecommendationsEndpoint<SeedArtists> {
    #[doc = include_str!("../docs/seed_limit.md")]
    pub fn seed_genres<T: AsRef<str>>(mut self, genres: &[T]) -> Self {
        self.seed_genres = Some(query_list(genres));
        self
    }

    #[doc = include_str!("../docs/seed_limit.md")]
    pub fn seed_tracks<T: AsRef<str>>(mut self, track_ids: &[T]) -> Self {
        self.seed_tracks = Some(query_list(track_ids));
        self
    }
}

impl RecommendationsEndpoint<SeedGenres> {
    #[doc = include_str!("../docs/seed_limit.md")]
    pub fn seed_artists<T: AsRef<str>>(mut self, artist_ids: &[T]) -> Self {
        self.seed_genres = Some(query_list(artist_ids));
        self
    }

    #[doc = include_str!("../docs/seed_limit.md")]
    pub fn seed_tracks<T: AsRef<str>>(mut self, track_ids: &[T]) -> Self {
        self.seed_tracks = Some(query_list(track_ids));
        self
    }
}

impl RecommendationsEndpoint<SeedTracks> {
    #[doc = include_str!("../docs/seed_limit.md")]
    pub fn seed_genres<T: AsRef<str>>(mut self, genres: &[T]) -> Self {
        self.seed_genres = Some(query_list(genres));
        self
    }

    #[doc = include_str!("../docs/seed_limit.md")]
    pub fn seed_artists<T: AsRef<str>>(mut self, artist_ids: &[T]) -> Self {
        self.seed_genres = Some(query_list(artist_ids));
        self
    }
}

impl<S: SeedType> RecommendationsEndpoint<S> {
    #[doc = include_str!("../docs/limit.md")]
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    #[doc = include_str!("../docs/market.md")]
    pub fn market(mut self, market: impl Into<String>) -> Self {
        self.market = Some(market.into());
        self
    }

    /// A list of [`Features`](Feature). Read more about the available features
    /// [here](https://developer.spotify.com/documentation/web-api/reference/get-recommendations).
    pub fn features(mut self, features: &[Feature]) -> Self {
        self.features = Some(features.into());
        self
    }

    #[doc = include_str!("../docs/send.md")]
    pub async fn get(self, spotify: &Client<impl AuthFlow>) -> Result<Recommendations> {
        spotify.get("/recommendations".to_owned(), self).await
    }
}
