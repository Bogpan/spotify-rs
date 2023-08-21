use serde::Serialize;

use crate::{auth::AuthFlow, model::search::SearchResults, Result};

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

impl<F: AuthFlow> Builder<'_, F, SearchEndpoint> {
    pub fn market(mut self, market: &str) -> Self {
        self.endpoint.market = Some(market.to_owned());
        self
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.endpoint.limit = Some(Limit::new(limit));
        self
    }

    pub fn offset(mut self, offset: u32) -> Self {
        self.endpoint.offset = Some(offset);
        self
    }

    pub fn include_external(mut self, include_external: bool) -> Self {
        self.endpoint.include_external = Some(include_external);
        self
    }

    pub async fn get(self) -> Result<SearchResults> {
        self.spotify.get("/search".to_owned(), self.endpoint).await
    }
}
