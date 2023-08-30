use serde::Serialize;

use crate::{
    auth::AuthFlow,
    error::Result,
    model::{
        category::{Categories, Category},
        Page,
    },
};

use super::{Builder, Endpoint, Limit};

impl Endpoint for BrowseCategoryEndpoint {}
impl Endpoint for BrowseCategoriesEndpoint {}

#[derive(Clone, Debug, Default, Serialize)]
pub struct BrowseCategoryEndpoint {
    #[serde(skip)]
    pub(crate) id: String,
    pub(crate) country: Option<String>,
    pub(crate) locale: Option<String>,
}

impl<F: AuthFlow> Builder<'_, F, BrowseCategoryEndpoint> {
    #[doc = include_str!("../docs/country.md")]
    pub fn country(mut self, country: &str) -> Self {
        self.endpoint.country = Some(country.to_owned());
        self
    }

    #[doc = include_str!("../docs/locale.md")]
    pub fn locale(mut self, locale: &str) -> Self {
        self.endpoint.locale = Some(locale.to_owned());
        self
    }

    #[doc = include_str!("../docs/send.md")]
    pub async fn get(self) -> Result<Category> {
        self.spotify
            .get(
                format!("/browse/categories/{}", self.endpoint.id),
                self.endpoint,
            )
            .await
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct BrowseCategoriesEndpoint {
    pub(crate) country: Option<String>,
    pub(crate) locale: Option<String>,
    pub(crate) limit: Option<Limit>,
    pub(crate) offset: Option<u32>,
}

impl<F: AuthFlow> Builder<'_, F, BrowseCategoriesEndpoint> {
    #[doc = include_str!("../docs/country.md")]
    pub fn country(mut self, country: &str) -> Self {
        self.endpoint.country = Some(country.to_owned());
        self
    }

    #[doc = include_str!("../docs/locale.md")]
    pub fn locale(mut self, locale: &str) -> Self {
        self.endpoint.locale = Some(locale.to_owned());
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

    #[doc = include_str!("../docs/send.md")]
    pub async fn get(self) -> Result<Page<Category>> {
        self.spotify
            .get("/browse/categories".to_owned(), self.endpoint)
            .await
            .map(|c: Categories| c.categories)
    }
}
