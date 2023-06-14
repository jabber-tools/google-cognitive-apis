use crate::errors::{Error, Result};
use serde::Deserialize;
use serde_json::Value as JsonData;
use std::collections::HashMap;

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
    pub action: Option<String>,

    pub parameters: Option<JsonData>,

    #[serde(rename = "diagnosticInfo")]
    pub diagnostic_info: Option<DiagnosticInfo>,
}

#[derive(Debug, Deserialize)]
pub struct DiagnosticInfo {
    pub end_conversation: Option<bool>,
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

#[allow(dead_code)]
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

#[allow(dead_code)]
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

/// Converts string into DetectIntentResponse. Uses serde_path_to_error to get detailed and meaningful parsing errors
pub fn deserialize_detect_intent_response(json_str: &str) -> Result<DetectIntentResponse> {
    let jd = &mut serde_json::Deserializer::from_str(json_str);
    let result: std::result::Result<DetectIntentResponse, _> = serde_path_to_error::deserialize(jd);
    match result {
        Ok(config) => Ok(config),
        Err(err) => {
            let err_path = err.path().to_string();
            Err(Error::new(format!(
                "Error when deserializing detect_intent ressponse at path: {}. Full error: {}",
                err_path, err
            )))
        }
    }
}

/// Converts DetectIntentResponse params (arbitrary json data)
/// into Map<String, String>. If provided jason value is not object
/// None is returned instead
pub fn deserialize_detect_intent_response_params_to_str_map(
    json_val: &JsonData,
) -> Option<HashMap<String, String>> {
    if json_val.is_object() {
        let mut hash_map = HashMap::new();
        #[allow(for_loops_over_fallibles)]
        for params_map in json_val.as_object().iter() {
            for (key, val) in params_map.iter() {
                let mut val_str = &val.to_string()[..];
                if val_str.starts_with('"') && val_str.ends_with('"') {
                    val_str = &val_str[1..val_str.len() - 1];
                }
                hash_map.insert(key.to_string(), val_str.to_string());
            }
        }
        Some(hash_map)
    } else {
        None
    }
}

mod tests {
    // use super::*;

    // cargo test -- --show-output test_deserialize_detect_intent_resp
    #[test]
    fn test_deserialize_detect_intent_resp() {
        let json_str = r#"
            {
              "responseId": "721c1b2a-0403-4124-aa4f-b63773ef5cc1-123456",
              "queryResult": {
                "queryText": "cup of tea",
                "action": "_vap_events_voicegw_GDFHangupTimeout",
                "parameters": {
                  "hangup_play_msg": "1",
                  "hangup_timeout": "20000",
                  "foo": "bar",
                  "bar": {
                    "barbar": "foobar"
                  }
                },
                "allRequiredParamsPresent": true,
                "fulfillmentText": "Speak to me otherwise I will hangup in 20 seconds.",
                "fulfillmentMessages": [
                  {
                    "text": {
                      "text": [
                        "Speak to me otherwise I will hangup in 20 seconds."
                      ]
                    }
                  }
                ],
                "intent": {
                  "name": "projects/dummy-gcp-project/agent/intents/acdc4b71-0845-4a56-8084-123456",
                  "displayName": "Dummy Intent"
                },
                "intentDetectionConfidence": 1,
                "languageCode": "en",
                "sentimentAnalysisResult": {
                  "queryTextSentiment": {}
                }
              }
            }
            "#;

        let detect_intent_resp = super::deserialize_detect_intent_response(json_str).unwrap();
        println!("detect_intent_resp {:#?}", detect_intent_resp);

        let params = detect_intent_resp.query_result.unwrap().parameters.unwrap();
        let params_map =
            super::deserialize_detect_intent_response_params_to_str_map(&params).unwrap();
        println!("params {:#?}", params_map);
        assert_eq!(params_map.len(), 4);
        assert_eq!(params_map.get("hangup_timeout").unwrap(), "20000");
        assert_eq!(params_map.get("hangup_play_msg").unwrap(), "1");
        assert_eq!(params_map.get("foo").unwrap(), "bar");
        assert_eq!(params_map.get("bar").unwrap(), "{\"barbar\":\"foobar\"}");
    }
}
