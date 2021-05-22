use google_cognitive_apis::api::grpc::google::cloud::dialogflow::v2beta1::{
    query_input::Input, InputAudioConfig, QueryInput, StreamingDetectIntentRequest,
};
use google_cognitive_apis::dialogflow::sessions_client::SessionsClient;

use log::*;
use std::env;
use std::fs::{self, File};
use std::io::Read;

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();
    info!("sessions_client_streaming_detect_intent example");

    let credentials = fs::read_to_string("/tmp/gdf-cred.json").unwrap();

    let guid = "10db5977-7f28-4a57-92fb-88459ff8c239";
    let session_id = SessionsClient::get_session_string("<<gcp project id>>", guid);

    #[allow(deprecated)]
    let streaming_detect_intent_req = StreamingDetectIntentRequest {
        session: session_id.to_owned(),
        query_params: None,
        query_input: Some(QueryInput {
            input: Some(Input::AudioConfig(InputAudioConfig {
                audio_encoding: 1, // linear16
                sample_rate_hertz: 8000,
                language_code: "en".to_owned(),
                enable_word_info: false,
                phrase_hints: vec![],
                speech_contexts: vec![],
                model: "".to_string(),
                model_variant: 0,
                single_utterance: false,
                disable_no_speech_recognized_event: false,
            })),
        }),
        single_utterance: false,
        output_audio_config: None,
        output_audio_config_mask: None,
        input_audio: vec![],
    };

    let mut sessions_client =
        SessionsClient::create_async(credentials, streaming_detect_intent_req, None)
            .await
            .unwrap();

    let audio_sender = sessions_client.get_audio_sink().unwrap();

    let mut result_receiver = sessions_client.get_streaming_result_receiver(None);

    tokio::spawn(async move {
        let recognition_result = sessions_client.streaming_detect_intent().await;

        match recognition_result {
            Err(err) => error!("streaming_detect_intent error {:?}", err),
            Ok(_) => info!("streaming_detect_intent ok!"),
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

            let streaming_request =
                SessionsClient::streaming_request_from_bytes(session_id.to_string(), chunk);

            audio_sender.send(streaming_request).await.unwrap();

            if n < chunk_size {
                // At this moment client should send half-close message to server
                // indicating no more data will be sent. Google API streaming_detect_intent
                // only then initiates NLP analysis and will start producing responses
                // Right now half-close is not implemented by this library
                // Details here:
                // https://grpc.github.io/grpc/core/md_doc_core_transport_explainer.html
                // https://cloud.google.com/dialogflow/es/docs/how/detect-intent-stream#detect-intent-stream-go
                break;
            }
        }
    });

    while let Some(reco_result) = result_receiver.recv().await {
        info!("recognition result {:?}", reco_result);
    }
}
