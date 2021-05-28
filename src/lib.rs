//! # google-cognitive-apis
//!
//! Asynchronous Rust bindings for Google Cloud Platform cognitive gRPC APIs.
//! Provides high level interfaces wrapping complexity of low-level GRPC implementation.

/// Google trusted certificates. Used by underlying Rust/Tonic gRPC services when establishing gRPC connection/channel.
pub const CERTIFICATES: &[u8] = include_bytes!("../res/certs/roots.pem");

pub mod api;
pub mod common;
pub mod dialogflow;
pub mod errors;
pub mod speechtotext;
pub mod texttospeech;
