use futures_util::pin_mut;
use futures_util::stream::StreamExt;
use google_cognitive_apis::speechtotext::recognizer::Recognizer;
use log::*;
use std::env;
use std::fs;

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();
    info!("recognizer example");
    let credentials = fs::read_to_string("/tmp/cred.json").unwrap();
    let mut recognizer = Recognizer::new(credentials).await.unwrap();

    let audio_sender = recognizer.get_audio_sink();

    let s = recognizer.streaming_recognize().await;
    pin_mut!(s); // needed for iteration

    tokio::spawn(async move {
        audio_sender.send(vec![1, 2]).await.unwrap();
        audio_sender.send(vec![3, 4]).await.unwrap();
        audio_sender.send(vec![5, 6]).await.unwrap();
        audio_sender.send(vec![7, 8]).await.unwrap();
    });

    while let Some(val) = s.next().await {
        info!("got value {:?}", val);
    }
}
