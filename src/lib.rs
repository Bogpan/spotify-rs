pub mod auth;
pub mod client;
pub mod endpoint;
mod error;
pub mod model;

use client::Body;
use serde::{Deserialize, Deserializer};

pub(crate) fn query_list<T: AsRef<str>>(list: &[T]) -> String {
    list.iter()
        .map(|i| i.as_ref())
        .collect::<Vec<&str>>()
        .join(",")
}

pub(crate) fn body_list<T: AsRef<str>>(name: &str, list: &[T]) -> Body<Value> {
    let list: Vec<_> = list.iter().map(|i| i.as_ref()).collect();
    Body::Json(json!({ name: list }))
}

pub type Result<T> = std::result::Result<T, error::Error>;

pub use auth::{AuthCodeGrantFlow, AuthCodeGrantPKCEFlow, ClientCredsGrantFlow};
pub use client::Client;
pub use oauth2::RedirectUrl;
use serde_json::{json, Value};

/// Represents an empty API response.
pub struct Nil;

impl<'de> Deserialize<'de> for Nil {
    fn deserialize<D>(_: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Nil)
    }
}
