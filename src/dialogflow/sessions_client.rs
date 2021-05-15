use crate::api::grpc::google::cloud::dialogflow::v2beta1::{
    sessions_client::SessionsClient as GrpcSessionsClient, DetectIntentRequest,
    DetectIntentResponse,
};
use crate::common::{new_grpc_channel, new_interceptor};
use crate::errors::Result;
use gouth::Builder;
use std::sync::Arc;
use tonic::transport::Channel;
use tonic::Response as TonicResponse;

/// Google Dialogflow sessions client.
pub struct SessionsClient {
    sessions_client: GrpcSessionsClient<Channel>,
}

impl SessionsClient {
    /// Creates new sessions client using GCP project JSON credentials
    pub async fn create(
        // Google Cloud Platform JSON credentials for project with Speech APIs enabled
        google_credentials: impl AsRef<str>,
    ) -> Result<Self> {
        let channel = new_grpc_channel(
            "dialogflow.googleapis.com",
            "https://dialogflow.googleapis.com",
            None,
        )
        .await?;

        let token = Builder::new().json(google_credentials).build()?;
        let token_header_val: Arc<String> = token.header_value()?;

        let sessions_client: GrpcSessionsClient<Channel> =
            GrpcSessionsClient::with_interceptor(channel, new_interceptor(token_header_val)?);

        Ok(SessionsClient { sessions_client })
    }

    /// Calls detect_intent API of underlying GRPC SessionClient.
    pub async fn detect_intent(
        &mut self,
        request: DetectIntentRequest,
    ) -> Result<DetectIntentResponse> {
        let detect_intent_req = tonic::Request::new(request);
        let response: TonicResponse<DetectIntentResponse> = self
            .sessions_client
            .detect_intent(detect_intent_req)
            .await?;
        Ok(response.into_inner())
    }

    /// Convenience function to check if DetectIntentResponse
    /// represents end of conversation. If so, returns true, otherwise false.
    pub fn is_eoc(response: DetectIntentResponse) -> bool {
        return if let Some(query_result) = response.query_result {
            let mut eoc = false;
            if let Some(diagnostic_info) = query_result.diagnostic_info {
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
}
