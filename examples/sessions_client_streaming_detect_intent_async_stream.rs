use futures_util::pin_mut;
use futures_util::stream::StreamExt;
use google_cognitive_apis::api::grpc::google::cloud::dialogflow::v2beta1::{
    query_input::Input, InputAudioConfig, QueryInput, StreamingDetectIntentRequest,
};
use google_cognitive_apis::dialogflow::sessions_client_streaming::SessionsClient;

use log::*;
use std::env;
use std::fs::{self, File};
use std::io::Read;

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();
    info!("sessions_client_streaming_detect_intent_async_stream example");

    let credentials = fs::read_to_string("/tmp/cred.json").unwrap();

    let guid = "8d58ca66-8977-4d14-8664-c48388b283b8";
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
        SessionsClient::create(credentials, streaming_detect_intent_req, None)
            .await
            .unwrap();

    // Make sure to use take_audio_sink, not get_audio_sink here! get_audio_sink is cloning the sender
    // contained in session client whereas take_audio_sink will take the sender/sink out of the wrapping option.
    // Thus once tokio task pushing the audio data into Google Dialogflow will push all the data, sender will be
    // dropped signaling no more data will be sent. Only then Dialogflow will stream back the final
    // response (with detected intent, fulfillment messages, etc.).
    // If get_audio_sink is used instead following error occurs ( give it a try:-) ):
    // Audio Timeout Error: Long duration elapsed without audio. Audio should be sent close to real time.
    // See also method drop_audio_sink
    let audio_sender = sessions_client.take_audio_sink().unwrap();

    let stream = sessions_client.streaming_detect_intent_async_stream().await;
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

    while let Some(val) = stream.next().await {
        info!("recognition result {:?}", val);
    }
}
