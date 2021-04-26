/// tests whether build has created all nested modules properly
/// we are trying to import sample modules to check if all is compiled properly
#[test]
#[allow(unused_imports)]
fn test_modules() {
    use google_cognitive_apis::api::grpc::google::cloud::dialogflow::cx::v3::SpeechWordInfo;
    use google_cognitive_apis::api::grpc::google::cloud::dialogflow::cx::v3beta1::SpeechWordInfo as SpeechWordInfoBeta;
    use google_cognitive_apis::api::grpc::google::cloud::dialogflow::v2::SpeechContext;
    use google_cognitive_apis::api::grpc::google::cloud::dialogflow::v2beta1::SpeechContext as SpeechContextBeta;
    assert_eq!(2 + 2, 4);
}
