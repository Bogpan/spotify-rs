use serde::Serialize;

use crate::{
    auth::{AuthFlow, Token, Verifier},
    client::Client,
};

pub mod album;
pub mod artist;
pub mod audiobook;
pub mod category;
pub mod player;
pub mod playlist;
pub mod search;
pub mod show;
pub mod track;
pub mod user;

pub trait Endpoint: Serialize {}

pub(crate) trait PrivateEndpoint: Serialize {
    fn json(self) -> crate::client::Body<Self>
    where
        Self: Sized,
    {
        crate::client::Body::Json(self)
    }
}

impl<T: Endpoint> PrivateEndpoint for T {}

/// Builder for methods that get information from the API.
pub struct Builder<'s, F: AuthFlow, V: Verifier, E: Endpoint> {
    pub(crate) spotify: &'s mut Client<Token, F, V>,
    pub(crate) endpoint: E,
}

#[derive(Clone, Debug)]
pub(crate) struct Limit<const MIN: u32 = 1, const MAX: u32 = 50>(u32);

impl<const MIN: u32, const MAX: u32> Limit<MIN, MAX> {
    pub(crate) fn new(n: u32) -> Self {
        Self(n.clamp(MIN, MAX))
    }
}

impl<const MIN: u32, const MAX: u32> Serialize for Limit<MIN, MAX> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32(self.0)
    }
}
