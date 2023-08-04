use serde::Deserialize;

use super::Image;

#[derive(Clone, Debug, Deserialize)]
pub struct Category {
    pub href: String,
    pub icons: Vec<Image>,
    pub id: String,
    pub name: String,
}
