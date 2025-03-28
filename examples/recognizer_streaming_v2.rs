use google_cognitive_apis::api::grpc::google::cloud::speechtotext::v2::{RecognitionConfig, RecognizeRequest, StreamingRecognitionConfig};
use google_cognitive_apis::speechtotext::recognizer::Recognizer;

use google_cognitive_apis::api::grpc::google::cloud::speechtotext::v2::recognition_config::DecodingConfig;
use google_cognitive_apis::api::grpc::google::cloud::speechtotext::v2::recognize_request::AudioSource;
use google_cognitive_apis::speechtotext::recognizer_v2::RecognizerV2;
use log::*;
use std::env;
use std::fs::{self, File};
use std::io::Read;

const PROJECT_ID: &str  = "...";
const RECOGNIZER: &str  = "...";

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();
    info!("streaming recognizer example");

    let recognizer_string = format!("projects/{PROJECT_ID}/locations/global/recognizers/{RECOGNIZER}");
    println!("Using recognizer {}", recognizer_string);
    let credentials = fs::read_to_string("cred.json").unwrap();

    let streaming_config = StreamingRecognitionConfig {
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
        streaming_features: None,
    };

    let mut recognizer =
        RecognizerV2::create_streaming_recognizer(credentials, streaming_config, None, recognizer_string.clone())
            .await
            .unwrap();

    // Make sure to use take_audio_sink, not get_audio_sink here! get_audio_sink is cloning the sender
    // contained in recognizer client whereas take_audio_sink will take the sender/sink out of the wrapping option.
    // Thus once tokio task pushing the audio data into Google Speech-to-text API will push all the data, sender will be
    // dropped signaling no more data will be sent. Only then Speech-to-text API will stream back the final
    // response (with is_final attribute set to true).
    // If get_audio_sink is used instead following error occurs ( give it a try:-) ):
    // Audio Timeout Error: Long duration elapsed without audio. Audio should be sent close to real time.
    // See also method drop_audio_sink
    let audio_sender = recognizer.take_audio_sink().unwrap();

    let mut result_receiver = recognizer.get_streaming_result_receiver(None);

    tokio::spawn(async move {
        let recognition_result = recognizer.streaming_recognize().await;

        match recognition_result {
            Err(err) => error!("streaming_recognize error {:?}", err),
            Ok(_) => info!("streaming_recognize ok!"),
        }
    });

    tokio::spawn(async move {
        let mut file = File::open("test_data.wav").unwrap();
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

            let streaming_request = RecognizerV2::streaming_request_from_bytes(chunk, recognizer_string.clone());

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
