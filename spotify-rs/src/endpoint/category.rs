use serde::Serialize;

use crate::{
    auth::AuthFlow,
    error::Result,
    model::{
        category::{Categories, Category},
        Page,
    },
};

use super::{Client, Endpoint};

impl Endpoint for BrowseCategoryEndpoint {}
impl Endpoint for BrowseCategoriesEndpoint {}

pub fn browse_category(id: impl Into<String>) -> BrowseCategoryEndpoint {
    BrowseCategoryEndpoint {
        id: id.into(),
        ..Default::default()
    }
}

pub fn browse_categories() -> BrowseCategoriesEndpoint {
    BrowseCategoriesEndpoint::default()
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct BrowseCategoryEndpoint {
    #[serde(skip)]
    pub(crate) id: String,
    pub(crate) country: Option<String>,
    pub(crate) locale: Option<String>,
}

impl BrowseCategoryEndpoint {
    #[doc = include_str!("../docs/country.md")]
    pub fn country(mut self, country: impl Into<String>) -> Self {
        self.country = Some(country.into());
        self
    }

    #[doc = include_str!("../docs/locale.md")]
    pub fn locale(mut self, locale: impl Into<String>) -> Self {
        self.locale = Some(locale.into());
        self
    }

    #[doc = include_str!("../docs/send.md")]
    pub async fn get(self, spotify: &Client<impl AuthFlow>) -> Result<Category> {
        spotify
            .get(format!("/browse/categories/{}", self.id), self)
            .await
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct BrowseCategoriesEndpoint {
    pub(crate) country: Option<String>,
    pub(crate) locale: Option<String>,
    pub(crate) limit: Option<u32>,
    pub(crate) offset: Option<u32>,
}

impl BrowseCategoriesEndpoint {
    #[doc = include_str!("../docs/country.md")]
    pub fn country(mut self, country: impl Into<String>) -> Self {
        self.country = Some(country.into());
        self
    }

    #[doc = include_str!("../docs/locale.md")]
    pub fn locale(mut self, locale: impl Into<String>) -> Self {
        self.locale = Some(locale.into());
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

    #[doc = include_str!("../docs/send.md")]
    pub async fn get(self, spotify: &Client<impl AuthFlow>) -> Result<Page<Category>> {
        spotify
            .get("/browse/categories".to_owned(), self)
            .await
            .map(|c: Categories| c.categories)
    }
}
