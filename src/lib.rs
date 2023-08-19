pub mod auth;
pub mod client;
pub mod endpoint;
mod error;
pub mod model;

pub(crate) fn query_list<T: AsRef<str>>(list: &[T]) -> String {
    list.iter()
        .map(|i| i.as_ref())
        .collect::<Vec<&str>>()
        .join(",")
}

pub type Result<T> = std::result::Result<T, error::Error>;

pub use oauth2::RedirectUrl;
use serde::{Deserialize, Deserializer};

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
