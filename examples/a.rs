use google_cognitive_apis::api::grpc::google::cloud::speechtotext::v1::{
    recognition_audio::AudioSource, recognition_config::AudioEncoding, LongRunningRecognizeRequest,
    RecognitionAudio, RecognitionConfig,
};
use google_cognitive_apis::api::grpc::google::longrunning::{
    operations_client::OperationsClient, GetOperationRequest, WaitOperationRequest,
};
use google_cognitive_apis::speechtotext::recognizer::Recognizer;
use google_cognitive_apis::CERTIFICATES;
use gouth::Builder;
use log::*;
use std::env;
use std::fs::{self, File};
use std::io::Read;
use std::sync::Arc;

use tonic::{
    metadata::MetadataValue,
    transport::{Certificate, Channel, ClientTlsConfig},
};

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();
    info!("recognizer long running example");

    let wait_req = WaitOperationRequest {
        name: "6542939144986125284".to_string(),
        timeout: None,
    };

    let gop_req = GetOperationRequest {
        name: "4220841925028000729".to_string(),
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

    let credentials = fs::read_to_string("/tmp/cred.json").unwrap();

    let token = Builder::new()
        .scopes(&[
            "https://www.googleapis.com/auth/service.management",
            "https://www.googleapis.com/auth/cloud-platform",
        ])
        .json(credentials)
        .build()
        .unwrap();

    let token_header_val: Arc<String> = token.header_value().unwrap();

    info!("token is  {}", token_header_val);

    let mut oper_client =
        OperationsClient::with_interceptor(channel, move |mut req: tonic::Request<()>| {
            let meta = MetadataValue::from_str(&token_header_val).unwrap();
            req.metadata_mut().insert("authorization", meta);
            Ok(req)
        });

    let final_result = oper_client.wait_operation(wait_req).await.unwrap();
    info!("final_result ok {:?}", final_result);
    // let fff = oper_client.get_operation(gop_req).await.unwrap();
    // info!("fff ok {:?}", fff);
}
