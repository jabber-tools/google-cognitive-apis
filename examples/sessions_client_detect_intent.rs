use google_cognitive_apis::api::grpc::google::cloud::dialogflow::v2beta1::{
    query_input::Input, DetectIntentRequest, QueryInput, TextInput,
};
use google_cognitive_apis::dialogflow::sessions_client::SessionsClient;
use log::*;
use std::env;
use std::fs;

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();
    info!("synchronous recognizer example");

    let credentials = fs::read_to_string("/tmp/cred.json").unwrap();

    let guid = "8d58ca66-8977-4d14-8664-c48388b283b8";
    let session_id = SessionsClient::get_session_string("<<gcp project id>>", guid);

    let request = DetectIntentRequest {
        session: session_id,
        query_params: None,
        query_input: Some(QueryInput {
            input: Some(Input::Text(TextInput {
                text: "Hi there".to_owned(),
                language_code: "en".to_owned(),
            })),
        }),
        output_audio_config: None,
        output_audio_config_mask: None,
        input_audio: vec![],
    };

    let mut sessions_client = SessionsClient::create(credentials).await.unwrap();

    match sessions_client.detect_intent(request).await {
        Err(err) => {
            error!("detect_intent error {:?}", err);
        }
        Ok(detect_intent_response) => {
            info!("detect_intent_response {:?}", detect_intent_response);
        }
    }
}
