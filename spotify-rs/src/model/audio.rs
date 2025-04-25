use serde::Deserialize;
use serde_repr::Deserialize_repr;

/// Audio features for a track.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct AudioFeatures {
    /// A measure of confidence from `0.0` to `1.0` indicating whether the track is acoustic.
    /// A score of `1.0` implies high certainty that the track is acoustic.
    pub acousticness: f32,
    /// A URL to access the full audio analysis of the track.
    pub analysis_url: String,
    /// A measure between `0.0` - `1.0` that describes how suitable a track is for
    /// dancing based on various musical factors like tempo, rhythm stability,
    /// beat strength, and overall regularity.
    pub danceability: f32,
    /// The duration of the track in miliseconds.
    pub duration_ms: u32,
    /// A score between `0.0` - `1.0` that represents the intensity and activity
    /// level of a track.
    ///
    /// Energetic tracks feel fast, loud, and lively. For instance, death metal
    /// scores high in energy, while a Bach prelude scores low.
    ///
    /// Features contributing to this include dynamic range,
    /// perceived loudness, timbre, onset rate, and overall entropy.
    pub energy: f32,
    /// The [Spotify ID](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the track.
    pub id: String,
    /// A value between `0.0` - `1.0` that estimates the likelihood of a track being
    /// instrumental (no vocals).
    ///
    /// 'Ooh' and 'aah' sounds are considered instrumental. Rap or spoken word
    /// tracks are considered vocal.
    ///
    /// Values above `0.5` suggest instrumental tracks, but confidence is higher
    /// as the value nears `1.0`.
    pub instrumentalness: f32,
    /// A value ranging between `-1` - `11` that denotes musical key of the track,
    /// represented by integers mapping to pitches using standard
    /// [Pitch Class notation] (https://en.wikipedia.org/wiki/Pitch_class).
    ///
    /// If no key is detected, the value is `-1`.
    pub key: i32,
    /// A value ranging between `0.0` and `1.0` that measures the likelihood
    /// of the presence of an audience in the recording.
    ///
    /// A value above `0.8` strongly suggests that the track is live.
    pub liveness: f32,
    /// The average loudness of the track in decibels (dB). Loudness values
    /// are averaged across the entire track and are useful for comparing
    /// relative loudness of tracks.
    ///
    /// Values typically range between `-60` and `0` dB.
    pub loudness: f32,
    /// Indicates the modality (major or minor) of the track.
    pub mode: Mode,
    /// A value between `0.0` - `1.0` that detects the presence of spoken words in
    /// a track. A value closer to 1.0 indicates more speech-like content.
    ///
    /// Values below `0.33` most likely represent music.
    ///
    /// Values between `0.33` - `0.66` describe tracks that may contain both music and speech,
    /// either in sections or layered.
    ///
    /// Values above `0.66` most likely represent tracks that consist entirely of
    /// spoken words, like podcasts, audiobooks etc.
    pub speechiness: f32,
    /// The estimated pace of the track in beats per minute (BPM).
    pub tempo: f32,
    /// An estimated notation of how many beats are in each measure.
    ///
    /// Values range between `3` - `7`, indicating time signatures ranging
    /// between `3/4` - `7/4`.
    pub time_signature: u32,
    /// A link to the Spotify Web API endpoint providing full details of the track.
    pub track_href: String,
    /// The object type. Allowed values: `audio_features`.
    pub r#type: String,
    /// The [Spotify URI](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids)
    /// for the track.
    pub uri: String,
    /// A measure from `0.0` to `1.0` describing the musical positiveness
    /// conveyed by a track.
    ///
    /// High valence tracks sound more positive (e.g. happy, cheerful), while
    /// low valence tracks sound more negative (e.g. sad, depressed, angry).
    pub valence: f32,
}

// Used only to deserialize JSON responses with arrays that are named objects.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub(crate) struct AudioFeaturesList {
    pub(crate) audio_features: Vec<Option<AudioFeatures>>,
}

/// Audio analysis for a track.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct AudioAnalysis {
    pub meta: Meta,
    pub track: TrackAnalysis,
    /// The time intervals of the measures throughout the track. A measure
    /// (or bar) is a segment of time defined by a specific number of beats.
    pub bars: Vec<Bar>,
    /// The timing intervals of beats throughout the track. A beat is the
    /// fundamental time unit of a piece of music; for instance, each tick
    /// of a metronome. Beats are usually multiples of tatums.
    pub beats: Vec<Beat>,
    /// Sections are defined by significant changes in rhythm or timbre,
    /// such as the chorus, verse, bridge, guitar solo, etc.
    ///
    /// Each section has its own descriptions of tempo, key, mode,
    /// time signature, and loudness.
    pub sections: Vec<Section>,
    /// Each segment contains a relatively consistent sound throughout its duration.
    pub segments: Vec<Segment>,
    /// A tatum represents the smallest regular pulse that a listener naturally
    /// infers from the timing of perceived musical events (segments).
    pub tatums: Vec<Tatum>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Meta {
    /// The version of the analyser used to analyse the track.
    pub analyzer_version: String,
    /// The platform used to read the track's audio data (e.g. Linux).
    pub platform: String,
    /// A status code for the track. If analysis data is missing,
    /// the status code may explain why.
    pub detailed_status: String,
    /// The return code of the analyser process.
    pub status_code: AnalysisStatusCode,
    /// The Unix timestamp at which the track was analysed.
    pub timestamp: u64,
    /// The amount of time taken to analyse the track.
    pub analysis_time: f32,
    /// The method used to read the track's audio data.
    pub input_process: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct TrackAnalysis {
    /// The number of audio samples from the track that were analysed.
    pub num_samples: u32,
    /// The length of the track in seconds.
    pub duration: f32,
    /// This field will always contain an empty string.
    pub sample_md5: String,
    /// An offset to the start of the segment of the track that awas analysed.
    /// As the entire track is analysed, this should be `0`.
    pub offset_seconds: u32,
    /// The length of the segment of the track that was analysed, if only a subset
    /// was. As the entire track is analysed, this should be `0`.
    pub window_seconds: u32,
    /// The sample rate used to decode and analyse the track. May vary from the
    /// track's sample rate on Spotify.
    pub analysis_sample_rate: u32,
    /// The number of channels used in the analysis. If the value is `1`, all
    /// the channels were combined to mono before analysis.
    pub analysis_channels: u32,
    /// The time, in seconds, when the track's fade-in ends. If the track has no
    /// fade-in, the value is `0`.
    pub end_of_fade_in: f32,
    /// The time, in seconds, when the track's fade-out begins. If the track has
    /// no fade-out, this will match the track's length.
    pub start_of_fade_out: f32,
    /// The average loudness of the track in decibels (dB). Loudness values
    /// are averaged across the entire track and are useful for comparing
    /// relative loudness of tracks.
    ///
    /// Values typically range between `-60` and `0` dB.
    pub loudness: f32,
    /// The estimated pace of the track in beats per minute (BPM).
    pub tempo: f32,
    /// A value ranging between `0.0` - `1.0` that indicates the confidence
    /// of the `tempo`.
    pub tempo_confidence: f32,
    /// An estimated notation of how many beats are in each measure.
    ///
    /// Values range between `3` - `7`, indicating time signatures ranging
    /// between `3/4` - `7/4`.
    pub time_signature: u32,
    /// A value ranging between `0.0` - `1.0` that indicates the confidence
    /// of the `time_signature`.
    pub time_signature_confidence: f32,
    /// A value ranging between `-1` - `11` that denotes musical key of the track,
    /// represented by integers mapping to pitches using standard
    /// [Pitch Class notation] (https://en.wikipedia.org/wiki/Pitch_class).
    ///
    /// If no key is detected, the value is `-1`.
    pub key: i32,
    /// A value ranging between `0.0` - `1.0` that indicates the confidence
    /// of the `key`.
    pub key_confidence: f32,
    /// Indicates the modality (major or minor) of a track.
    pub mode: Mode,
    /// A value ranging between `0.0` - `1.0` that indicates the confidence
    /// of the `mode`.
    pub mode_confidence: f32,
    /// An [ENMPF](https://academiccommons.columbia.edu/doi/10.7916/D8Q248M4)
    /// codestring for the track.
    pub codestring: String,
    /// The version for the ENMPF used in the `codestring`.
    pub code_version: f32,
    /// An [EchoPrint](https://github.com/spotify/echoprint-codegen) codestring
    /// for the track.
    #[serde(rename = "echoprintstring")]
    pub echoprint_string: String,
    /// The version for the EchoPrint format used in the `echoprintstring`.
    pub echoprint_version: f32,
    /// A [SynchString](https://github.com/echonest/synchdata) for the track.
    pub synchstring: String,
    /// The version for the Synchstring used in the `synchstring`.
    pub synch_version: f32,
    /// A Rhythmstring for the track. Its format is similar to that of the
    /// `synchstring`.
    pub rhythmstring: String,
    /// The version for the Rhythmstring used in the `rhythmstring`.
    pub rhythm_version: f32,
}

/// A measure (or bar) is a segment of time defined by a specific number of beats.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Bar {
    /// The starting point, in seconds, of the time interval.
    pub start: f32,
    /// The duration, in seconds, of the time interval.
    pub duration: f32,
    /// A value ranging betweeen `0.0` - `1.0` that indicates the confidence of
    /// the interval.
    pub confidence: f32,
}

/// A beat is the fundamental time unit of a piece of music; for instance,
/// each tick of a metronome. Beats are usually multiples of tatums.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Beat {
    /// The starting point, in seconds, of the time interval.
    pub start: f32,
    /// The duration, in seconds, of the time interval.
    pub duration: f32,
    /// A value ranging betweeen `0.0` - `1.0` that indicates the confidence of
    /// the interval.
    pub confidence: f32,
}

/// A section is defined by significant changes in rhythm or timbre, such as
/// the chorus, verse, bridge, guitar solo, etc.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Section {
    /// The starting point, in seconds, of the section.
    pub start: f32,
    /// The duration, in seconds, of the section.
    pub duration: f32,
    /// A value ranging betweeen `0.0` - `1.0` that indicates the confidence of
    /// the section's "designation".
    pub confidence: f32,
    /// The average loudness of the section in decibels (dB). Loudness values
    /// are useful for comparing relative loudness of sections.
    pub loudness: f32,
    /// The estimated pace of the section in beats per minute (BPM).
    pub tempo: f32,
    /// A value ranging betweeen `0.0` - `1.0` that indicates the confidence of
    /// the tempo.
    pub tempo_confidence: f32,
    /// A value ranging between `-1` - `11` that denotes musical key of the section,
    /// represented by integers mapping to pitches using standard
    /// [Pitch Class notation] (https://en.wikipedia.org/wiki/Pitch_class).
    ///
    /// If no key is detected, the value is `-1`.
    pub key: i32,
    /// A value ranging betweeen `0.0` - `1.0` that indicates the confidence of
    /// the key.
    pub key_confidence: f32,
    /// Indicates the modality (major or minor) of the section.
    pub mode: Mode,
    /// A value ranging betweeen `0.0` - `1.0` that indicates the confidence of
    /// the mode.
    pub mode_confidence: f32,
    /// An estimated notation of how many beats are in each measure.
    ///
    /// Values range between `3` - `7`, indicating time signatures ranging
    /// between `3/4` - `7/4`.
    pub time_signature: u32,
    /// A value ranging betweeen `0.0` - `1.0` that indicates the confidence of
    /// the time signature.
    pub time_signature_confidence: f32,
}

/// A segment contains a relatively consistent sound throughout its duration.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Segment {
    /// The starting point, in seconds, of the segment.
    pub start: f32,
    /// The duration, in seconds, of the segment.
    pub duration: f32,
    /// A value ranging betweeen `0.0` - `1.0` that indicates the confidence of
    /// the segment.
    pub confidence: f32,
    pub loudness_start: f32,
    pub loudness_max: f32,
    pub loudness_max_time: f32,
    pub loudness_end: f32,
    pub pitches: Vec<f32>,
    pub timbre: Vec<f32>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Tatum {
    pub start: f32,
    pub duration: f32,
    pub confidence: f32,
}

#[derive(Clone, Copy, Debug, Deserialize_repr, PartialEq)]
#[repr(u8)]
pub enum Mode {
    Minor,
    Major,
}

#[derive(Clone, Copy, Debug, Deserialize_repr, PartialEq)]
#[repr(u8)]
pub enum AnalysisStatusCode {
    Success,
    Failure,
}
