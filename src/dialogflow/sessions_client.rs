//! This module contains Google Dialogflow Session client
//! that can be used for detec_intent operation. This is simple
//! request/response operation, no streaming is involved. As a result
//! this client is easily cloneable.
#![allow(clippy::manual_map)]
use crate::api::grpc::google::cloud::dialogflow::v2beta1::{
    sessions_client::SessionsClient as GrpcSessionsClient, DetectIntentRequest,
    DetectIntentResponse, StreamingDetectIntentRequest, StreamingDetectIntentResponse,
};
use crate::common::{get_token, new_grpc_channel, new_interceptor, TokenInterceptor};
use crate::errors::Result;
use tokio::sync::mpsc;
use tonic::codegen::InterceptedService;
use tonic::transport::Channel;
use tonic::Response as TonicResponse;

/// Google Dialogflow sessions client.
/// Used for detect intent API. Is cloneable!
#[derive(Debug, Clone)]
pub struct SessionsClient {
    /// internal GRPC dialogflow sessions client
    sessions_client: GrpcSessionsClient<InterceptedService<Channel, TokenInterceptor>>,


    /// channel for sending audio data
    audio_sender: Option<mpsc::Sender<StreamingDetectIntentRequest>>,

    /// For channel based streaming this is the internal channel sender
    /// where STT results will be sent. Library client is using respective
    /// receiver to get the results.
    result_sender: Option<mpsc::Sender<StreamingDetectIntentResponse>>,
}

impl SessionsClient {
    /// Creates new sessions client using GCP project JSON credentials
    /// This client should be used for synchronous invocation (detect_intent)
    pub async fn create(google_credentials: impl AsRef<str>) -> Result<Self> {
        let channel = new_grpc_channel(
            "dialogflow.googleapis.com",
            "https://dialogflow.googleapis.com",
            None,
        )
        .await?;

        let token_header_val = get_token(google_credentials)?;

        let sessions_client =
            GrpcSessionsClient::with_interceptor(channel, new_interceptor(token_header_val));

        Ok(SessionsClient {
            sessions_client,
            audio_sender: None,
            result_sender: None,
        })
    }

    /// Returns sender than can be used to stream in audio bytes.
    pub fn get_audio_sink(&mut self) -> Option<mpsc::Sender<StreamingDetectIntentRequest>> {
        if let Some(audio_sender) = &self.audio_sender {
            Some(audio_sender.clone())
        } else {
            None
        }
    }

    /// Convenience function so that client does not have to create full StreamingDetectIntentRequest
    /// and can just pass audio bytes vector instead.
    #[allow(deprecated)]
    pub fn streaming_request_from_bytes(
        session: String,
        audio_bytes: Vec<u8>,
    ) -> StreamingDetectIntentRequest {
        StreamingDetectIntentRequest {
            session,
            query_params: None,
            query_input: None,
            // setting always to false. This should be set by user
            // in initial streaming config (see create_async)
            single_utterance: false,
            output_audio_config: None,
            output_audio_config_mask: None,
            input_audio: audio_bytes,
        }
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
    pub fn is_eoc(response: &DetectIntentResponse) -> bool {
        super::is_eoc(response)
    }

    /// Convenience function to return properly formatted session string
    /// for detect intent call.
    pub fn get_session_string(project_id: &str, session_id: &str) -> String {
        super::get_session_string(project_id, session_id)
    }
}
