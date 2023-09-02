use serde::Serialize;

use crate::{
    auth::{AuthFlow, Verifier},
    error::Result,
    model::search::SearchResults,
};

use super::{Builder, Endpoint, Limit};

impl Endpoint for SearchEndpoint {}

#[derive(Clone, Debug, Default, Serialize)]
pub struct SearchEndpoint {
    #[serde(rename = "q")]
    pub(crate) query: String,
    pub(crate) r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) market: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) limit: Option<Limit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) offset: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) include_external: Option<bool>,
}

impl<F: AuthFlow, V: Verifier> Builder<'_, F, V, SearchEndpoint> {
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

    /// If specified, it signals that the client can play externally hosted audio content,
    /// and marks the content as playable in the response.
    ///
    /// By default externally hosted audio content is marked as unplayable in the response.
    pub fn include_external(mut self, include_external: bool) -> Self {
        self.endpoint.include_external = Some(include_external);
        self
    }

    #[doc = include_str!("../docs/send.md")]
    pub async fn get(self) -> Result<SearchResults> {
        self.spotify.get("/search".to_owned(), self.endpoint).await
    }
}
