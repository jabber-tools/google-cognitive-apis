use crate::api::grpc::google::cloud::texttospeech::v1::{
    AudioConfig as GrpcAudioConfig, VoiceSelectionParams as GrpcVoiceSelectionParams,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct VoiceSelectionParams {
    #[serde(rename = "languageCode")]
    pub language_code: String,

    pub name: String,

    #[serde(rename = "ssmlGender")]
    pub ssml_gender: i32,
}
impl From<GrpcVoiceSelectionParams> for VoiceSelectionParams {
    fn from(grpc_vsp: GrpcVoiceSelectionParams) -> Self {
        VoiceSelectionParams {
            language_code: grpc_vsp.language_code,
            name: grpc_vsp.name,
            ssml_gender: grpc_vsp.ssml_gender,
        }
    }
}

// TBD: use this for VoiceSelectionParams.ssml_gender
// need to find real i32 values, google doc does not specify them
#[derive(Debug, Deserialize)]
#[allow(non_camel_case_types)]
pub enum SsmlVoiceGender {
    SSML_VOICE_GENDER_UNSPECIFIED = 0,
    MALE = 1,
    FEMALE = 2,
    NEUTRAL = 3,
}

#[derive(Debug, Deserialize)]
pub struct AudioConfig {
    #[serde(rename = "audioEncoding")]
    pub audio_encoding: i32,

    #[serde(rename = "speakingRate")]
    pub speaking_rate: f64,

    pub pitch: f64,

    #[serde(rename = "volumeGainDb")]
    pub volume_gain_db: f64,

    #[serde(rename = "sampleRateHertz")]
    pub sample_rate_hertz: i32,

    #[serde(rename = "effectsProfileId")]
    pub effects_profile_id: Vec<String>,
}

impl From<GrpcAudioConfig> for AudioConfig {
    fn from(grpc_audio_cfg: GrpcAudioConfig) -> Self {
        AudioConfig {
            audio_encoding: grpc_audio_cfg.audio_encoding,
            speaking_rate: grpc_audio_cfg.speaking_rate,
            pitch: grpc_audio_cfg.pitch,
            volume_gain_db: grpc_audio_cfg.volume_gain_db,
            sample_rate_hertz: grpc_audio_cfg.sample_rate_hertz,
            effects_profile_id: grpc_audio_cfg.effects_profile_id,
        }
    }
}
