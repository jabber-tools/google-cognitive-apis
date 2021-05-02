use google_cognitive_apis::api::grpc::google::cloud::speechtotext::v1::{
    recognition_audio::AudioSource, recognition_config::AudioEncoding, LongRunningRecognizeRequest,
    RecognitionAudio, RecognitionConfig,
};
use google_cognitive_apis::api::grpc::google::longrunning::{
    operation::Result as LROResult, operations_client::OperationsClient, GetOperationRequest,
    Operation, WaitOperationRequest,
};
use google_cognitive_apis::speechtotext::recognizer::Recognizer;
use google_cognitive_apis::CERTIFICATES;
use gouth::Builder;
use log::*;
use std::env;
use std::fs::{self, File};
use std::io::Read;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use tonic::Response as TonicResponse;

use tonic::{
    metadata::MetadataValue,
    transport::{Certificate, Channel, ClientTlsConfig},
};

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();
    info!("recognizer long running example");

    let credentials = fs::read_to_string("/tmp/cred.json").unwrap();

    let mut file = File::open("/tmp/hello_rust_8.wav").unwrap();
    let mut audio_bytes = Vec::new();
    file.read_to_end(&mut audio_bytes).unwrap();

    let long_running_request = LongRunningRecognizeRequest {
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

    let mut recognizer = Recognizer::new_for_long_running(credentials.clone())
        .await
        .unwrap();
    let token = Builder::new().json(credentials).build().unwrap();

    let token_header_val: Arc<String> = token.header_value().unwrap();

    match recognizer
        .long_running_recognize(long_running_request)
        .await
    {
        Err(err) => {
            error!("long_running_recognize error {:?}", err);
        }
        Ok(grpc_response) => {
            let long_running_operation = grpc_response.into_inner();
            info!("long_running_operation ok {:?}", long_running_operation);

            let long_running_operation_name = long_running_operation.name;

            let _wait_req = WaitOperationRequest {
                name: long_running_operation_name.clone(),
                timeout: None,
            };

            let gop_req = GetOperationRequest {
                name: long_running_operation_name.clone(),
            };

            let tls_config = ClientTlsConfig::new()
                .ca_certificate(Certificate::from_pem(CERTIFICATES))
                .domain_name("speech.googleapis.com");

            let channel = Channel::from_static("https://speech.googleapis.com")
                .tls_config(tls_config.clone())
                .unwrap()
                //.timeout(std::time::Duration::from_secs(2))
                .connect()
                .await
                .unwrap();

            let mut oper_client =
                OperationsClient::with_interceptor(channel, move |mut req: tonic::Request<()>| {
                    let meta = MetadataValue::from_str(&token_header_val).unwrap();
                    req.metadata_mut().insert("authorization", meta);
                    Ok(req)
                });

            // not working
            // let _final_result = oper_client.wait_operation(_wait_req).await.unwrap();

            // wait 5 sec so that we have some result in long running operation
            // retrieved by get_operation call bellow
            sleep(Duration::from_millis(5000)).await;

            let tonic_response: TonicResponse<Operation> =
                oper_client.get_operation(gop_req).await.unwrap();
            info!("tonic_response ok {:?}", tonic_response);

            let lro_result: LROResult = tonic_response.into_inner().result.unwrap();
            info!("lro_result {:#?}", lro_result);

            match lro_result {
                LROResult::Error(lro_err) => {
                    error!("LROResult::Error {:#?}", lro_err);
                }
                LROResult::Response(lro_resp) => {
                    info!("LROResult::Response {:#?}", lro_resp);
                }
            }
        }
    }
}
