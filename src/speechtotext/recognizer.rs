use crate::api::grpc::google::cloud::speechtotext::v1::{
    speech_client::SpeechClient, streaming_recognize_request::StreamingRequest,
    LongRunningRecognizeRequest, StreamingRecognitionConfig, StreamingRecognizeRequest,
    StreamingRecognizeResponse,
};
use crate::api::grpc::google::longrunning::{
    operation::Result as OperationResult, operations_client::OperationsClient, GetOperationRequest,
    Operation,
};
use crate::errors::Result;
use crate::CERTIFICATES;
use async_stream::try_stream;
use gouth::Builder;
use log::*;
use std::result::Result as StdResult;
use std::sync::Arc;
use tonic::Response as TonicResponse;
use tonic::{
    metadata::MetadataValue,
    transport::{Certificate, Channel, ClientTlsConfig},
    Response as GrpcResponse, Streaming,
};

use futures_core::stream::Stream;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::sleep;
use tokio_stream::wrappers::ReceiverStream;

/// Speech recognizer
pub struct Recognizer {
    speech_client: SpeechClient<Channel>,
    operations_client: Option<OperationsClient<Channel>>,
    audio_sender: Option<mpsc::Sender<StreamingRecognizeRequest>>,
    audio_receiver: Option<mpsc::Receiver<StreamingRecognizeRequest>>,
}

impl Recognizer {
    /// Creates new speech recognizer from provided
    /// Google credentials and google speech configuration.
    /// This kind of recognizer can be used for streaming recognition.
    pub async fn new_for_streaming(
        google_credentials: impl AsRef<str>,
        config: StreamingRecognitionConfig,
    ) -> Result<Self> {
        let tls_config = ClientTlsConfig::new()
            .ca_certificate(Certificate::from_pem(CERTIFICATES))
            .domain_name("speech.googleapis.com");

        let channel = Channel::from_static("https://speech.googleapis.com")
            .tls_config(tls_config.clone())?
            //.timeout(std::time::Duration::from_secs(2))
            .connect()
            .await?;

        let token = Builder::new().json(google_credentials).build()?;

        let token_header_val: Arc<String> = token.header_value()?;
        let speech_client: SpeechClient<Channel> =
            SpeechClient::with_interceptor(channel, move |mut req: tonic::Request<()>| {
                let meta = MetadataValue::from_str(&token_header_val).unwrap();
                req.metadata_mut().insert("authorization", meta);
                Ok(req)
            });

        let (audio_sender, audio_receiver) = mpsc::channel::<StreamingRecognizeRequest>(10240);

        let streaming_config = StreamingRecognizeRequest {
            streaming_request: Some(StreamingRequest::StreamingConfig(config)),
        };

        audio_sender.send(streaming_config).await?;

        Ok(Recognizer {
            speech_client,
            operations_client: None,
            audio_sender: Some(audio_sender),
            audio_receiver: Some(audio_receiver),
        })
    }

    /// Creates new speech recognizer from provided
    /// Google credentials. This kind of recognizer can be used
    /// for long running recognition.
    pub async fn new_for_long_running(google_credentials: impl AsRef<str>) -> Result<Self> {
        let tls_config = ClientTlsConfig::new()
            .ca_certificate(Certificate::from_pem(CERTIFICATES))
            .domain_name("speech.googleapis.com");

        let channel = Channel::from_static("https://speech.googleapis.com")
            .tls_config(tls_config.clone())?
            //.timeout(std::time::Duration::from_secs(2))
            .connect()
            .await?;

        let token = Builder::new().json(google_credentials).build()?;

        let token_header_val: Arc<String> = token.header_value()?;
        let token_header_val2: Arc<String> = token_header_val.clone();

        let speech_client: SpeechClient<Channel> =
            SpeechClient::with_interceptor(channel.clone(), move |mut req: tonic::Request<()>| {
                let meta = MetadataValue::from_str(&token_header_val).unwrap();
                req.metadata_mut().insert("authorization", meta);
                Ok(req)
            });

        let operations_client =
            OperationsClient::with_interceptor(channel, move |mut req: tonic::Request<()>| {
                let meta = MetadataValue::from_str(&token_header_val2).unwrap();
                req.metadata_mut().insert("authorization", meta);
                Ok(req)
            });

        Ok(Recognizer {
            speech_client,
            operations_client: Some(operations_client),
            audio_sender: None,
            audio_receiver: None,
        })
    }

    /// Returns sender than can be used to stream in audio bytes.
    pub fn get_audio_sink(&mut self) -> Option<mpsc::Sender<StreamingRecognizeRequest>> {
        return if let Some(audio_sender) = &self.audio_sender {
            Some(audio_sender.clone())
        } else {
            None
        };
    }

    /// Convenience function so that client does not have to create full StreamingRecognizeRequest
    /// and can just pass audio bytes vector instead.
    pub fn streaming_request_from_bytes(audio_bytes: Vec<u8>) -> StreamingRecognizeRequest {
        StreamingRecognizeRequest {
            streaming_request: Some(StreamingRequest::AudioContent(audio_bytes)),
        }
    }

    /// Initiates bidirectional streaming. returns
    /// asynchronous stream of streaming recognition results
    /// Audio data must be fed into recognizer via channel sender
    /// returned by function get_audio_sink.
    #[allow(unreachable_code)]
    pub async fn streaming_recognize(
        &mut self,
    ) -> impl Stream<Item = Result<StreamingRecognizeResponse>> + '_ {
        try_stream! {
                // yank self.audio_receiver so that we can consume it
                if let Some(audio_receiver) = self.audio_receiver.take() {
                    let streaming_recognize_result: StdResult<
                        tonic::Response<Streaming<StreamingRecognizeResponse>>,
                        tonic::Status,
                    > = self.speech_client.streaming_recognize(ReceiverStream::new(audio_receiver)).await;

                    let mut response_stream: Streaming<StreamingRecognizeResponse> =
                        streaming_recognize_result?.into_inner();

                    trace!("streaming_recognize: entering loop");
                    loop {
                        if let Some(streaming_recognize_response) = response_stream.message().await? {
                            yield streaming_recognize_response;
                        }
                    }
                    trace!("streaming_recognize: leaving loop");
                }
        }
    }

    pub async fn long_running_recognize(
        &mut self,
        request: LongRunningRecognizeRequest,
    ) -> Result<GrpcResponse<Operation>> {
        Ok(self.speech_client.long_running_recognize(request).await?)
    }

    pub async fn long_running_wait(
        &mut self,
        operation: Operation,
    ) -> Result<Option<OperationResult>> {
        let operation_req = GetOperationRequest {
            name: operation.name.clone(),
        };

        loop {
            if let Some(oper_client) = &mut self.operations_client {
                let tonic_response: TonicResponse<Operation> =
                    oper_client.get_operation(operation_req.clone()).await?;
                let operation = tonic_response.into_inner();
                if operation.done == true {
                    return Ok(operation.result);
                } else {
                    sleep(Duration::from_millis(1000)).await;
                }
            }
        }
    }
}
