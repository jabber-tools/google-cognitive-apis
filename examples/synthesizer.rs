use google_cognitive_apis::api::grpc::google::cloud::texttospeech::v1::{
    synthesis_input::InputSource, AudioConfig, AudioEncoding, ListVoicesRequest,
    ListVoicesResponse, SynthesisInput, SynthesizeSpeechRequest, VoiceSelectionParams,
};
use google_cognitive_apis::texttospeech::synthesizer::Synthesizer;
use log::*;
use std::env;
use std::fs::{self, File};
use std::io::prelude::*;

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();
    info!("synthetizer example");

    let credentials = fs::read_to_string("/tmp/cred.json").unwrap();

    let mut synthesizer = Synthesizer::create(credentials).await.unwrap();

    let voices_resp: ListVoicesResponse = synthesizer
        .list_voices(ListVoicesRequest {
            language_code: "en".to_string(),
        })
        .await
        .unwrap();

    let voice1 = &voices_resp.voices[0];

    info!("voices {:#?}", voices_resp);

    let response = synthesizer
        .synthesize_speech(SynthesizeSpeechRequest {
            input: Some(SynthesisInput {
                input_source: Some(InputSource::Text(
                    "Let's do some text to speech!".to_string(),
                )),
            }),
            voice: Some(VoiceSelectionParams {
                language_code: "en".to_string(),
                name: voice1.name.to_owned(),
                ssml_gender: voice1.ssml_gender,
            }),
            audio_config: Some(AudioConfig {
                audio_encoding: AudioEncoding::Linear16 as i32,
                speaking_rate: 1f64,
                pitch: 0f64,
                volume_gain_db: 0f64,
                sample_rate_hertz: 16000,
                effects_profile_id: vec![],
            }),
        })
        .await
        .unwrap();

    let mut file = File::create("/tmp/synthetized_audio.wav").unwrap();

    file.write_all(&response.audio_content[..]).unwrap();
}
