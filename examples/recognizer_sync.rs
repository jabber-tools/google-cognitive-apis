use google_cognitive_apis::api::grpc::google::cloud::speechtotext::v1::{
    recognition_audio::AudioSource, recognition_config::AudioEncoding, RecognitionAudio,
    RecognitionConfig, RecognizeRequest,
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
    info!("synchronous recognizer example");

    let credentials = fs::read_to_string("/tmp/cred.json").unwrap();

    let mut file = File::open("/tmp/hello_rust_8.wav").unwrap();
    let mut audio_bytes = Vec::new();
    file.read_to_end(&mut audio_bytes).unwrap();

    let recognize_request = RecognizeRequest {
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
        audio: Some(RecognitionAudio {
            audio_source: Some(AudioSource::Content(audio_bytes)),
        }),
    };

    let mut recognizer = Recognizer::create_synchronous_recognizer(credentials.clone())
        .await
        .unwrap();

    match recognizer.recognize(recognize_request).await {
        Err(err) => {
            error!("recognize error {:?}", err);
        }
        Ok(recognize_response) => {
            info!("recognize_response {:?}", recognize_response);
        }
    }
}
