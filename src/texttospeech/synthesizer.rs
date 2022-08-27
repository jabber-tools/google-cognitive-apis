//! Text-to-speech synthesizer module.
use crate::api::grpc::google::cloud::texttospeech::v1::{
    text_to_speech_client::TextToSpeechClient, ListVoicesRequest, ListVoicesResponse,
    SynthesizeSpeechRequest, SynthesizeSpeechResponse,
};
use crate::common::{get_token, new_grpc_channel, new_interceptor, TokenInterceptor};
use crate::errors::Result;
use tonic::codegen::InterceptedService;
use tonic::transport::Channel;
use tonic::Response as TonicResponse;

/// Google speech synthesizer
#[derive(Debug, Clone)]
pub struct Synthesizer {
    /// underlying gRPC Tonic text-to-speech client
    // text_to_speech_client: TextToSpeechClient<Channel>,
    text_to_speech_client: TextToSpeechClient<InterceptedService<Channel, TokenInterceptor>>,
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

        let token_header_val = get_token(google_credentials)?;

        let text_to_speech_client =
            TextToSpeechClient::with_interceptor(channel, new_interceptor(token_header_val));

        Ok(Synthesizer {
            text_to_speech_client,
        })
    }

    /// Synthesizes speech synchronously.
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

    /// Returns a list of Voice supported for synthesis.
    pub async fn list_voices(&mut self, request: ListVoicesRequest) -> Result<ListVoicesResponse> {
        let list_voices_req = tonic::Request::new(request);
        let response: TonicResponse<ListVoicesResponse> = self
            .text_to_speech_client
            .list_voices(list_voices_req)
            .await?;
        Ok(response.into_inner())
    }
}
