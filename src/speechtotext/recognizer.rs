use crate::api::grpc::google::cloud::speechtotext::v1::{
    speech_client::SpeechClient, streaming_recognize_request::StreamingRequest,
    StreamingRecognitionConfig, StreamingRecognizeRequest, StreamingRecognizeResponse,
};
use crate::errors::Result;
use crate::CERTIFICATES;
use async_stream::try_stream;
use gouth::Builder;
use log::*;
use std::result::Result as StdResult;
use std::sync::Arc;
use tonic::{
    metadata::MetadataValue,
    transport::{Certificate, Channel, ClientTlsConfig},
    Streaming,
};

use futures_core::stream::Stream;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

/// Speech recognizer
pub struct Recognizer {
    speech_client: SpeechClient<Channel>,
    audio_sender: mpsc::Sender<StreamingRecognizeRequest>,
    audio_receiver: Option<mpsc::Receiver<StreamingRecognizeRequest>>,
}

impl Recognizer {
    /// Creates new speech recognizer from provided
    /// Google credentials and google speech configuration.
    pub async fn new(
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
            audio_sender,
            audio_receiver: Some(audio_receiver),
        })
    }

    /// Returns sender than can be used to stream in audio bytes.
    pub fn get_audio_sink(&mut self) -> mpsc::Sender<StreamingRecognizeRequest> {
        self.audio_sender.clone()
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
}
