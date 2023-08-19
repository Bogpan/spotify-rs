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
pub mod show;

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
