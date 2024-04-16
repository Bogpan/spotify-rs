use serde::{Deserialize, de::DeserializeOwned};

use crate::{client::Client, auth::{Token, AuthFlow, Verifier}, Error};

pub mod album;
pub mod artist;
pub mod audio;
pub mod audiobook;
pub mod category;
pub mod market;
pub mod player;
pub mod playlist;
pub mod recommendation;
pub mod search;
pub mod show;
pub mod track;
pub mod user;

#[derive(Clone, Debug, Deserialize)]
pub struct Page<T> {
    pub href: String,
    pub limit: u32,
    pub next: Option<String>,
    pub offset: u32,
    pub previous: Option<String>,
    pub total: u32,
    pub items: Vec<T>,
}

impl<T: DeserializeOwned> Page<T> {
    pub async fn get_next<F: AuthFlow, V: Verifier> (&self, client: &mut Client<Token, F, V>) -> Result<Page<T>, Error> {
        client.get::<(), _>(self.next.as_ref().unwrap().clone(), None).await
    }

    pub async fn get_previous<F: AuthFlow, V: Verifier> (&self, client: &mut Client<Token, F, V>) -> Result<Page<T>, Error> {
        client.get::<(), _>(self.previous.as_ref().unwrap().clone(), None).await
    }

    pub async fn fetch_all<F: AuthFlow, V: Verifier> (self, client: &mut Client<Token, F, V>) -> Result<Vec<T>, Error> {
        
        let mut result = self.items;
        let mut current_page = Page{ items: Vec::new(), ..self };

        loop {
            if current_page.next.is_none() { break; }
            let mut next_page = current_page.get_next(client).await?;
            result.append(&mut next_page.items);
            current_page = next_page;
        }

        Ok(result)
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct CursorPage<T> {
    pub href: String,
    pub limit: u32,
    pub next: Option<String>,
    pub cursors: Cursor,
    pub total: Option<u32>,
    pub items: Vec<T>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Cursor {
    pub after: Option<String>,
    pub before: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Image {
    pub url: String,
    pub height: Option<u32>,
    pub width: Option<u32>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Copyright {
    pub text: String,
    pub r#type: CopyrightType,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Restrictions {
    pub reason: RestrictionReason,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ExternalIds {
    pub isrc: Option<String>,
    pub ean: Option<String>,
    pub upc: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ExternalUrls {
    pub spotify: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Followers {
    /// This will always be set to null, as the Web API does not support it at the moment.
    pub href: Option<String>,
    pub total: u32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ResumePoint {
    pub fully_played: bool,
    pub resume_position_ms: u32,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestrictionReason {
    Market,
    Product,
    Explicit,
    #[serde(other)]
    Unknown,
}

#[derive(Clone, Debug, Deserialize)]
pub enum CopyrightType {
    #[serde(rename = "C")]
    Copyright,
    #[serde(rename = "P")]
    Performance,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DatePrecision {
    Year,
    Month,
    Day,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum PlayableItem {
    Track(track::Track),
    Episode(show::Episode),
}
