use serde::Serialize;

use crate::{
    auth::{AuthFlow, Token},
    client::Client,
};

pub mod album;
pub mod artist;
pub mod audiobook;
pub mod category;
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

pub struct Builder<'s, F: AuthFlow, E: Endpoint> {
    pub(crate) spotify: &'s mut Client<Token, F>,
    pub(crate) endpoint: E,
}

#[derive(Clone, Debug)]
pub(crate) struct BoundedU32<const MIN: u32, const MAX: u32>(u32);

impl<const MIN: u32, const MAX: u32> BoundedU32<MIN, MAX> {
    pub(crate) fn new(n: u32) -> Self {
        Self(n.clamp(MIN, MAX))
    }
}

pub(crate) type Limit = BoundedU32<1, 50>;

impl<const MIN: u32, const MAX: u32> Serialize for BoundedU32<MIN, MAX> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32(self.0)
    }
}
