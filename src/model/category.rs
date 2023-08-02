use serde::{Deserialize, Serialize};

use super::Image;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Category {
    pub href: String,
    pub icons: Vec<Image>,
    pub id: String,
    pub name: String,
}
