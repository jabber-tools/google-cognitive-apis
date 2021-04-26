use serde::Deserialize;

/// simplified dialogflow response
/// deserializes only what we need
#[derive(Debug, Deserialize)]
pub struct DialogFlowResponse {
    #[serde(rename = "queryResult")]
    pub query_result: Option<QueryResult>,
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
