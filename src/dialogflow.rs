//! Contains dialogflow session client.

use crate::api::grpc::google::cloud::dialogflow::v2beta1::DetectIntentResponse;

pub mod sessions_client;
pub mod sessions_client_streaming;

/// Convenience function to check if DetectIntentResponse
/// represents end of conversation. If so, returns true, otherwise false.
pub fn is_eoc(response: &DetectIntentResponse) -> bool {
    return if let Some(query_result) = &response.query_result {
        let mut eoc = false;
        if let Some(diagnostic_info) = &query_result.diagnostic_info {
            if let Some(end_conversation) = diagnostic_info.fields.get("end_conversation") {
                eoc = match end_conversation.kind {
                    Some(prost_types::value::Kind::BoolValue(val)) => val,
                    _ => false,
                };
            }
        }
        eoc
    } else {
        false
    };
}

/// Convenience function to return properly formatted session string
/// for detect intent call.
pub fn get_session_string(project_id: &str, session_id: &str) -> String {
    format!("projects/{}/agent/sessions/{}", project_id, session_id)
}
