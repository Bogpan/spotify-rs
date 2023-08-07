use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct CategoryQuery {
    #[serde(skip)]
    pub(crate) category_id: String,
    country: Option<String>,
    locale: Option<String>,
}

impl CategoryQuery {
    pub fn new(category_id: &str) -> Self {
        Self {
            category_id: category_id.to_owned(),
            country: None,
            locale: None,
        }
    }

    pub fn country(mut self, country: &str) -> Self {
        self.country = Some(country.to_owned());
        self
    }

    pub fn locale(mut self, locale: &str) -> Self {
        self.locale = Some(locale.to_owned());
        self
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct CategoriesQuery {
    country: Option<String>,
    locale: Option<String>,
    limit: Option<u32>,
    offset: Option<u32>,
}

impl CategoriesQuery {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn country(mut self, country: &str) -> Self {
        self.country = Some(country.to_owned());
        self
    }

    pub fn locale(mut self, locale: &str) -> Self {
        self.locale = Some(locale.to_owned());
        self
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }
}
