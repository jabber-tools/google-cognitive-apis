use crate::api::grpc::google::cloud::speechtotext::v1::{
    speech_client::SpeechClient, streaming_recognize_request::StreamingRequest,
    StreamingRecognitionConfig, StreamingRecognizeRequest, StreamingRecognizeResponse,
};
use crate::errors::Result;
use crate::CERTIFICATES;
use async_stream::try_stream;
use gouth::Builder;
use log::*;
use std::sync::Arc;
use tonic::{
    metadata::MetadataValue,
    transport::{Certificate, Channel, ClientTlsConfig},
    Code, Streaming,
};

use futures_core::stream::Stream;
use futures_util::pin_mut;
use futures_util::stream::StreamExt;
use tokio::sync::mpsc::{self, Receiver, Sender};

pub struct Recognizer {
    pub speech_client: SpeechClient<Channel>,
    audio_receiver: Option<mpsc::Receiver<u8>>,
}

impl Recognizer {
    pub async fn new(google_credentials: impl AsRef<str>) -> Result<Self> {
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

        Ok(Recognizer {
            speech_client,
            audio_receiver: None,
        })
    }

    pub fn get_audio_sink(&mut self) -> mpsc::Sender<u8> {
        let (audio_sender, audio_receiver) = mpsc::channel::<u8>(10240);
        self.audio_receiver = Some(audio_receiver);
        audio_sender
    }

    pub async fn streaming_recognize(&mut self) -> impl Stream<Item = Result<u8>> + '_ {
        try_stream! {
            if let Some(audio_receiver) = &mut self.audio_receiver {
                while let Some(audio_bytes) = audio_receiver.recv().await {
                    let output = audio_bytes * 2;
                    yield output;
                }
            }
            /*for i in 0..3 {
                yield i;
            }*/
        }
    }
}
