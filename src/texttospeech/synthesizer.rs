use crate::api::grpc::google::cloud::texttospeech::v1::{
    text_to_speech_client::TextToSpeechClient, SynthesizeSpeechRequest, SynthesizeSpeechResponse,
};
use crate::common::{new_grpc_channel, new_interceptor};
use crate::errors::Result;
use gouth::Builder;
use std::sync::Arc;
use tonic::transport::Channel;
use tonic::Response as TonicResponse;

/// Google speech synthesizer
#[derive(Debug, Clone)]
pub struct Synthesizer {
    text_to_speech_client: TextToSpeechClient<Channel>,
}

impl Synthesizer {
    /// Creates new text-to-speech synthesizer using GCP project JSON credentials
    pub async fn create(
        // Google Cloud Platform JSON credentials for project with Speech APIs enabled
        google_credentials: impl AsRef<str>,
    ) -> Result<Self> {
        let channel = new_grpc_channel(
            "texttospeech.googleapis.com",
            "https://texttospeech.googleapis.com",
            None,
        )
        .await?;

        let token = Builder::new().json(google_credentials).build()?;
        let token_header_val: Arc<String> = token.header_value()?;

        let text_to_speech_client: TextToSpeechClient<Channel> =
            TextToSpeechClient::with_interceptor(channel, new_interceptor(token_header_val)?);

        Ok(Synthesizer {
            text_to_speech_client,
        })
    }

    /// Synthetizes plain string or ssml string into audio bytes.
    pub async fn synthesize_speech(
        &mut self,
        request: SynthesizeSpeechRequest,
    ) -> Result<SynthesizeSpeechResponse> {
        let synthesize_speech_req = tonic::Request::new(request);
        let response: TonicResponse<SynthesizeSpeechResponse> = self
            .text_to_speech_client
            .synthesize_speech(synthesize_speech_req)
            .await?;
        Ok(response.into_inner())
    }
}
