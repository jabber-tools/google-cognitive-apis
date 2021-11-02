#![allow(clippy::from_over_into)]
#![allow(clippy::manual_unwrap_or)]
#![allow(clippy::manual_map)]
use crate::api::grpc::google::cloud::speechtotext::v1p1beta1::SpeechAdaptation as GrpcSpeechAdaptation;
/// This module DOES NOT address all the differences between GRPC v1 and v1p1beta1 proto definitions
/// of speech-to-text API. For now it only defines extended version of SpeechContext struct
/// where boost parameter is added. Also RecognitionConfig using new SpeechContext is defined here.
///
/// RecognitionConfig also provides support for adaptation attribute (SpeechAdaptation) which
/// when used replaces SpeechContext.
/// JSON-to-GRPC struct conversion does not support following attributes:
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
use crate::errors::{Error, Result};
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

use crate::api::grpc::google::cloud::speechtotext::v1p1beta1::custom_class::ClassItem as GrpcClassItem;
use crate::api::grpc::google::cloud::speechtotext::v1p1beta1::phrase_set::Phrase as GrpcPhrase;
use crate::api::grpc::google::cloud::speechtotext::v1p1beta1::CustomClass as GrpcCustomClass;
use crate::api::grpc::google::cloud::speechtotext::v1p1beta1::PhraseSet as GrpcPhraseSet;
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

    #[serde(skip_serializing_if = "Option::is_none")]
    pub adaptation: Option<SpeechAdaptation>,
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
            adaptation: match self.adaptation {
                Some(adpt) => Some(adpt.into()),
                _ => None,
            },
            // Below are the new v1p1beta1 attributes of RecognitionConfig
            // Since we introduced this config initially for enhanced SpeechContext
            // only these are not supported currently and we map just default/None values!
            alternative_language_codes: vec![],
            diarization_speaker_count: 2,
            enable_speaker_diarization: false,
            enable_spoken_emojis: None,
            enable_spoken_punctuation: None,
            enable_word_confidence: false,
        }
    }
}
/// See https://cloud.google.com/speech-to-text/docs/reference/rest/v1p1beta1/RecognitionConfig#SpeechAdaptation
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SpeechAdaptation {
    #[serde(rename = "phraseSets")]
    pub phrase_sets: Vec<PhraseSet>,

    #[serde(rename = "phraseSetReferences")]
    pub phrase_set_references: Vec<String>,

    #[serde(rename = "customClasses")]
    pub custom_classes: Vec<CustomClass>,
}

impl Into<GrpcSpeechAdaptation> for SpeechAdaptation {
    fn into(self) -> GrpcSpeechAdaptation {
        GrpcSpeechAdaptation {
            phrase_sets: {
                let mut phrase_sets: Vec<GrpcPhraseSet> = vec![];

                for item in self.phrase_sets {
                    phrase_sets.push(item.into())
                }
                phrase_sets
            },
            phrase_set_references: {
                let mut phrase_set_references: Vec<String> = vec![];

                for item in self.phrase_set_references {
                    phrase_set_references.push(item.to_owned())
                }
                phrase_set_references
            },
            custom_classes: {
                let mut custom_classes: Vec<GrpcCustomClass> = vec![];

                for item in self.custom_classes {
                    custom_classes.push(item.into())
                }
                custom_classes
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PhraseSet {
    pub name: String,
    pub phrases: Vec<Phrase>,
    pub boost: f32,
}

impl Into<GrpcPhraseSet> for PhraseSet {
    fn into(self) -> GrpcPhraseSet {
        GrpcPhraseSet {
            name: self.name,
            phrases: {
                let mut grpc_phrases: Vec<GrpcPhrase> = vec![];

                for item in self.phrases {
                    grpc_phrases.push(item.into())
                }
                grpc_phrases
            },
            boost: self.boost,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Phrase {
    pub value: String,
    pub boost: f32,
}

impl Into<GrpcPhrase> for Phrase {
    fn into(self) -> GrpcPhrase {
        GrpcPhrase {
            value: self.value,
            boost: self.boost,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CustomClass {
    pub name: String,

    #[serde(rename = "customClassId")]
    pub custom_class_id: String,

    pub items: Vec<ClassItem>,
}

impl Into<GrpcCustomClass> for CustomClass {
    fn into(self) -> GrpcCustomClass {
        GrpcCustomClass {
            name: self.name,
            custom_class_id: self.custom_class_id,
            items: {
                let mut cc_items: Vec<GrpcClassItem> = vec![];

                for item in self.items {
                    cc_items.push(GrpcClassItem {
                        value: item.value.to_owned(),
                    })
                }
                cc_items
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClassItem {
    pub value: String,
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
                "Error when deserializing speech recognition config (v1p1beta1) at path: {}. Full error: {}",
                err_path,
                err.to_string()
            )))
        }
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    // cargo test -- --show-output test_convert_to_beta_grpc
    #[test]
    fn test_convert_to_beta_grpc() {
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
                },
                "adaptation": {
                    "phraseSets": [
                        {
                          "name": "phraseSets1",
                          "phrases": [
                            {
                              "value": "PHRASE_XY",
                              "boost": 23.0
                            }
                          ],
                          "boost": 24.0
                        }
                    ],
                    "phraseSetReferences": ["foo", "bar"],
                    "customClasses": [
                        {
                          "name": "customClasses1",
                          "customClassId": "customClasses1ID1",
                          "items": [
                            {
                              "value": "customClassItem1"
                            }
                          ]
                        }
                    ]
                }
            }
            "#;
        let recognition_config = deserialize_recognition_config(json_str).unwrap();
        let recognition_config_grpc: GrpcRecognitionConfig = recognition_config.into();
        // in the listing below speechContexts WILL contain boosts since this is supported
        // in v1p1beta1 GRPC API
        // diarization_config will be none since we do not support it yet for v1p1beta1
        // adaptation element will be present as well
        println!(
            "recognition_config_grpc(v1p1beta1) {:#?}",
            recognition_config_grpc
        );
    }
}
