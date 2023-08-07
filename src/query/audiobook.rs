use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AudiobookQuery {
    #[serde(skip)]
    pub(crate) audiobook_id: String,
    market: Option<String>,
}

impl AudiobookQuery {
    pub fn new(audiobook_id: &str) -> Self {
        Self {
            audiobook_id: audiobook_id.to_owned(),
            market: None,
        }
    }

    pub fn market(mut self, market: &str) -> Self {
        self.market = Some(market.to_owned());
        self
    }
}

#[derive(Debug, Serialize)]
pub struct AudiobooksQuery {
    #[serde(rename = "ids")]
    audiobook_ids: String,
    market: Option<String>,
}

impl AudiobooksQuery {
    pub fn new(audiobook_ids: &[&str]) -> Self {
        Self {
            audiobook_ids: audiobook_ids.join(","),
            market: None,
        }
    }

    pub fn market(mut self, market: &str) -> Self {
        self.market = Some(market.to_owned());
        self
    }
}

#[derive(Debug, Serialize)]
pub struct AudiobookChaptersQuery {
    #[serde(skip)]
    pub(crate) audiobook_id: String,
    market: Option<String>,
    limit: Option<u32>,
    offset: Option<u32>,
}

impl AudiobookChaptersQuery {
    pub fn new(audiobook_id: &str) -> Self {
        Self {
            audiobook_id: audiobook_id.to_owned(),
            market: None,
            limit: None,
            offset: None,
        }
    }

    pub fn market(mut self, market: &str) -> Self {
        self.market = Some(market.to_owned());
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

#[derive(Debug, Default, Serialize)]
pub struct SavedAudiobooksQuery {
    limit: Option<u32>,
    offset: Option<u32>,
}

impl SavedAudiobooksQuery {
    pub fn new() -> Self {
        Self::default()
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

#[derive(Debug, Default, Serialize)]
pub struct ChapterQuery {
    #[serde(skip)]
    pub(crate) chapter_id: String,
    market: Option<String>,
}

impl ChapterQuery {
    pub fn new(chapter_id: &str) -> Self {
        Self {
            chapter_id: chapter_id.to_owned(),
            market: None,
        }
    }

    pub fn market(mut self, market: &str) -> Self {
        self.market = Some(market.to_owned());
        self
    }
}

#[derive(Debug, Default, Serialize)]
pub struct ChaptersQuery {
    #[serde(rename = "ids")]
    pub(crate) chapter_ids: String,
    market: Option<String>,
}

impl ChaptersQuery {
    pub fn new(chapter_ids: &[&str]) -> Self {
        Self {
            chapter_ids: chapter_ids.join(","),
            market: None,
        }
    }

    pub fn market(mut self, market: &str) -> Self {
        self.market = Some(market.to_owned());
        self
    }
}
