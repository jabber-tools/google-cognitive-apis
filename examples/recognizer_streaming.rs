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
    info!("streaming recognizer example");

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

    let mut recognizer =
        Recognizer::create_streaming_recognizer(credentials, streaming_config, None)
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
