use crate::{auth::AuthFlow, error::Result, model::market::Markets};

use super::Client;

pub async fn get_available_markets(spotify: &Client<impl AuthFlow>) -> Result<Vec<String>> {
    spotify
        .get::<(), _>("/markets".to_owned(), None)
        .await
        .map(|m: Markets| m.markets)
}
