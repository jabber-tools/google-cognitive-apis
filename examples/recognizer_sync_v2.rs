use google_cognitive_apis::api::grpc::google::cloud::speechtotext::v2::{RecognitionConfig, RecognizeRequest, TranslationConfig};
use google_cognitive_apis::speechtotext::recognizer::Recognizer;
use log::*;
use std::env;
use std::fs::{self, File};
use std::io::Read;
use google_cognitive_apis::api::grpc::google::cloud::speechtotext::v2::recognition_config::DecodingConfig;
use google_cognitive_apis::api::grpc::google::cloud::speechtotext::v2::recognition_config::DecodingConfig::AutoDecodingConfig;
use google_cognitive_apis::api::grpc::google::cloud::speechtotext::v2::recognize_request::AudioSource;
use google_cognitive_apis::api::grpc::google::cloud::speechtotext::v2::speech_client::SpeechClient;
use google_cognitive_apis::speechtotext::recognizer_v2::RecognizerV2;

const PROJECT_ID: &str  = "...";
const RECOGNIZER: &str  = "...";

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();
    info!("synchronous recognizer example");

    let credentials = fs::read_to_string("cred.json").unwrap();

    let mut file = File::open("test_data.wav").unwrap();
    let mut audio_bytes = Vec::new();
    file.read_to_end(&mut audio_bytes).unwrap();

    let recognizer_string = format!("projects/{PROJECT_ID}/locations/global/recognizers/{RECOGNIZER}");
    println!("Using recognizer {}", recognizer_string);

    let recognize_request = RecognizeRequest {
        recognizer: recognizer_string,
        config: Some(RecognitionConfig {
            model: "".to_string(),
            language_codes: vec!["cs-CZ".to_string()],
            features: None,
            adaptation: None,
            transcript_normalization: None,
            translation_config: None,
            decoding_config: Some(DecodingConfig::AutoDecodingConfig{ 0: Default::default() }),
        }),
        config_mask: None,
        audio_source: Some(AudioSource::Content(audio_bytes)),
    };

    let mut recognizer = RecognizerV2::create_synchronous_recognizer(credentials.clone())
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
