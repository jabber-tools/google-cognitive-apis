/// This module DOES NOT address all the differences between GRPC v1 and v1p1beta1 proto definitions
/// of speech-to-text API. For now it only defines extended version of SpeechContext struct
/// where boost parameter is added. Also RecognitionConfig using new SpeechContext is defined here.
/// JSON-to-GRPC struct conversion does not support following attributes:
///     adaptation
///     alternative_language_codes
///     diarization_speaker_count
///     enable_speaker_diarization
///     enable_spoken_emojis
///     enable_spoken_punctuation
///     enable_word_confidence
///     diarization_config
///     metadata
/// This can be added but would require implementation of serde deserialization
/// and Into<T> implementation as we currently have in v1 REST API.
use crate::api::grpc::google::cloud::speechtotext::v1p1beta1::SpeechContext as GrpcSpeechContext;
use serde::{Deserialize, Serialize};
use std::convert::Into;
// since initial implementation really extends just SpeechContext and defines RecognitionConfig which
// uses it we will reuse all other structures from v1 API
use super::v1::{
    default_audio_channel_count, default_enable_automatic_punctuation,
    default_enable_separate_recognition_per_channel, default_enable_word_time_offsets,
    default_encoding, default_language_code, default_max_alternatives, default_model,
    default_profanity_filter, AudioEncoding, RecognitionConfigModel, RecognitionMetadata,
    SpeakerDiarizationConfig,
};

use crate::api::grpc::google::cloud::speechtotext::v1p1beta1::RecognitionConfig as GrpcRecognitionConfig;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SpeechContext {
    phrases: Vec<String>,
    boost: f32,
}

impl Into<GrpcSpeechContext> for SpeechContext {
    fn into(self) -> GrpcSpeechContext {
        GrpcSpeechContext {
            phrases: self.phrases,
            boost: self.boost,
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

#[allow(deprecated)]
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
            // So far not supported!
            diarization_config: None,
            // So far not supported!
            metadata: None,
            model: self.model.to_string(),
            use_enhanced: match self.use_enhanced {
                Some(val) => val,
                _ => false,
            },
            // Below are the new v1p1beta1 attributes of RecognitionConfig
            // Since we introduced this config initially for enhanced SpeechContext
            // only these are not supported currently and we map just default/None values!
            adaptation: None,
            alternative_language_codes: vec![],
            diarization_speaker_count: 2,
            enable_speaker_diarization: false,
            enable_spoken_emojis: None,
            enable_spoken_punctuation: None,
            enable_word_confidence: false,
        }
    }
}
