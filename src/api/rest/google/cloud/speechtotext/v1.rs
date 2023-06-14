#![allow(clippy::from_over_into)]
#![allow(clippy::manual_unwrap_or)]
#![allow(clippy::manual_map)]
use crate::api::grpc::google::cloud::speechtotext::v1::RecognitionConfig as GrpcRecognitionConfig;
use crate::api::grpc::google::cloud::speechtotext::v1::RecognitionMetadata as GrpcRecognitionMetadata;
use crate::api::grpc::google::cloud::speechtotext::v1::SpeakerDiarizationConfig as GrpcSpeakerDiarizationConfig;
use crate::api::grpc::google::cloud::speechtotext::v1::SpeechContext as GrpcSpeechContext;
use crate::errors::{Error, Result};
///
/// https://cloud.google.com/speech-to-text/docs/reference/rest/v1/RecognitionConfig
///
use serde::{Deserialize, Serialize};
use std::convert::Into;
use std::fmt;

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AudioEncoding {
    ENCODING_UNSPECIFIED,
    LINEAR16,
    FLAC,
    MULAW,
    AMR,
    AMR_WB,
    OGG_OPUS,
    SPEEX_WITH_HEADER_BYTE,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SpeechContext {
    phrases: Vec<String>,
}

impl Into<GrpcSpeechContext> for SpeechContext {
    fn into(self) -> GrpcSpeechContext {
        GrpcSpeechContext {
            phrases: self.phrases,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SpeakerDiarizationConfig {
    #[serde(rename = "enableSpeakerDiarization")]
    enable_speaker_diarization: bool,

    #[serde(rename = "minSpeakerCount", default = "default_min_speaker_count")]
    min_speaker_count: i32,

    #[serde(rename = "maxSpeakerCount", default = "default_max_speaker_count")]
    max_speaker_count: i32,

    #[serde(rename = "speakerTag", skip_serializing_if = "Option::is_none")]
    speaker_tag: Option<i32>,
}

#[allow(deprecated)]
impl Into<GrpcSpeakerDiarizationConfig> for SpeakerDiarizationConfig {
    fn into(self) -> GrpcSpeakerDiarizationConfig {
        GrpcSpeakerDiarizationConfig {
            enable_speaker_diarization: self.enable_speaker_diarization,
            min_speaker_count: self.min_speaker_count,
            max_speaker_count: self.max_speaker_count,
            speaker_tag: 0, // REST interface will never provide this
        }
    }
}

fn default_min_speaker_count() -> i32 {
    2
}

fn default_max_speaker_count() -> i32 {
    6
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RecognitionConfigModel {
    command_and_search,
    phone_call,
    video,
    default,
    latest_long,
    latest_short,
    medical_conversation,
    medical_dictation
}

impl fmt::Display for RecognitionConfigModel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum InteractionType {
    INTERACTION_TYPE_UNSPECIFIED,
    DISCUSSION,
    PRESENTATION,
    PHONE_CALL,
    VOICEMAIL,
    PROFESSIONALLY_PRODUCED,
    VOICE_SEARCH,
    VOICE_COMMAND,
    DICTATION,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MicrophoneDistance {
    MICROPHONE_DISTANCE_UNSPECIFIED,
    NEARFIELD,
    MIDFIELD,
    FARFIELD,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum OriginalMediaType {
    ORIGINAL_MEDIA_TYPE_UNSPECIFIED,
    AUDIO,
    VIDEO,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RecordingDeviceType {
    RECORDING_DEVICE_TYPE_UNSPECIFIED,
    SMARTPHONE,
    PC,
    PHONE_LINE,
    VEHICLE,
    OTHER_OUTDOOR_DEVICE,
    OTHER_INDOOR_DEVICE,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RecognitionMetadata {
    #[serde(rename = "interactionType")]
    pub interaction_type: InteractionType,

    #[serde(rename = "industryNaicsCodeOfAudio")]
    pub industry_naics_code_of_audio: u32,

    #[serde(rename = "microphoneDistance")]
    pub microphone_distance: MicrophoneDistance,

    #[serde(rename = "originalMediaType")]
    pub original_media_type: OriginalMediaType,

    #[serde(rename = "recordingDeviceType")]
    pub recording_device_type: RecordingDeviceType,

    #[serde(rename = "recordingDeviceName")]
    pub recording_device_name: String,

    #[serde(rename = "originalMimeType")]
    pub original_mime_type: String,

    #[serde(rename = "audioTopic")]
    pub audio_topic: String,
}

impl Into<GrpcRecognitionMetadata> for RecognitionMetadata {
    fn into(self) -> GrpcRecognitionMetadata {
        GrpcRecognitionMetadata {
            interaction_type: match self.interaction_type {
                InteractionType::INTERACTION_TYPE_UNSPECIFIED => 0,
                InteractionType::DISCUSSION => 1,
                InteractionType::PRESENTATION => 2,
                InteractionType::PHONE_CALL => 3,
                InteractionType::VOICEMAIL => 4,
                InteractionType::PROFESSIONALLY_PRODUCED => 5,
                InteractionType::VOICE_SEARCH => 6,
                InteractionType::VOICE_COMMAND => 7,
                InteractionType::DICTATION => 8,
            },
            industry_naics_code_of_audio: self.industry_naics_code_of_audio,
            microphone_distance: match self.microphone_distance {
                MicrophoneDistance::MICROPHONE_DISTANCE_UNSPECIFIED => 0,
                MicrophoneDistance::NEARFIELD => 1,
                MicrophoneDistance::MIDFIELD => 2,
                MicrophoneDistance::FARFIELD => 3,
            },
            original_media_type: match self.original_media_type {
                OriginalMediaType::ORIGINAL_MEDIA_TYPE_UNSPECIFIED => 0,
                OriginalMediaType::AUDIO => 1,
                OriginalMediaType::VIDEO => 2,
            },
            recording_device_type: match self.recording_device_type {
                RecordingDeviceType::RECORDING_DEVICE_TYPE_UNSPECIFIED => 0,
                RecordingDeviceType::SMARTPHONE => 1,
                RecordingDeviceType::PC => 2,
                RecordingDeviceType::PHONE_LINE => 3,
                RecordingDeviceType::VEHICLE => 4,
                RecordingDeviceType::OTHER_OUTDOOR_DEVICE => 5,
                RecordingDeviceType::OTHER_INDOOR_DEVICE => 6,
            },
            recording_device_name: self.recording_device_name,
            original_mime_type: self.original_mime_type,
            audio_topic: self.audio_topic,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RecognitionConfig {
    #[serde(default = "default_encoding")]
    pub encoding: AudioEncoding,

    #[serde(rename = "sampleRateHertz", skip_serializing_if = "Option::is_none")]
    pub sample_rate_hertz: Option<i32>,

    #[serde(rename = "audioChannelCount", default = "default_audio_channel_count")]
    pub audio_channel_count: i32,

    #[serde(
        rename = "enableSeparateRecognitionPerChannel",
        default = "default_enable_separate_recognition_per_channel"
    )]
    pub enable_separate_recognition_per_channel: bool,

    #[serde(rename = "languageCode", default = "default_language_code")]
    pub language_code: String,

    #[serde(rename = "maxAlternatives", default = "default_max_alternatives")]
    pub max_alternatives: i32,

    #[serde(rename = "profanityFilter", default = "default_profanity_filter")]
    pub profanity_filter: bool,

    #[serde(rename = "speechContexts")]
    pub speech_contexts: Vec<SpeechContext>,

    #[serde(
        rename = "enableWordTimeOffsets",
        default = "default_enable_word_time_offsets"
    )]
    pub enable_word_time_offsets: bool,

    #[serde(
        rename = "enableAutomaticPunctuation",
        default = "default_enable_automatic_punctuation"
    )]
    pub enable_automatic_punctuation: bool,

    #[serde(rename = "diarizationConfig", skip_serializing_if = "Option::is_none")]
    pub diarization_config: Option<SpeakerDiarizationConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<RecognitionMetadata>,

    #[serde(default = "default_model")]
    pub model: RecognitionConfigModel,

    #[serde(rename = "useEnhanced", skip_serializing_if = "Option::is_none")]
    pub use_enhanced: Option<bool>,
}

pub fn default_language_code() -> String {
    // for convenience we are setting this in google STT implementation
    // based on STT DB config
    "".to_owned()
}

pub fn default_model() -> RecognitionConfigModel {
    RecognitionConfigModel::default
}

pub fn default_encoding() -> AudioEncoding {
    AudioEncoding::LINEAR16
}

pub fn default_audio_channel_count() -> i32 {
    1
}

pub fn default_max_alternatives() -> i32 {
    1
}

pub fn default_profanity_filter() -> bool {
    false
}

pub fn default_enable_word_time_offsets() -> bool {
    false
}

pub fn default_enable_automatic_punctuation() -> bool {
    false
}

pub fn default_enable_separate_recognition_per_channel() -> bool {
    false
}

impl Into<GrpcRecognitionConfig> for RecognitionConfig {
    fn into(self) -> GrpcRecognitionConfig {
        GrpcRecognitionConfig {
            encoding: match self.encoding {
                AudioEncoding::ENCODING_UNSPECIFIED => 0,
                AudioEncoding::LINEAR16 => 1,
                AudioEncoding::FLAC => 2,
                AudioEncoding::MULAW => 3,
                AudioEncoding::AMR => 4,
                AudioEncoding::AMR_WB => 5,
                AudioEncoding::OGG_OPUS => 6,
                AudioEncoding::SPEEX_WITH_HEADER_BYTE => 7,
            },
            sample_rate_hertz: match self.sample_rate_hertz {
                Some(val) => val,
                None => 8000,
            },
            audio_channel_count: self.audio_channel_count,
            enable_separate_recognition_per_channel: self.enable_separate_recognition_per_channel,
            language_code: self.language_code,
            max_alternatives: self.max_alternatives,
            profanity_filter: self.profanity_filter,
            speech_contexts: {
                let mut speech_contexts: Vec<GrpcSpeechContext> = vec![];

                for item in self.speech_contexts {
                    speech_contexts.push(item.into())
                }
                speech_contexts
            },
            enable_word_time_offsets: self.enable_word_time_offsets,
            enable_automatic_punctuation: self.enable_automatic_punctuation,
            diarization_config: match self.diarization_config {
                Some(cfg) => Some(cfg.into()),
                _ => None,
            },
            metadata: match self.metadata {
                Some(md) => Some(md.into()),
                _ => None,
            },
            model: self.model.to_string(),
            use_enhanced: match self.use_enhanced {
                Some(val) => val,
                _ => false,
            },
        }
    }
}

/// Converts string into RecognitionConfig. Uses serde_path_to_error to get detailed and meaningful parsing errors
pub fn deserialize_recognition_config(json_str: &str) -> Result<RecognitionConfig> {
    let jd = &mut serde_json::Deserializer::from_str(json_str);
    let result: std::result::Result<RecognitionConfig, _> = serde_path_to_error::deserialize(jd);
    match result {
        Ok(config) => Ok(config),
        Err(err) => {
            let err_path = err.path().to_string();
            Err(Error::new(format!(
                "Error when deserializing speech recognition config (v1) at path: {}. Full error: {}",
                err_path,
                err
            )))
        }
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    // cargo test -- --show-output test_deserialize_recognition_config
    #[test]
    fn test_deserialize_recognition_config() {
        let json_str = r#"
            {
                "languageCode" : "sv_SE",
                "speechContexts" : [
                    {
                        "phrases" : [
                            "$OOV_CLASS_DIGIT_SEQUENCE"
                        ],
                        "boost" : 19
                    },
                    {
                        "phrases" : [
                            "$FULLPHONENUM"
                        ],
                        "boost" : 1
                    }
                ]
            }
            "#;
        // let recognition_config = serde_json::from_str::<RecognitionConfig>(json_str);
        // println!("recognition_config is {:#?}", recognition_config);
        let result = deserialize_recognition_config(json_str);
        match result {
            Ok(cfg) => println!("recognition_config is {:#?}", cfg),
            Err(err) => print!("recognition_config error: {:#?}", err),
        }
    }

    // cargo test -- --show-output test_2_deserialize_recognition_config
    #[test]
    fn test_2_deserialize_recognition_config() {
        let json_str = r#"
            {
                "encoding": "MULAW",
                "speechContexts" : [
                    {
                        "phrases" : [
                            "$OOV_CLASS_DIGIT_SEQUENCE"
                        ],
                        "boost" : 19
                    },
                    {
                        "phrases" : [
                            "$FULLPHONENUM"
                        ],
                        "boost" : 1
                    }
                ],
                "diarizationConfig": {
                    "enableSpeakerDiarization": false,
                    "minSpeakerCount": 2
                }
            }
            "#;
        let result = deserialize_recognition_config(json_str);
        match result {
            Ok(cfg) => println!("recognition_config is {:#?}", cfg),
            Err(err) => print!("recognition_config error: {:#?}", err),
        }
    }

    // cargo test -- --show-output test_convert_to_grpc
    #[test]
    fn test_convert_to_grpc() {
        let json_str = r#"
            {
                "encoding": "MULAW",
                "languageCode" : "sv_SE",
                "speechContexts" : [
                    {
                        "phrases" : [
                            "$FULLPHONENUM"
                        ],
                        "boost" : 1
                    }
                ],
                "diarizationConfig": {
                    "enableSpeakerDiarization": false,
                    "minSpeakerCount": 2
                }
            }
            "#;
        let recognition_config = deserialize_recognition_config(json_str).unwrap();
        let recognition_config_grpc: GrpcRecognitionConfig = recognition_config.into();
        // in the listing below speechContexts WILL NOT contain boosts since this is not supported
        // in v1 GRPC API
        println!("recognition_config_grpc(v1) {:#?}", recognition_config_grpc);
    }
}
