use crate::api::grpc::google::cloud::speechtotext::v1::{
    speech_client::SpeechClient, streaming_recognize_request::StreamingRequest,
    LongRunningRecognizeRequest, LongRunningRecognizeResponse, RecognizeRequest, RecognizeResponse,
    StreamingRecognitionConfig, StreamingRecognizeRequest, StreamingRecognizeResponse,
};
use crate::api::grpc::google::longrunning::{
    operation::Result as OperationResult, operations_client::OperationsClient, GetOperationRequest,
    Operation,
};
use crate::errors::{Error, Result};
use crate::CERTIFICATES;
use async_stream::try_stream;
use futures_core::stream::Stream;
use gouth::Builder;
use log::*;
use prost::Message;
use std::io::Cursor;
use std::result::Result as StdResult;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::sleep;
use tokio_stream::wrappers::ReceiverStream;
use tonic::Response as TonicResponse;
use tonic::{
    metadata::MetadataValue,
    transport::{Certificate, Channel, ClientTlsConfig},
    Response as GrpcResponse, Streaming,
};

/// Google Speech API recognizer
pub struct Recognizer {
    speech_client: SpeechClient<Channel>,
    operations_client: Option<OperationsClient<Channel>>,
    audio_sender: Option<mpsc::Sender<StreamingRecognizeRequest>>,
    audio_receiver: Option<mpsc::Receiver<StreamingRecognizeRequest>>,
    /// experimental for now. to compare traditional channels with
    /// async streams in terms of performance. used by streaming_recognize_2
    result_sender: Option<mpsc::Sender<StreamingRecognizeResponse>>,
}

impl Recognizer {
    /// Convenience function to return tonic Interceptor
    /// (see https://docs.rs/tonic/0.4.3/tonic/struct.Interceptor.html)
    fn new_interceptor(token_header_val: Arc<String>) -> Result<tonic::Interceptor> {
        let interceptor = tonic::Interceptor::new(move |mut req: tonic::Request<()>| {
            let meta_result = MetadataValue::from_str(&token_header_val);

            return match meta_result {
                Ok(meta) => {
                    req.metadata_mut().insert("authorization", meta);
                    Ok(req)
                }
                Err(some_error) => Err(tonic::Status::internal(format!(
                    "new_interceptor: Error when getting MetadataValue from token {:?}",
                    some_error
                ))),
            };
        });
        Ok(interceptor)
    }

    /// Creates new GRPC channel to speech.googleapis.com API
    async fn new_grpc_channel() -> Result<Channel> {
        let tls_config = ClientTlsConfig::new()
            .ca_certificate(Certificate::from_pem(CERTIFICATES))
            .domain_name("speech.googleapis.com");

        Ok(Channel::from_static("https://speech.googleapis.com")
            .tls_config(tls_config.clone())?
            //.timeout(std::time::Duration::from_secs(2))
            .connect()
            .await?)
    }

    /// Creates new speech recognizer from provided
    /// Google credentials and google speech configuration.
    /// This kind of recognizer can be used for streaming recognition.
    pub async fn create_streaming_recognizer(
        google_credentials: impl AsRef<str>,
        config: StreamingRecognitionConfig,
    ) -> Result<Self> {
        let channel = Recognizer::new_grpc_channel().await?;

        let token = Builder::new().json(google_credentials).build()?;
        let token_header_val: Arc<String> = token.header_value()?;

        let speech_client: SpeechClient<Channel> =
            SpeechClient::with_interceptor(channel, Recognizer::new_interceptor(token_header_val)?);

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
            result_sender: None,
        })
    }

    /// Creates new speech recognizer from provided
    /// Google credentials. This kind of recognizer can be used
    /// for long running recognition.
    pub async fn create_asynchronous_recognizer(
        google_credentials: impl AsRef<str>,
    ) -> Result<Self> {
        let channel = Recognizer::new_grpc_channel().await?;

        let token = Builder::new().json(google_credentials).build()?;
        let token_header_val: Arc<String> = token.header_value()?;

        let speech_client: SpeechClient<Channel> = SpeechClient::with_interceptor(
            channel.clone(),
            Recognizer::new_interceptor(token_header_val.clone())?,
        );

        let operations_client = OperationsClient::with_interceptor(
            channel,
            Recognizer::new_interceptor(token_header_val)?,
        );

        Ok(Recognizer {
            speech_client,
            operations_client: Some(operations_client),
            audio_sender: None,
            audio_receiver: None,
            result_sender: None,
        })
    }

    /// Creates new speech recognizer from provided
    /// Google credentials. This kind of recognizer can be used
    /// for synchronous recognition.
    pub async fn create_synchronous_recognizer(
        google_credentials: impl AsRef<str>,
    ) -> Result<Self> {
        let channel = Recognizer::new_grpc_channel().await?;

        let token = Builder::new().json(google_credentials).build()?;
        let token_header_val: Arc<String> = token.header_value()?;

        let speech_client: SpeechClient<Channel> = SpeechClient::with_interceptor(
            channel.clone(),
            Recognizer::new_interceptor(token_header_val.clone())?,
        );

        Ok(Recognizer {
            speech_client,
            operations_client: None,
            audio_sender: None,
            audio_receiver: None,
            result_sender: None,
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

    /// Returns receiver that can be used to receive speech-to-text results
    /// used with streaming_recognize_2 function
    pub fn get_streaming_result_receiver(&mut self) -> mpsc::Receiver<StreamingRecognizeResponse> {
        let (result_sender, result_receiver) = mpsc::channel::<StreamingRecognizeResponse>(10240);
        self.result_sender = Some(result_sender);
        result_receiver
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

    /// to compare async streams with channels in terms of performance
    pub async fn streaming_recognize_2(&mut self) -> Result<()> {
        // yank self.audio_receiver so that we can consume it
        if let Some(audio_receiver) = self.audio_receiver.take() {
            let streaming_recognize_result: StdResult<
                tonic::Response<Streaming<StreamingRecognizeResponse>>,
                tonic::Status,
            > = self
                .speech_client
                .streaming_recognize(ReceiverStream::new(audio_receiver))
                .await;

            let mut response_stream: Streaming<StreamingRecognizeResponse> =
                streaming_recognize_result?.into_inner();

            loop {
                if let Some(streaming_recognize_response) = response_stream.message().await? {
                    if let Some(result_sender) = &self.result_sender {
                        result_sender.send(streaming_recognize_response).await?;
                    }
                }
            }
        }

        Ok(())
    }

    /// Initiates asynchronous recognition.
    /// Returns long running operation representing
    /// asynchronous computation performed by Google Cloud Platform.
    /// Use long_running_wait to wait until operation is done.
    pub async fn long_running_recognize(
        &mut self,
        request: LongRunningRecognizeRequest,
    ) -> Result<GrpcResponse<Operation>> {
        Ok(self.speech_client.long_running_recognize(request).await?)
    }

    /// Waits for completion of long running operation returned
    /// by long_running_recognize function. Long running operation
    /// result is then casted into LongRunningRecognizeResponse struct.
    /// Function checks operation status regularly using get_operation
    /// which is called every check_interval_ms ms. If check_interval_ms
    /// is not specified default interval check is 1 sec.
    pub async fn long_running_wait(
        &mut self,
        operation: Operation,
        check_interval_ms: Option<u64>,
    ) -> Result<Option<LongRunningRecognizeResponse>> {
        let operation_req = GetOperationRequest {
            name: operation.name.clone(),
        };

        loop {
            if let Some(oper_client) = &mut self.operations_client {
                let tonic_response: TonicResponse<Operation> =
                    oper_client.get_operation(operation_req.clone()).await?;
                let operation = tonic_response.into_inner();
                if operation.done == true {
                    return if let Some(operation_result) = operation.result {
                        match operation_result {
                            OperationResult::Error(rpc_status) => {
                                error!("Recognizer.long_running_wait rpc error {:?}", rpc_status);
                                Err(Error::new_with_code(
                                    rpc_status.message,
                                    rpc_status.code.to_string(),
                                ))
                            }
                            OperationResult::Response(any_response) => {
                                let lrr_response: LongRunningRecognizeResponse =
                                    LongRunningRecognizeResponse::decode(&mut Cursor::new(
                                        any_response.value,
                                    ))?;
                                Ok(Some(lrr_response))
                            }
                        }
                    } else {
                        Ok(None)
                    };
                } else {
                    sleep(Duration::from_millis(check_interval_ms.unwrap_or(1000))).await;
                }
            }
        }
    }

    /// Performs synchronous speech recognition
    pub async fn recognize(&mut self, request: RecognizeRequest) -> Result<RecognizeResponse> {
        let tonic_response: TonicResponse<RecognizeResponse> =
            self.speech_client.recognize(request).await?;
        Ok(tonic_response.into_inner())
    }
}
