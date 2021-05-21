use crate::api::grpc::google::cloud::dialogflow::v2beta1::{
    sessions_client::SessionsClient as GrpcSessionsClient, DetectIntentRequest,
    DetectIntentResponse, StreamingDetectIntentRequest, StreamingDetectIntentResponse,
};
use crate::common::{get_token, new_grpc_channel, new_interceptor};
use crate::errors::Result;
use async_stream::try_stream;
use futures_core::stream::Stream;
use log::*;
use std::result::Result as StdResult;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::Response as TonicResponse;
use tonic::Status as TonicStatus;
use tonic::{transport::Channel, Streaming};

/// Google Dialogflow sessions client.
#[derive(Debug)]
pub struct SessionsClient {
    /// internal GRPC dialogflow sessions client
    sessions_client: GrpcSessionsClient<Channel>,

    /// channel for sending audio data
    audio_sender: Option<mpsc::Sender<StreamingDetectIntentRequest>>,

    /// channel for streaming audio data into GRPC API
    audio_receiver: Option<mpsc::Receiver<StreamingDetectIntentRequest>>,

    /// For channel based streaming this is the internal channel sender
    /// where STT results will be sent. Library client is using respective
    /// receiver to get the results.
    result_sender: Option<mpsc::Sender<StreamingDetectIntentResponse>>,
}

impl SessionsClient {
    /// Creates new sessions client using GCP project JSON credentials
    /// This client should be used for synchronous invocation (detect_intent)
    pub async fn create_sync(google_credentials: impl AsRef<str>) -> Result<Self> {
        let channel = new_grpc_channel(
            "dialogflow.googleapis.com",
            "https://dialogflow.googleapis.com",
            None,
        )
        .await?;

        let token_header_val = get_token(google_credentials)?;

        let sessions_client: GrpcSessionsClient<Channel> =
            GrpcSessionsClient::with_interceptor(channel, new_interceptor(token_header_val)?);

        Ok(SessionsClient {
            sessions_client,
            audio_sender: None,
            audio_receiver: None,
            result_sender: None,
        })
    }

    /// Creates new sessions client using GCP project JSON credentials
    /// This client should be used for asynchronous invocation (streaming_detect_intent)
    /// See https://cloud.google.com/dialogflow/es/docs/how/detect-intent-stream
    pub async fn create_async(
        google_credentials: impl AsRef<str>,
        // initial configuration request
        streaming_detect_intent_req: StreamingDetectIntentRequest,
        // Capacity of audio sink (tokio channel used by caller to send audio data).
        // If not provided defaults to 1000.
        buffer_size: Option<usize>,
    ) -> Result<Self> {
        let channel = new_grpc_channel(
            "dialogflow.googleapis.com",
            "https://dialogflow.googleapis.com",
            None,
        )
        .await?;

        let token_header_val = get_token(google_credentials)?;

        let sessions_client: GrpcSessionsClient<Channel> =
            GrpcSessionsClient::with_interceptor(channel, new_interceptor(token_header_val)?);

        let (audio_sender, audio_receiver) =
            mpsc::channel::<StreamingDetectIntentRequest>(buffer_size.unwrap_or(1000));

        audio_sender.send(streaming_detect_intent_req).await?;

        Ok(SessionsClient {
            sessions_client,
            audio_sender: Some(audio_sender),
            audio_receiver: Some(audio_receiver),
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

    /// Returns receiver that can be used to receive streaming detect intents results
    pub fn get_streaming_result_receiver(
        &mut self,
        // buffer size for tokio channel. If not provided defaults to 1000.
        buffer_size: Option<usize>,
    ) -> mpsc::Receiver<StreamingDetectIntentResponse> {
        let (result_sender, result_receiver) =
            mpsc::channel::<StreamingDetectIntentResponse>(buffer_size.unwrap_or(1000));
        self.result_sender = Some(result_sender);
        result_receiver
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

    /// IMPORTANT: currently streaming_detect_intent does not work properly
    /// because half-close operation is not implemented. Details here(go example):
    /// https://cloud.google.com/dialogflow/es/docs/how/detect-intent-stream#detect-intent-stream-go
    #[allow(unreachable_code)]
    pub async fn streaming_detect_intent_async_stream(
        &mut self,
    ) -> impl Stream<Item = Result<StreamingDetectIntentResponse>> + '_ {
        try_stream! {
                // yank self.audio_receiver so that we can consume it
                if let Some(audio_receiver) = self.audio_receiver.take() {
                    let streaming_recognize_result: StdResult<
                        TonicResponse<Streaming<StreamingDetectIntentResponse>>,
                        TonicStatus,
                    > = self.sessions_client.streaming_detect_intent(ReceiverStream::new(audio_receiver)).await;

                    let mut response_stream: Streaming<StreamingDetectIntentResponse> =
                        streaming_recognize_result?.into_inner();

                    trace!("streaming_detect_intent_async_stream: entering loop");
                    loop {
                        if let Some(streaming_detect_intent_response) = response_stream.message().await? {
                            yield streaming_detect_intent_response;
                        }
                    }
                    trace!("streaming_detect_intent_async_stream: leaving loop");
                }
        }
    }

    /// Initiates bidirectional streaming. This call should be spawned
    /// into separate tokio task. Results can be then retrieved via
    /// channel receiver returned by method get_streaming_result_receiver.
    /// IMPORTANT: currently streaming_detect_intent does not work properly
    /// because half-close operation is not implemented. Details here(go example):
    /// https://cloud.google.com/dialogflow/es/docs/how/detect-intent-stream#detect-intent-stream-go
    pub async fn streaming_detect_intent(&mut self) -> Result<()> {
        // yank self.audio_receiver so that we can consume it
        if let Some(audio_receiver) = self.audio_receiver.take() {
            let streaming_recognize_result: StdResult<
                tonic::Response<Streaming<StreamingDetectIntentResponse>>,
                tonic::Status,
            > = self
                .sessions_client
                .streaming_detect_intent(ReceiverStream::new(audio_receiver))
                .await;

            let mut response_stream: Streaming<StreamingDetectIntentResponse> =
                streaming_recognize_result?.into_inner();

            loop {
                if let Some(streaming_detect_intent_response) = response_stream.message().await? {
                    if let Some(result_sender) = &self.result_sender {
                        result_sender.send(streaming_detect_intent_response).await?;
                    }
                }
            }
        }

        Ok(())
    }
}
