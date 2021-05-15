pub const CERTIFICATES: &[u8] = include_bytes!("../res/certs/roots.pem");

pub mod api;
pub mod common;
pub mod dialogflow;
pub mod errors;
pub mod speechtotext;
