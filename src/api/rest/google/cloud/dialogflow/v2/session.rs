use serde::Deserialize;
use serde_json::Value as JsonData;

/// Draft of DetectIntentResponse. Full specification here:
/// https://cloud.google.com/dialogflow/es/docs/reference/rest/v2/DetectIntentResponse
/// Currently QueryResult is not fully mapped into structs. We map only what we need.
#[derive(Debug, Deserialize)]
pub struct DetectIntentResponse {
    #[serde(rename = "responseId")]
    pub response_id: String,

    #[serde(rename = "queryResult", skip_serializing_if = "Option::is_none")]
    pub query_result: Option<QueryResult>,

    #[serde(rename = "webhookStatus", skip_serializing_if = "Option::is_none")]
    pub webhook_status: Option<Status>,

    #[serde(rename = "outputAudio", skip_serializing_if = "Option::is_none")]
    pub output_audio: Option<String>,

    #[serde(rename = "outputAudioConfig", skip_serializing_if = "Option::is_none")]
    pub output_audio_config: Option<OutputAudioConfig>,
}

#[derive(Debug, Deserialize)]
pub struct QueryResult {
    #[serde(rename = "diagnosticInfo")]
    pub diagnostic_info: Option<DiagnosticInfo>,
}

#[derive(Debug, Deserialize)]
pub struct DiagnosticInfo {
    pub end_conversation: bool,
}

#[derive(Debug, Deserialize)]
pub struct OutputAudioConfig {
    #[serde(rename = "audioEncoding")]
    pub audio_encoding: i32,

    #[serde(rename = "sampleRateHertz")]
    pub sample_rate_hertz: i32,

    #[serde(
        rename = "synthesizeSpeechConfig",
        skip_serializing_if = "Option::is_none"
    )]
    pub synthesize_speech_config: Option<SynthesizeSpeechConfig>,
}

#[derive(Debug, Deserialize)]
pub struct Status {
    code: i32,
    message: String,
    details: Vec<JsonData>,
}

#[derive(Debug, Deserialize)]
pub struct SynthesizeSpeechConfig {
    #[serde(rename = "speakingRate")]
    pub speaking_rate: f64,

    pub pitch: f64,

    #[serde(rename = "volumeGainDb")]
    pub volume_gain_db: f64,

    #[serde(rename = "effectsProfileId")]
    pub effects_profile_id: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<VoiceSelectionParams>,
}

#[derive(Debug, Deserialize)]
pub struct VoiceSelectionParams {
    name: String,

    #[serde(rename = "ssmlGender")]
    ssml_gender: i32,
}

// TBD: use this for VoiceSelectionParams.ssml_gender
// need to find real i32 values, google doc does not specify them
#[derive(Debug, Deserialize)]
#[allow(non_camel_case_types)]
pub enum SsmlVoiceGender {
    SSML_VOICE_GENDER_UNSPECIFIED = 0,
    SSML_VOICE_GENDER_MALE = 1,
    SSML_VOICE_GENDER_FEMALE = 2,
    SSML_VOICE_GENDER_NEUTRAL = 3,
}
