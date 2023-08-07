use serde::Deserialize;

use super::{Image, Page};

#[derive(Clone, Debug, Deserialize)]
pub struct Category {
    pub href: String,
    pub icons: Vec<Image>,
    pub id: String,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct Categories {
    pub(crate) categories: Page<Category>,
}
