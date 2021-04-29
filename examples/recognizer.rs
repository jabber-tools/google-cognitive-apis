use futures_util::pin_mut;
use futures_util::stream::StreamExt;
use google_cognitive_apis::speechtotext::recognizer::Recognizer;

use google_cognitive_apis::api::grpc::google::cloud::speechtotext::v1::{
    recognition_config::AudioEncoding, RecognitionConfig, StreamingRecognitionConfig,
};

use log::*;
use std::env;
use std::fs::{self, File};
use std::io::Read;

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();
    info!("recognizer example");

    let credentials = fs::read_to_string("/tmp/cred.json").unwrap();
    let streaming_config = StreamingRecognitionConfig {
        config: Some(RecognitionConfig {
            encoding: AudioEncoding::Linear16 as i32,
            sample_rate_hertz: 8000,
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

    let stream = recognizer.streaming_recognize().await;
    pin_mut!(stream); // needed for iteration

    tokio::spawn(async move {
        let mut file = File::open("/tmp/hello_rust_8.wav").unwrap();
        let chunk_size = 1024;

        loop {
            let mut chunk = Vec::with_capacity(chunk_size);
            let n = file
                .by_ref()
                .take(chunk_size as u64)
                .read_to_end(&mut chunk)
                .unwrap();
            if n == 0 {
                break;
            }

            let streaming_request = Recognizer::streaming_request_from_bytes(chunk);

            audio_sender.send(streaming_request).await.unwrap();

            if n < chunk_size {
                break;
            }
        }
    });

    while let Some(val) = stream.next().await {
        info!("recognition result {:?}", val);
    }
}
