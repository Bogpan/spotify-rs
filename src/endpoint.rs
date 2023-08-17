use crate::{
    auth::{AuthFlow, Token},
    client::Client,
};

pub mod album;
pub mod artist;
pub mod audiobook;
pub mod category;
pub mod show;

pub trait Endpoint {}

pub struct Builder<'s, F: AuthFlow, E: Endpoint> {
    pub(crate) spotify: &'s mut Client<Token, F>,
    pub(crate) endpoint: E,
}
