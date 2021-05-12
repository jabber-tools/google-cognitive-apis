use google_cognitive_apis::api::grpc::google::cloud::speechtotext::v1::{
    recognition_config::AudioEncoding, RecognitionConfig, StreamingRecognitionConfig,
};
use google_cognitive_apis::speechtotext::recognizer::Recognizer;

use log::*;
use std::env;
use std::fs::{self, File};
use std::io::Read;

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();
    info!("streaming recognizer example #2");

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

    let mut recognizer = Recognizer::create_streaming_recognizer(credentials, streaming_config, None)
        .await
        .unwrap();

    let audio_sender = recognizer.get_audio_sink().unwrap();

    let mut result_receiver = recognizer.get_streaming_result_receiver(None);

    tokio::spawn(async move {
        let recognition_result = recognizer.streaming_recognize_2().await;

        match recognition_result {
            Err(err) => error!("streaming_recognize_2 error {:?}", err),
            Ok(_) => info!("streaming_recognize_2 ok!"),
        }
    });

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

    while let Some(reco_result) = result_receiver.recv().await {
        info!("recognition result {:?}", reco_result);
    }
}
