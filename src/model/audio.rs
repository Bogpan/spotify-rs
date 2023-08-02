use serde::{Deserialize, Serialize};
use serde_repr::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AudioFeatures {
    pub acousticness: f32,
    pub analysis_url: String,
    pub danceability: f32,
    pub duration_ms: u32,
    pub energy: f32,
    pub id: String,
    pub instrumentalness: f32,
    /// The key the track is in. Integers map to pitches using standard Pitch Class notation. E.g. 0 = C, 1 = C♯/D♭, 2 = D, and so on. If no key was detected, the value is -1.
    pub key: i32,
    pub liveness: f32,
    pub loudness: f32,
    // Mode indicates the modality (major or minor) of a track, the type of scale from which its melodic content is derived. Major is represented by 1 and minor is 0.
    pub mode: Mode,
    pub speechiness: f32,
    pub tempo: f32,
    pub time_signature: u32,
    pub track_href: String,
    pub r#type: String,
    pub uri: String,
    pub valence: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AudioAnalysis {
    pub meta: Meta,
    pub track: TrackAnalysis,
    pub bars: Vec<Bar>,
    pub beats: Vec<Beat>,
    pub sections: Vec<Section>,
    pub segments: Vec<Segment>,
    pub tatums: Vec<Tatum>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Meta {
    pub analyzer_version: String,
    pub platform: String,
    pub detailed_status: String,
    /// The return code of the analyzer process. 0 if successful, 1 if any errors occurred.
    pub status_code: u32,
    pub timestamp: u64,
    pub analysis_time: f32,
    pub input_process: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TrackAnalysis {
    pub num_samples: u32,
    pub duration: f32,
    /// This field will always contain an empty string.
    pub sample_md5: String,
    pub offset_seconds: u32,
    pub window_seconds: u32,
    pub analysis_sample_rate: u32,
    pub analysis_channels: u32,
    pub end_of_fade_in: f32,
    pub start_of_fade_out: f32,
    pub loudness: f32,
    pub tempo: f32,
    pub tempo_confidence: f32,
    pub time_signature: u32,
    pub time_signature_confidence: f32,
    pub key: i32,
    pub key_confidence: f32,
    pub mode: Mode,
    pub mode_confidence: f32,
    pub codestring: String,
    pub code_version: f32,
    pub echoprintstring: String,
    pub echoprint_version: f32,
    pub synchstring: String,
    pub synch_version: f32,
    pub rhythmstring: String,
    pub rhythm_version: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Bar {
    pub start: f32,
    pub duration: f32,
    pub confidence: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Beat {
    pub start: f32,
    pub duration: f32,
    pub confidence: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Section {
    pub start: f32,
    pub duration: f32,
    pub confidence: f32,
    pub loudness: f32,
    pub tempo: f32,
    pub tempo_confidence: f32,
    pub key: i32,
    pub key_confidence: f32,
    pub mode: Mode,
    pub mode_confidence: f32,
    pub time_signature: u32,
    pub time_signature_confidence: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Segment {
    pub start: f32,
    pub duration: f32,
    pub confidence: f32,
    pub loudness_start: f32,
    pub loudness_max: f32,
    pub loudness_max_time: f32,
    pub loudness_end: f32,
    pub pitches: Vec<f32>,
    pub timbre: Vec<f32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tatum {
    pub start: f32,
    pub duration: f32,
    pub confidence: f32,
}

#[derive(Clone, Copy, Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum Mode {
    Minor,
    Major,
}
