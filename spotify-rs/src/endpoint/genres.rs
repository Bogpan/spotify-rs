use crate::{auth::AuthFlow, error::Result, model::recommendation::Genres};

use super::Client;

pub async fn get_genre_seeds(spotify: &Client<impl AuthFlow>) -> Result<Vec<String>> {
    spotify
        .get::<(), _>("/recommendations/available-genre-seeds".to_owned(), None)
        .await
        .map(|g: Genres| g.genres)
}
