use serde::Deserialize;
use spotify_rs_macros::docs;

use super::{Image, Page};

/// A browse category.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[docs]
pub struct Category {
    pub href: String,
    /// The icon of the category, in various sizes.
    pub icons: Vec<Image>,
    pub id: String,
    pub name: String,
}

// Used only to deserialize JSON responses with arrays that are named objects.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub(crate) struct Categories {
    pub(crate) categories: Page<Category>,
}
