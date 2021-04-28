use futures_util::pin_mut;
use futures_util::stream::StreamExt;
use google_cognitive_apis::speechtotext::recognizer::Recognizer;

use google_cognitive_apis::api::grpc::google::cloud::speechtotext::v1::{
    recognition_config::AudioEncoding, RecognitionConfig, StreamingRecognitionConfig,
    StreamingRecognizeRequest, StreamingRecognizeResponse,
};

use log::*;
use std::env;
use std::fs;

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();
    info!("recognizer example");

    let credentials = fs::read_to_string("/tmp/cred.json").unwrap();
    let streaming_config = StreamingRecognitionConfig {
        config: Some(RecognitionConfig {
            encoding: AudioEncoding::Linear16 as i32,
            sample_rate_hertz: 16000,
            audio_channel_count: 1,
            enable_separate_recognition_per_channel: false,
            language_code: "en-US".to_string(),
            max_alternatives: 1,
            profanity_filter: false,
            speech_contexts: vec![],
            enable_word_time_offsets: false,
            enable_automatic_punctuation: false,
            diarization_config: None,
            metadata: None,
            model: "".to_string(),
            use_enhanced: false,
        }),
        single_utterance: false,
        interim_results: true,
    };

    let mut recognizer = Recognizer::new(credentials, streaming_config)
        .await
        .unwrap();

    let audio_sender = recognizer.get_audio_sink();

    let s = recognizer.streaming_recognize().await;
    pin_mut!(s); // needed for iteration

    tokio::spawn(async move {
        audio_sender
            .send(Recognizer::streaming_request_from_bytes(vec![1, 2]))
            .await
            .unwrap();
        audio_sender
            .send(Recognizer::streaming_request_from_bytes(vec![1, 2]))
            .await
            .unwrap();
        audio_sender
            .send(Recognizer::streaming_request_from_bytes(vec![1, 2]))
            .await
            .unwrap();
        audio_sender
            .send(Recognizer::streaming_request_from_bytes(vec![1, 2]))
            .await
            .unwrap();
    });

    while let Some(val) = s.next().await {
        info!("got value {:?}", val);
    }
}
