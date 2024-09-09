use serde::Serialize;

pub mod album;
pub mod artist;
pub mod audiobook;
pub mod category;
pub mod genres;
pub mod markets;
pub mod player;
pub mod playlist;
pub mod search;
pub mod show;
pub mod track;
pub mod user;

// Authenticated client type to make it more convenient to use in the endpoints.
type Client<F> = crate::client::Client<crate::auth::Token, F>;

#[doc = include_str!("docs/internal_implementation_details.md")]
pub trait Endpoint: Serialize {}

impl<T: Endpoint> EndpointPrivate for T {}

// Trait to add endpoint methods that make writing the endpoints
// more convenient.
pub(crate) trait EndpointPrivate: Serialize {
    // Convenience method used to convert a type (an endpoint in this case)
    // to a Body::Json.
    fn json(self) -> crate::client::Body<Self>
    where
        Self: Sized,
    {
        crate::client::Body::Json(self)
    }
}
