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
pub trait Endpoint: Serialize {
    // This method isn't necessary, thus it's not implemented for all endpoints
    // It's used for pagination, for keeping track of the current endpoint
    // a `Page` refers to.
    //
    // However, in the future it might be implemented for all endpoints,
    // for consistency's sake.
    fn endpoint_url(&self) -> &'static str {
        "TODO (default URL)"
    }
}

impl<T: Endpoint> EndpointPrivate for T {}

// Trait to add endpoint methods that make writing the endpoints
// more convenient.
pub(crate) trait EndpointPrivate: Serialize + Endpoint {
    // Convenience method used to convert a type (an endpoint in this case)
    // to a Body::Json.
    fn json(self) -> crate::client::Body<Self>
    where
        Self: Sized,
    {
        crate::client::Body::Json(self)
    }
}
