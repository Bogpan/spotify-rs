use serde::Serialize;

use crate::{
    auth::AuthFlow,
    error::Result,
    model::search::{Item, SearchQuery, SearchResults},
    query_list,
};

use super::{Client, Endpoint};

impl Endpoint for SearchEndpoint {}

/// Search for an item. The query can be either a string or
/// [`SearchQuery`](crate::model::search::SearchQuery). More details about
/// search queries can be found
/// [here](https://developer.spotify.com/documentation/web-api/reference/search).
pub fn search(query: impl Into<SearchQuery>, item_types: &[Item]) -> SearchEndpoint {
    let r#type = query_list(item_types);
    let query = query.into().to_string();

    SearchEndpoint {
        query,
        r#type,
        ..Default::default()
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct SearchEndpoint {
    #[serde(rename = "q")]
    pub(crate) query: String,
    pub(crate) r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) market: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) offset: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) include_external: Option<bool>,
}

impl SearchEndpoint {
    #[doc = include_str!("../docs/market.md")]
    pub fn market(mut self, market: impl Into<String>) -> Self {
        self.market = Some(market.into());
        self
    }

    #[doc = include_str!("../docs/limit.md")]
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    #[doc = include_str!("../docs/offset.md")]
    pub fn offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }

    /// If specified, it signals that the client can play externally hosted audio content,
    /// and marks the content as playable in the response.
    ///
    /// By default externally hosted audio content is marked as unplayable in the response.
    pub fn include_external(mut self, include_external: bool) -> Self {
        self.include_external = Some(include_external);
        self
    }

    /// Allows you to change the types of items to search.
    pub fn item_types(mut self, item_types: &[Item]) -> Self {
        self.r#type = query_list(item_types);
        self
    }

    #[doc = include_str!("../docs/send.md")]
    pub async fn get(self, spotify: &Client<impl AuthFlow>) -> Result<SearchResults> {
        spotify.get("/search".to_owned(), self).await
    }
}
