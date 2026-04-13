fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=res/proto/google/cloud/dialogflow/v2/session.proto");
    println!("cargo:rerun-if-changed=res/proto/google/cloud/dialogflow/v2beta1/session.proto");
    println!("cargo:rerun-if-changed=res/proto/google/cloud/dialogflow/cx/v3/session.proto");
    println!("cargo:rerun-if-changed=res/proto/google/cloud/dialogflow/cx/v3beta1/session.proto");

    println!("cargo:rerun-if-changed=res/proto/google/cloud/speech/v1/cloud_speech.proto");
    println!("cargo:rerun-if-changed=res/proto/google/cloud/speech/v1p1beta1/cloud_speech.proto");
    println!("cargo:rerun-if-changed=res/proto/google/cloud/speech/v2/cloud_speech.proto");

    println!("cargo:rerun-if-changed=res/proto/google/cloud/texttospeech/v1/cloud_tts.proto");
    println!("cargo:rerun-if-changed=res/proto/google/cloud/texttospeech/v1beta1/cloud_tts.proto");

    let skip_proto_compilation = std::env::var("SKIP_PROTO_FILES_COMPILATION")
        .map(|v| v == "1")
        .unwrap_or(false);
    if skip_proto_compilation {
        return Ok(());
    }

    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        .out_dir("src/grpc_stubs")
        .compile(
            &[
                "res/proto/google/cloud/dialogflow/v2/session.proto",
                "res/proto/google/cloud/dialogflow/v2beta1/session.proto",
                "res/proto/google/cloud/speech/v1/cloud_speech.proto",
                "res/proto/google/cloud/speech/v1p1beta1/cloud_speech.proto",
                "res/proto/google/cloud/speech/v2/cloud_speech.proto",
                "res/proto/google/cloud/texttospeech/v1/cloud_tts.proto",
                "res/proto/google/cloud/texttospeech/v1beta1/cloud_tts.proto",
            ],
            &["res/proto"],
        )
        .unwrap();

    // building dialogflow cx separately as
    // it needs to be in nested folder
    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        .out_dir("src/grpc_stubs/dialogflow_cx")
        .compile(
            &[
                "res/proto/google/cloud/dialogflow/cx/v3/session.proto",
                "res/proto/google/cloud/dialogflow/cx/v3beta1/session.proto",
            ],
            &["res/proto"],
        )
        .unwrap();

    Ok(())
}
