///
/// Not yet implemented!
///
#[derive(Debug)]
pub struct TTSRequest {
    /// text or ssml to synthetize
    pub text: String,
}

#[derive(Debug)]
pub struct TTSResponse {
    /// synthetized audio data
    pub audio_base64: String,
}

