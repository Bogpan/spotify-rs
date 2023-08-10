use serde::Serialize;
use serde_json::Value;

#[derive(Clone, Debug, Default, Serialize)]
pub struct PlaybackStateQuery {
    market: Option<String>,
    additional_types: Option<String>,
}

impl PlaybackStateQuery {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn market(mut self, market: &str) -> Self {
        self.market = Some(market.to_owned());
        self
    }
    /// A comma-separated list of item types that your client supports besides the default track type. Valid types are: track and episode.
    /// In addition to providing this parameter, make sure that your client properly handles cases of new types in the future by checking against the type field of each object.
    /// Note: This parameter was introduced to allow existing clients to maintain their current behaviour and might be deprecated in the future.
    pub fn additional_types<T: AsRef<str>>(mut self, additional_types: &[T]) -> Self {
        self.additional_types = Some(
            additional_types
                .iter()
                .map(|i| i.as_ref())
                .collect::<Vec<&str>>()
                .join(","),
        );
        self
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct TransferPlaybackQuery {
    pub(crate) device_ids: [String; 1],
    pub(crate) play: Option<bool>,
}

impl TransferPlaybackQuery {
    /// A JSON array containing the ID of the device on which playback should be started/transferred.
    /// Note: Although an array is accepted, only a single device_id is currently supported. Supplying more than one will return 400 Bad Request
    pub fn new(device_id: &str) -> Self {
        Self {
            device_ids: [device_id.to_owned()],
            play: None,
        }
    }

    pub fn play(mut self, play: bool) -> Self {
        self.play = Some(play);
        self
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct CurrentlyPlayingQuery {
    market: Option<String>,
    additional_types: Option<String>,
}

impl CurrentlyPlayingQuery {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn market(mut self, market: &str) -> Self {
        self.market = Some(market.to_owned());
        self
    }
    /// A comma-separated list of item types that your client supports besides the default track type. Valid types are: track and episode.
    /// In addition to providing this parameter, make sure that your client properly handles cases of new types in the future by checking against the type field of each object.
    /// Note: This parameter was introduced to allow existing clients to maintain their current behaviour and might be deprecated in the future.
    pub fn additional_types<T: AsRef<str>>(mut self, additional_types: &[T]) -> Self {
        self.additional_types = Some(
            additional_types
                .iter()
                .map(|i| i.as_ref())
                .collect::<Vec<&str>>()
                .join(","),
        );
        self
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct TogglePlaybackQuery {
    #[serde(skip)]
    pub(crate) device_id: Option<String>,
    context_uri: Option<String>,
    uris: Option<Vec<String>>,
    offset: Option<u32>,
    position_ms: Option<u32>,
}

impl TogglePlaybackQuery {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn device_id(mut self, device_id: &str) -> Self {
        self.device_id = Some(device_id.to_owned());
        self
    }

    pub fn context_uri(mut self, context_uri: &str) -> Self {
        self.context_uri = Some(context_uri.to_owned());
        self
    }

    pub fn uris<T: AsRef<str>>(mut self, uris: &[T]) -> Self {
        self.uris = Some(uris.iter().map(|i| i.as_ref().to_owned()).collect());
        self
    }

    pub fn offset(mut self, offset: u32) -> Self {
        // TODO Error when is not album or playlist
        self.offset = Some(offset);
        self
    }

    pub fn position_ms(mut self, position: u32) -> Self {
        self.position_ms = Some(position);
        self
    }
}
