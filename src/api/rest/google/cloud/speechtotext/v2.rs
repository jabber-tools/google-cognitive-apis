#![allow(clippy::from_over_into)]
#![allow(clippy::manual_unwrap_or)]
#![allow(clippy::manual_map)]
#![allow(unused_imports)]
use crate::api::grpc::google::cloud::speechtotext::v2::SpeechAdaptation as GrpcSpeechAdaptation;
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
use crate::errors::{Error, Result};
use serde::{Deserialize, Serialize, Serializer};
use std::convert::Into;
// since initial implementation really extends just SpeechContext and defines RecognitionConfig which
// uses it we will reuse all other structures from v1 API
use super::v1::{
    default_audio_channel_count, default_enable_automatic_punctuation,
    default_enable_separate_recognition_per_channel, default_enable_word_time_offsets,
    default_encoding, default_language_code, default_max_alternatives, default_model,
    default_profanity_filter, AudioEncoding, RecognitionConfigModel, RecognitionMetadata,
};

use crate::api::grpc::google::cloud::speechtotext::v2::{
    custom_class::ClassItem as GrpcClassItem,
    phrase_set::Phrase as GrpcPhrase,
    CustomClass as GrpcCustomClass,
    PhraseSet as GrpcPhraseSet,
    RecognitionConfig as GrpcRecognitionConfig,
    RecognitionFeatures as GrpcRecognitionFeatures,
    speech_adaptation::AdaptationPhraseSet as GrpcAdaptationPhraseSet,
    speech_adaptation::adaptation_phrase_set::Value as GrpcValue,
    recognition_config::DecodingConfig,
    AutoDetectDecodingConfig, TranslationConfig,
    ExplicitDecodingConfig, 
    SpeakerDiarizationConfig as GrpcSpeakerDiarizationConfig
};

// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct SpeakerDiarizationConfig {

//     #[serde(rename = "minSpeakerCount", default = "default_min_speaker_count")]
//     min_speaker_count: i32,

//     #[serde(rename = "maxSpeakerCount", default = "default_max_speaker_count")]
//     max_speaker_count: i32,
// }

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RecognitionConfig {
    #[serde(default = "default_encoding")]
    pub encoding: AudioEncoding,

    #[serde(rename = "sampleRateHertz", skip_serializing_if = "Option::is_none")]
    pub sample_rate_hertz: Option<i32>,

    #[serde(rename = "audioChannelCount", default = "default_audio_channel_count")]
    pub audio_channel_count: i32,

    // #[serde(
    //     rename = "enableSeparateRecognitionPerChannel",
    //     default = "default_enable_separate_recognition_per_channel"
    // )]
    // pub enable_separate_recognition_per_channel: bool,

    #[serde(rename = "languageCode", default = "default_language_code")]
    pub language_code: String,

    #[serde(rename = "maxAlternatives", default = "default_max_alternatives")]
    pub max_alternatives: i32,

    #[serde(rename = "profanityFilter", default = "default_profanity_filter")]
    pub profanity_filter: bool,

    // // Deprecated
    // // #[serde(rename = "speechContexts")]
    // // pub speech_contexts: Vec<SpeechContext>,

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

    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub metadata: Option<RecognitionMetadata>,

    #[serde(default = "default_model")]
    pub model: RecognitionConfigModel,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub adaptation: Option<SpeechAdaptation>,

    pub target_language: Option<String>    
}

#[allow(deprecated)]
impl Into<GrpcRecognitionConfig> for RecognitionConfig {
    fn into(self) -> GrpcRecognitionConfig {
        GrpcRecognitionConfig {
            model: self.model.to_string(),
            language_codes: vec![self.language_code.to_string()],
            features: Some(GrpcRecognitionFeatures {
                profanity_filter: self.profanity_filter,
                enable_automatic_punctuation: self.enable_automatic_punctuation,
                enable_word_time_offsets: self.enable_word_time_offsets,
                enable_word_confidence: false,
                max_alternatives: self.max_alternatives,
                diarization_config: match self.diarization_config {
                    Some(conf) => Some(conf.into()),
                    _ => None
                },
                ..Default::default()
            }),
            adaptation: match self.adaptation {
                Some(adap) => Some(adap.into()),
                _ => None
            },
            transcript_normalization: None,
            translation_config: match self.target_language {
                Some(target_language) => Some(TranslationConfig {
                    target_language
                }),
                None => None
            },
            denoiser_config: None,
            decoding_config: Some(DecodingConfig::ExplicitDecodingConfig(
                ExplicitDecodingConfig {
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
                },
            )),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SpeakerDiarizationConfig {
    #[serde(rename = "minSpeakerCount")]
    pub min_speaker_count: i32,
    #[serde(rename = "maxSpeakerCount")]
    pub max_speaker_count: i32,
}

impl Into<GrpcSpeakerDiarizationConfig> for SpeakerDiarizationConfig {
    fn into(self) -> GrpcSpeakerDiarizationConfig {
        GrpcSpeakerDiarizationConfig { 
            min_speaker_count: self.min_speaker_count,
            max_speaker_count: self.max_speaker_count
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Value {
    PhraseSet(String),
    InlinePhraseSet(PhraseSet),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AdaptationPhraseSet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
}

impl Into<GrpcAdaptationPhraseSet> for AdaptationPhraseSet {
    fn into(self) -> GrpcAdaptationPhraseSet {
        let value = match self.value {
            Some(Value::PhraseSet(name)) => Some(GrpcValue::PhraseSet(name)),
            Some(Value::InlinePhraseSet(inline)) => {
                Some(GrpcValue::InlinePhraseSet(inline.into()))
            }
            None => None,
        };

        GrpcAdaptationPhraseSet { value }
    }
}
        
/// See https://cloud.google.com/speech-to-text/docs/reference/rest/v1p1beta1/RecognitionConfig#SpeechAdaptation
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SpeechAdaptation {
    #[serde(rename = "phraseSets")]
    pub phrase_sets: Vec<AdaptationPhraseSet>,

    #[serde(rename = "customClasses")]
    pub custom_classes: Vec<CustomClass>,
}

impl Into<GrpcSpeechAdaptation> for SpeechAdaptation {
    fn into(self) -> GrpcSpeechAdaptation {
        GrpcSpeechAdaptation {
            phrase_sets: self.phrase_sets
                .into_iter()
                .map(Into::into)
                .collect(),
            custom_classes: self.custom_classes
                .into_iter()
                .map(Into::into)
                .collect(),
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

            phrases: self.phrases
                .into_iter()
                .map(Into::into)
                .collect(),

            boost: self.boost,
            ..Default::default()
            // Output-only / optional fields → leave unset
            // uid: String::new(),
            // display_name: String::new(),
            // state: 0, // default enum value
            // create_time: None,
            // update_time: None,
            // delete_time: None,
            // expire_time: None,
            // annotations: Default::default(),
            // etag: String::new(),
            // reconciling: false,
            // kms_key_name: String::new(),
            // kms_key_version_name: String::new(),
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
            items: self.items
                .into_iter()
                .map(|item| GrpcClassItem {
                    value: item.value,
                })
                .collect(),
            ..Default::default()
            // Output-only / optional → defaults
            // uid: String::new(),
            // display_name: String::new(),
            // state: 0, // enum default
            // create_time: None,
            // update_time: None,
            // delete_time: None,
            // expire_time: None,
            // annotations: Default::default(),
            // etag: String::new(),
            // reconciling: false,
            // kms_key_name: String::new(),
            // kms_key_version_name: String::new(),
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
                err
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
