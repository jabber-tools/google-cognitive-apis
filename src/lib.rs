//! # google-cognitive-apis
//!
//! Library wrapping Google speech-to-text, text-to-speech and dialogflow APIs.
//! Provides high level API layer wrapping the underlying complexity of GRPC.
//!
pub const CERTIFICATES: &[u8] = include_bytes!("../res/certs/roots.pem");

pub mod api;
pub mod common;
pub mod dialogflow;
pub mod errors;
pub mod speechtotext;
pub mod texttospeech;
