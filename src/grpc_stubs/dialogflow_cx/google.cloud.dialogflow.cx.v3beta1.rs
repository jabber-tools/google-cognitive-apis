/// Information for a word recognized by the speech recognizer.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpeechWordInfo {
    /// The word this info is for.
    #[prost(string, tag = "3")]
    pub word: std::string::String,
    /// Time offset relative to the beginning of the audio that corresponds to the
    /// start of the spoken word. This is an experimental feature and the accuracy
    /// of the time offset can vary.
    #[prost(message, optional, tag = "1")]
    pub start_offset: ::std::option::Option<::prost_types::Duration>,
    /// Time offset relative to the beginning of the audio that corresponds to the
    /// end of the spoken word. This is an experimental feature and the accuracy of
    /// the time offset can vary.
    #[prost(message, optional, tag = "2")]
    pub end_offset: ::std::option::Option<::prost_types::Duration>,
    /// The Speech confidence between 0.0 and 1.0 for this word. A higher number
    /// indicates an estimated greater likelihood that the recognized word is
    /// correct. The default of 0.0 is a sentinel value indicating that confidence
    /// was not set.
    ///
    /// This field is not guaranteed to be fully stable over time for the same
    /// audio input. Users should also not rely on it to always be provided.
    #[prost(float, tag = "4")]
    pub confidence: f32,
}
/// Instructs the speech recognizer on how to process the audio content.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InputAudioConfig {
    /// Required. Audio encoding of the audio content to process.
    #[prost(enumeration = "AudioEncoding", tag = "1")]
    pub audio_encoding: i32,
    /// Sample rate (in Hertz) of the audio content sent in the query.
    /// Refer to
    /// [Cloud Speech API
    /// documentation](https://cloud.google.com/speech-to-text/docs/basics) for
    /// more details.
    #[prost(int32, tag = "2")]
    pub sample_rate_hertz: i32,
    /// Optional. If `true`, Dialogflow returns [SpeechWordInfo][google.cloud.dialogflow.cx.v3beta1.SpeechWordInfo] in
    /// [StreamingRecognitionResult][google.cloud.dialogflow.cx.v3beta1.StreamingRecognitionResult] with information about the recognized speech
    /// words, e.g. start and end time offsets. If false or unspecified, Speech
    /// doesn't return any word-level information.
    #[prost(bool, tag = "13")]
    pub enable_word_info: bool,
    /// Optional. A list of strings containing words and phrases that the speech
    /// recognizer should recognize with higher likelihood.
    ///
    /// See [the Cloud Speech
    /// documentation](https://cloud.google.com/speech-to-text/docs/basics#phrase-hints)
    /// for more details.
    #[prost(string, repeated, tag = "4")]
    pub phrase_hints: ::std::vec::Vec<std::string::String>,
    /// Optional. Which Speech model to select for the given request. Select the
    /// model best suited to your domain to get best results. If a model is not
    /// explicitly specified, then we auto-select a model based on the parameters
    /// in the InputAudioConfig.
    /// If enhanced speech model is enabled for the agent and an enhanced
    /// version of the specified model for the language does not exist, then the
    /// speech is recognized using the standard version of the specified model.
    /// Refer to
    /// [Cloud Speech API
    /// documentation](https://cloud.google.com/speech-to-text/docs/basics#select-model)
    /// for more details.
    #[prost(string, tag = "7")]
    pub model: std::string::String,
    /// Optional. Which variant of the [Speech model][google.cloud.dialogflow.cx.v3beta1.InputAudioConfig.model] to use.
    #[prost(enumeration = "SpeechModelVariant", tag = "10")]
    pub model_variant: i32,
    /// Optional. If `false` (default), recognition does not cease until the
    /// client closes the stream.
    /// If `true`, the recognizer will detect a single spoken utterance in input
    /// audio. Recognition ceases when it detects the audio's voice has
    /// stopped or paused. In this case, once a detected intent is received, the
    /// client should close the stream and start a new request with a new stream as
    /// needed.
    /// Note: This setting is relevant only for streaming methods.
    #[prost(bool, tag = "8")]
    pub single_utterance: bool,
}
/// Description of which voice to use for speech synthesis.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VoiceSelectionParams {
    /// Optional. The name of the voice. If not set, the service will choose a
    /// voice based on the other parameters such as language_code and
    /// [ssml_gender][google.cloud.dialogflow.cx.v3beta1.VoiceSelectionParams.ssml_gender].
    ///
    /// For the list of available voices, please refer to [Supported voices and
    /// languages](https://cloud.google.com/text-to-speech/docs/voices).
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Optional. The preferred gender of the voice. If not set, the service will
    /// choose a voice based on the other parameters such as language_code and
    /// [name][google.cloud.dialogflow.cx.v3beta1.VoiceSelectionParams.name]. Note that this is only a preference, not requirement. If a
    /// voice of the appropriate gender is not available, the synthesizer should
    /// substitute a voice with a different gender rather than failing the request.
    #[prost(enumeration = "SsmlVoiceGender", tag = "2")]
    pub ssml_gender: i32,
}
/// Configuration of how speech should be synthesized.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SynthesizeSpeechConfig {
    /// Optional. Speaking rate/speed, in the range [0.25, 4.0]. 1.0 is the normal
    /// native speed supported by the specific voice. 2.0 is twice as fast, and
    /// 0.5 is half as fast. If unset(0.0), defaults to the native 1.0 speed. Any
    /// other values < 0.25 or > 4.0 will return an error.
    #[prost(double, tag = "1")]
    pub speaking_rate: f64,
    /// Optional. Speaking pitch, in the range [-20.0, 20.0]. 20 means increase 20
    /// semitones from the original pitch. -20 means decrease 20 semitones from the
    /// original pitch.
    #[prost(double, tag = "2")]
    pub pitch: f64,
    /// Optional. Volume gain (in dB) of the normal native volume supported by the
    /// specific voice, in the range [-96.0, 16.0]. If unset, or set to a value of
    /// 0.0 (dB), will play at normal native signal amplitude. A value of -6.0 (dB)
    /// will play at approximately half the amplitude of the normal native signal
    /// amplitude. A value of +6.0 (dB) will play at approximately twice the
    /// amplitude of the normal native signal amplitude. We strongly recommend not
    /// to exceed +10 (dB) as there's usually no effective increase in loudness for
    /// any value greater than that.
    #[prost(double, tag = "3")]
    pub volume_gain_db: f64,
    /// Optional. An identifier which selects 'audio effects' profiles that are
    /// applied on (post synthesized) text to speech. Effects are applied on top of
    /// each other in the order they are given.
    #[prost(string, repeated, tag = "5")]
    pub effects_profile_id: ::std::vec::Vec<std::string::String>,
    /// Optional. The desired voice of the synthesized audio.
    #[prost(message, optional, tag = "4")]
    pub voice: ::std::option::Option<VoiceSelectionParams>,
}
/// Instructs the speech synthesizer how to generate the output audio content.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OutputAudioConfig {
    /// Required. Audio encoding of the synthesized audio content.
    #[prost(enumeration = "OutputAudioEncoding", tag = "1")]
    pub audio_encoding: i32,
    /// Optional. The synthesis sample rate (in hertz) for this audio. If not
    /// provided, then the synthesizer will use the default sample rate based on
    /// the audio encoding. If this is different from the voice's natural sample
    /// rate, then the synthesizer will honor this request by converting to the
    /// desired sample rate (which might result in worse audio quality).
    #[prost(int32, tag = "2")]
    pub sample_rate_hertz: i32,
    /// Optional. Configuration of how speech should be synthesized.
    #[prost(message, optional, tag = "3")]
    pub synthesize_speech_config: ::std::option::Option<SynthesizeSpeechConfig>,
}
/// Audio encoding of the audio content sent in the conversational query request.
/// Refer to the
/// [Cloud Speech API
/// documentation](https://cloud.google.com/speech-to-text/docs/basics) for more
/// details.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AudioEncoding {
    /// Not specified.
    Unspecified = 0,
    /// Uncompressed 16-bit signed little-endian samples (Linear PCM).
    Linear16 = 1,
    /// [`FLAC`](https://xiph.org/flac/documentation.html) (Free Lossless Audio
    /// Codec) is the recommended encoding because it is lossless (therefore
    /// recognition is not compromised) and requires only about half the
    /// bandwidth of `LINEAR16`. `FLAC` stream encoding supports 16-bit and
    /// 24-bit samples, however, not all fields in `STREAMINFO` are supported.
    Flac = 2,
    /// 8-bit samples that compand 14-bit audio samples using G.711 PCMU/mu-law.
    Mulaw = 3,
    /// Adaptive Multi-Rate Narrowband codec. `sample_rate_hertz` must be 8000.
    Amr = 4,
    /// Adaptive Multi-Rate Wideband codec. `sample_rate_hertz` must be 16000.
    AmrWb = 5,
    /// Opus encoded audio frames in Ogg container
    /// ([OggOpus](https://wiki.xiph.org/OggOpus)).
    /// `sample_rate_hertz` must be 16000.
    OggOpus = 6,
    /// Although the use of lossy encodings is not recommended, if a very low
    /// bitrate encoding is required, `OGG_OPUS` is highly preferred over
    /// Speex encoding. The [Speex](https://speex.org/) encoding supported by
    /// Dialogflow API has a header byte in each block, as in MIME type
    /// `audio/x-speex-with-header-byte`.
    /// It is a variant of the RTP Speex encoding defined in
    /// [RFC 5574](https://tools.ietf.org/html/rfc5574).
    /// The stream is a sequence of blocks, one block per RTP packet. Each block
    /// starts with a byte containing the length of the block, in bytes, followed
    /// by one or more frames of Speex data, padded to an integral number of
    /// bytes (octets) as specified in RFC 5574. In other words, each RTP header
    /// is replaced with a single byte containing the block length. Only Speex
    /// wideband is supported. `sample_rate_hertz` must be 16000.
    SpeexWithHeaderByte = 7,
}
/// Variant of the specified [Speech model][google.cloud.dialogflow.cx.v3beta1.InputAudioConfig.model] to use.
///
/// See the [Cloud Speech
/// documentation](https://cloud.google.com/speech-to-text/docs/enhanced-models)
/// for which models have different variants. For example, the "phone_call" model
/// has both a standard and an enhanced variant. When you use an enhanced model,
/// you will generally receive higher quality results than for a standard model.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SpeechModelVariant {
    /// No model variant specified. In this case Dialogflow defaults to
    /// USE_BEST_AVAILABLE.
    Unspecified = 0,
    /// Use the best available variant of the [Speech
    /// model][InputAudioConfig.model] that the caller is eligible for.
    ///
    /// Please see the [Dialogflow
    /// docs](https://cloud.google.com/dialogflow/docs/data-logging) for
    /// how to make your project eligible for enhanced models.
    UseBestAvailable = 1,
    /// Use standard model variant even if an enhanced model is available.  See the
    /// [Cloud Speech
    /// documentation](https://cloud.google.com/speech-to-text/docs/enhanced-models)
    /// for details about enhanced models.
    UseStandard = 2,
    /// Use an enhanced model variant:
    ///
    /// * If an enhanced variant does not exist for the given
    ///   [model][google.cloud.dialogflow.cx.v3beta1.InputAudioConfig.model] and request language, Dialogflow falls
    ///   back to the standard variant.
    ///
    ///   The [Cloud Speech
    ///   documentation](https://cloud.google.com/speech-to-text/docs/enhanced-models)
    ///   describes which models have enhanced variants.
    ///
    /// * If the API caller isn't eligible for enhanced models, Dialogflow returns
    ///   an error.  Please see the [Dialogflow
    ///   docs](https://cloud.google.com/dialogflow/docs/data-logging)
    ///   for how to make your project eligible.
    UseEnhanced = 3,
}
/// Gender of the voice as described in
/// [SSML voice element](https://www.w3.org/TR/speech-synthesis11/#edef_voice).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SsmlVoiceGender {
    /// An unspecified gender, which means that the client doesn't care which
    /// gender the selected voice will have.
    Unspecified = 0,
    /// A male voice.
    Male = 1,
    /// A female voice.
    Female = 2,
    /// A gender-neutral voice.
    Neutral = 3,
}
/// Audio encoding of the output audio format in Text-To-Speech.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OutputAudioEncoding {
    /// Not specified.
    Unspecified = 0,
    /// Uncompressed 16-bit signed little-endian samples (Linear PCM).
    /// Audio content returned as LINEAR16 also contains a WAV header.
    Linear16 = 1,
    /// MP3 audio at 32kbps.
    Mp3 = 2,
    /// MP3 audio at 64kbps.
    Mp364Kbps = 4,
    /// Opus encoded audio wrapped in an ogg container. The result will be a
    /// file which can be played natively on Android, and in browsers (at least
    /// Chrome and Firefox). The quality of the encoding is considerably higher
    /// than MP3 while using approximately the same bitrate.
    OggOpus = 3,
    /// 8-bit samples that compand 14-bit audio samples using G.711 PCMU/mu-law.
    Mulaw = 5,
}
/// Represents a response message that can be returned by a conversational agent.
///
/// Response messages are also used for output audio synthesis. The approach is
/// as follows:
///
/// * If at least one OutputAudioText response is present, then all
///   OutputAudioText responses are linearly concatenated, and the result is used
///   for output audio synthesis.
/// * If the OutputAudioText responses are a mixture of text and SSML, then the
///   concatenated result is treated as SSML; otherwise, the result is treated as
///   either text or SSML as appropriate. The agent designer should ideally use
///   either text or SSML consistently throughout the bot design.
/// * Otherwise, all Text responses are linearly concatenated, and the result is
///   used for output audio synthesis.
///
/// This approach allows for more sophisticated user experience scenarios, where
/// the text displayed to the user may differ from what is heard.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseMessage {
    /// Required. The rich response message.
    #[prost(
        oneof = "response_message::Message",
        tags = "1, 2, 9, 8, 10, 11, 12, 13"
    )]
    pub message: ::std::option::Option<response_message::Message>,
}
pub mod response_message {
    /// The text response message.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Text {
        /// Required. A collection of text responses.
        #[prost(string, repeated, tag = "1")]
        pub text: ::std::vec::Vec<std::string::String>,
        /// Output only. Whether the playback of this message can be interrupted by the end
        /// user's speech and the client can then starts the next Dialogflow
        /// request.
        #[prost(bool, tag = "2")]
        pub allow_playback_interruption: bool,
    }
    /// Indicates that the conversation should be handed off to a live agent.
    ///
    /// Dialogflow only uses this to determine which conversations were handed off
    /// to a human agent for measurement purposes. What else to do with this signal
    /// is up to you and your handoff procedures.
    ///
    /// You may set this, for example:
    /// * In the [entry_fulfillment][google.cloud.dialogflow.cx.v3beta1.Page.entry_fulfillment] of a [Page][google.cloud.dialogflow.cx.v3beta1.Page] if
    ///   entering the page indicates something went extremely wrong in the
    ///   conversation.
    /// * In a webhook response when you determine that the customer issue can only
    ///   be handled by a human.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LiveAgentHandoff {
        /// Custom metadata for your handoff procedure. Dialogflow doesn't impose
        /// any structure on this.
        #[prost(message, optional, tag = "1")]
        pub metadata: ::std::option::Option<::prost_types::Struct>,
    }
    /// Indicates that the conversation succeeded, i.e., the bot handled the issue
    /// that the customer talked to it about.
    ///
    /// Dialogflow only uses this to determine which conversations should be
    /// counted as successful and doesn't process the metadata in this message in
    /// any way. Note that Dialogflow also considers conversations that get to the
    /// conversation end page as successful even if they don't return
    /// [ConversationSuccess][google.cloud.dialogflow.cx.v3beta1.ResponseMessage.ConversationSuccess].
    ///
    /// You may set this, for example:
    /// * In the [entry_fulfillment][google.cloud.dialogflow.cx.v3beta1.Page.entry_fulfillment] of a [Page][google.cloud.dialogflow.cx.v3beta1.Page] if
    ///   entering the page indicates that the conversation succeeded.
    /// * In a webhook response when you determine that you handled the customer
    ///   issue.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ConversationSuccess {
        /// Custom metadata. Dialogflow doesn't impose any structure on this.
        #[prost(message, optional, tag = "1")]
        pub metadata: ::std::option::Option<::prost_types::Struct>,
    }
    /// A text or ssml response that is preferentially used for TTS output audio
    /// synthesis, as described in the comment on the ResponseMessage message.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct OutputAudioText {
        /// Output only. Whether the playback of this message can be interrupted by the end
        /// user's speech and the client can then starts the next Dialogflow
        /// request.
        #[prost(bool, tag = "3")]
        pub allow_playback_interruption: bool,
        /// The source, which is either plain text or SSML.
        #[prost(oneof = "output_audio_text::Source", tags = "1, 2")]
        pub source: ::std::option::Option<output_audio_text::Source>,
    }
    pub mod output_audio_text {
        /// The source, which is either plain text or SSML.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Source {
            /// The raw text to be synthesized.
            #[prost(string, tag = "1")]
            Text(std::string::String),
            /// The SSML text to be synthesized. For more information, see
            /// [SSML](/speech/text-to-speech/docs/ssml).
            #[prost(string, tag = "2")]
            Ssml(std::string::String),
        }
    }
    /// Indicates that interaction with the Dialogflow agent has ended.
    /// This message is generated by Dialogflow only and not supposed to be
    /// defined by the user.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EndInteraction {}
    /// Specifies an audio clip to be played by the client as part of the response.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PlayAudio {
        /// Required. URI of the audio clip. Dialogflow does not impose any validation on this
        /// value. It is specific to the client that reads it.
        #[prost(string, tag = "1")]
        pub audio_uri: std::string::String,
        /// Output only. Whether the playback of this message can be interrupted by the end
        /// user's speech and the client can then starts the next Dialogflow
        /// request.
        #[prost(bool, tag = "2")]
        pub allow_playback_interruption: bool,
    }
    /// Represents an audio message that is composed of both segments
    /// synthesized from the Dialogflow agent prompts and ones hosted externally
    /// at the specified URIs.
    /// The external URIs are specified via
    /// [play_audio][google.cloud.dialogflow.cx.v3beta1.ResponseMessage.play_audio].
    /// This message is generated by Dialogflow only and not supposed to be
    /// defined by the user.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MixedAudio {
        /// Segments this audio response is composed of.
        #[prost(message, repeated, tag = "1")]
        pub segments: ::std::vec::Vec<mixed_audio::Segment>,
    }
    pub mod mixed_audio {
        /// Represents one segment of audio.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Segment {
            /// Output only. Whether the playback of this segment can be interrupted by the end
            /// user's speech and the client should then start the next Dialogflow
            /// request.
            #[prost(bool, tag = "3")]
            pub allow_playback_interruption: bool,
            /// Content of the segment.
            #[prost(oneof = "segment::Content", tags = "1, 2")]
            pub content: ::std::option::Option<segment::Content>,
        }
        pub mod segment {
            /// Content of the segment.
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum Content {
                /// Raw audio synthesized from the Dialogflow agent's response using
                /// the output config specified in the request.
                #[prost(bytes, tag = "1")]
                Audio(std::vec::Vec<u8>),
                /// Client-specific URI that points to an audio clip accessible to the
                /// client. Dialogflow does not impose any validation on it.
                #[prost(string, tag = "2")]
                Uri(std::string::String),
            }
        }
    }
    /// Required. The rich response message.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        /// Returns a text response.
        #[prost(message, tag = "1")]
        Text(Text),
        /// Returns a response containing a custom, platform-specific payload.
        #[prost(message, tag = "2")]
        Payload(::prost_types::Struct),
        /// Indicates that the conversation succeeded.
        #[prost(message, tag = "9")]
        ConversationSuccess(ConversationSuccess),
        /// A text or ssml response that is preferentially used for TTS output audio
        /// synthesis, as described in the comment on the ResponseMessage message.
        #[prost(message, tag = "8")]
        OutputAudioText(OutputAudioText),
        /// Hands off conversation to a human agent.
        #[prost(message, tag = "10")]
        LiveAgentHandoff(LiveAgentHandoff),
        /// Output only. A signal that indicates the interaction with the Dialogflow agent has
        /// ended.
        /// This message is generated by Dialogflow only when the conversation
        /// reaches `END_SESSION` page. It is not supposed to be defined by the user.
        ///
        /// It's guaranteed that there is at most one such message in each response.
        #[prost(message, tag = "11")]
        EndInteraction(EndInteraction),
        /// Signal that the client should play an audio clip hosted at a
        /// client-specific URI. Dialogflow uses this to construct
        /// [mixed_audio][google.cloud.dialogflow.cx.v3beta1.ResponseMessage.mixed_audio]. However, Dialogflow itself
        /// does not try to read or process the URI in any way.
        #[prost(message, tag = "12")]
        PlayAudio(PlayAudio),
        /// Output only. An audio response message composed of both the synthesized Dialogflow
        /// agent responses and responses defined via
        /// [play_audio][google.cloud.dialogflow.cx.v3beta1.ResponseMessage.play_audio].
        /// This message is generated by Dialogflow only and not supposed to be
        /// defined by the user.
        #[prost(message, tag = "13")]
        MixedAudio(MixedAudio),
    }
}
/// A fulfillment can do one or more of the following actions at the same time:
///
///   * Generate rich message responses.
///   * Set parameter values.
///   * Call the webhook.
///
/// Fulfillments can be called at various stages in the [Page][google.cloud.dialogflow.cx.v3beta1.Page] or
/// [Form][google.cloud.dialogflow.cx.v3beta1.Form] lifecycle. For example, when a [DetectIntentRequest][google.cloud.dialogflow.cx.v3beta1.DetectIntentRequest] drives a
/// session to enter a new page, the page's entry fulfillment can add a static
/// response to the [QueryResult][google.cloud.dialogflow.cx.v3beta1.QueryResult] in the returning [DetectIntentResponse][google.cloud.dialogflow.cx.v3beta1.DetectIntentResponse],
/// call the webhook (for example, to load user data from a database), or both.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Fulfillment {
    /// The list of rich message responses to present to the user.
    #[prost(message, repeated, tag = "1")]
    pub messages: ::std::vec::Vec<ResponseMessage>,
    /// The webhook to call.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/webhooks/<Webhook ID>`.
    #[prost(string, tag = "2")]
    pub webhook: std::string::String,
    /// The tag used by the webhook to identify which fulfillment is being called.
    /// This field is required if `webhook` is specified.
    #[prost(string, tag = "3")]
    pub tag: std::string::String,
    /// Set parameter values before executing the webhook.
    #[prost(message, repeated, tag = "4")]
    pub set_parameter_actions: ::std::vec::Vec<fulfillment::SetParameterAction>,
    /// Conditional cases for this fulfillment.
    #[prost(message, repeated, tag = "5")]
    pub conditional_cases: ::std::vec::Vec<fulfillment::ConditionalCases>,
}
pub mod fulfillment {
    /// Setting a parameter value.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SetParameterAction {
        /// Display name of the parameter.
        #[prost(string, tag = "1")]
        pub parameter: std::string::String,
        /// The new value of the parameter. A null value clears the parameter.
        #[prost(message, optional, tag = "2")]
        pub value: ::std::option::Option<::prost_types::Value>,
    }
    /// A list of cascading if-else conditions. Cases are mutually exclusive.
    /// The first one with a matching condition is selected, all the rest ignored.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ConditionalCases {
        /// A list of cascading if-else conditions.
        #[prost(message, repeated, tag = "1")]
        pub cases: ::std::vec::Vec<conditional_cases::Case>,
    }
    pub mod conditional_cases {
        /// Each case has a Boolean condition. When it is evaluated to be True, the
        /// corresponding messages will be selected and evaluated recursively.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Case {
            /// The condition to activate and select this case. Empty means the
            /// condition is always true. The condition is evaluated against [form
            /// parameters][Form.parameters] or [session
            /// parameters][SessionInfo.parameters].
            ///
            /// See the [conditions
            /// reference](https://cloud.google.com/dialogflow/cx/docs/reference/condition).
            #[prost(string, tag = "1")]
            pub condition: std::string::String,
            /// A list of case content.
            #[prost(message, repeated, tag = "2")]
            pub case_content: ::std::vec::Vec<case::CaseContent>,
        }
        pub mod case {
            /// The list of messages or conditional cases to activate for this case.
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct CaseContent {
                /// Either a message is returned or additional cases to be evaluated.
                #[prost(oneof = "case_content::CasesOrMessage", tags = "1, 2")]
                pub cases_or_message: ::std::option::Option<case_content::CasesOrMessage>,
            }
            pub mod case_content {
                /// Either a message is returned or additional cases to be evaluated.
                #[derive(Clone, PartialEq, ::prost::Oneof)]
                pub enum CasesOrMessage {
                    /// Returned message.
                    #[prost(message, tag = "1")]
                    Message(super::super::super::super::ResponseMessage),
                    /// Additional cases to be evaluated.
                    #[prost(message, tag = "2")]
                    AdditionalCases(super::super::super::ConditionalCases),
                }
            }
        }
    }
}
/// A Dialogflow CX conversation (session) can be described and visualized as a
/// state machine. The states of a CX session are represented by pages.
///
/// For each flow, you define many pages, where your combined pages can handle a
/// complete conversation on the topics the flow is designed for. At any given
/// moment, exactly one page is the current page, the current page is considered
/// active, and the flow associated with that page is considered active. Every
/// flow has a special start page. When a flow initially becomes active, the
/// start page page becomes the current page. For each conversational turn, the
/// current page will either stay the same or transition to another page.
///
/// You configure each page to collect information from the end-user that is
/// relevant for the conversational state represented by the page.
///
/// For more information, see the
/// [Page guide](https://cloud.google.com/dialogflow/cx/docs/concept/page).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Page {
    /// The unique identifier of the page.
    /// Required for the [Pages.UpdatePage][google.cloud.dialogflow.cx.v3beta1.Pages.UpdatePage] method. [Pages.CreatePage][google.cloud.dialogflow.cx.v3beta1.Pages.CreatePage]
    /// populates the name automatically.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/flows/<Flow ID>/pages/<Page ID>`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. The human-readable name of the page, unique within the agent.
    #[prost(string, tag = "2")]
    pub display_name: std::string::String,
    /// The fulfillment to call when the session is entering the page.
    #[prost(message, optional, tag = "7")]
    pub entry_fulfillment: ::std::option::Option<Fulfillment>,
    /// The form associated with the page, used for collecting parameters
    /// relevant to the page.
    #[prost(message, optional, tag = "4")]
    pub form: ::std::option::Option<Form>,
    /// Ordered list of [`TransitionRouteGroups`][google.cloud.dialogflow.cx.v3beta1.TransitionRouteGroup] associated
    /// with the page. Transition route groups must be unique within a page.
    ///
    /// *   If multiple transition routes within a page scope refer to the same
    ///     intent, then the precedence order is: page's transition route -> page's
    ///     transition route group -> flow's transition routes.
    ///
    /// *   If multiple transition route groups within a page contain the same
    ///     intent, then the first group in the ordered list takes precedence.
    ///
    /// Format:`projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/flows/<Flow ID>/transitionRouteGroups/<TransitionRouteGroup ID>`.
    #[prost(string, repeated, tag = "11")]
    pub transition_route_groups: ::std::vec::Vec<std::string::String>,
    /// A list of transitions for the transition rules of this page.
    /// They route the conversation to another page in the same flow, or another
    /// flow.
    ///
    /// When we are in a certain page, the TransitionRoutes are evalauted in the
    /// following order:
    ///
    /// *   TransitionRoutes defined in the page with intent specified.
    /// *   TransitionRoutes defined in the
    ///     [transition route groups][google.cloud.dialogflow.cx.v3beta1.Page.transition_route_groups] with intent
    ///     specified.
    /// *   TransitionRoutes defined in flow with intent specified.
    /// *   TransitionRoutes defined in the
    ///     [transition route groups][google.cloud.dialogflow.cx.v3beta1.Flow.transition_route_groups] with intent
    ///     specified.
    /// *   TransitionRoutes defined in the page with only condition specified.
    /// *   TransitionRoutes defined in the
    ///     [transition route groups][google.cloud.dialogflow.cx.v3beta1.Page.transition_route_groups] with only
    ///     condition specified.
    #[prost(message, repeated, tag = "9")]
    pub transition_routes: ::std::vec::Vec<TransitionRoute>,
    /// Handlers associated with the page to handle events such as webhook errors,
    /// no match or no input.
    #[prost(message, repeated, tag = "10")]
    pub event_handlers: ::std::vec::Vec<EventHandler>,
}
/// A form is a data model that groups related parameters that can be collected
/// from the user. The process in which the agent prompts the user and collects
/// parameter values from the user is called form filling. A form can be added to
/// a [page][google.cloud.dialogflow.cx.v3beta1.Page]. When form filling is done, the filled parameters will be
/// written to the [session][google.cloud.dialogflow.cx.v3beta1.SessionInfo.parameters].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Form {
    /// Parameters to collect from the user.
    #[prost(message, repeated, tag = "1")]
    pub parameters: ::std::vec::Vec<form::Parameter>,
}
pub mod form {
    /// Represents a form parameter.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Parameter {
        /// Required. The human-readable name of the parameter, unique within the
        /// form.
        #[prost(string, tag = "1")]
        pub display_name: std::string::String,
        /// Indicates whether the parameter is required. Optional parameters will not
        /// trigger prompts; however, they are filled if the user specifies them.
        /// Required parameters must be filled before form filling concludes.
        #[prost(bool, tag = "2")]
        pub required: bool,
        /// Required. The entity type of the parameter.
        /// Format: `projects/-/locations/-/agents/-/entityTypes/<System Entity Type
        /// ID>` for system entity types (for example,
        /// `projects/-/locations/-/agents/-/entityTypes/sys.date`), or
        /// `projects/<Project ID>/locations/<Location ID>/agents/<Agent
        /// ID>/entityTypes/<Entity Type ID>` for developer entity types.
        #[prost(string, tag = "3")]
        pub entity_type: std::string::String,
        /// Indicates whether the parameter represents a list of values.
        #[prost(bool, tag = "4")]
        pub is_list: bool,
        /// Required. Defines fill behavior for the parameter.
        #[prost(message, optional, tag = "7")]
        pub fill_behavior: ::std::option::Option<parameter::FillBehavior>,
        /// The default value of an optional parameter. If the parameter is required,
        /// the default value will be ignored.
        #[prost(message, optional, tag = "9")]
        pub default_value: ::std::option::Option<::prost_types::Value>,
        /// Indicates whether the parameter content should be redacted in log.  If
        /// redaction is enabled, the parameter content will be replaced by parameter
        /// name during logging.
        /// Note: the parameter content is subject to redaction if either parameter
        /// level redaction or [entity type level redaction][google.cloud.dialogflow.cx.v3beta1.EntityType.redact] is
        /// enabled.
        #[prost(bool, tag = "11")]
        pub redact: bool,
    }
    pub mod parameter {
        /// Configuration for how the filling of a parameter should be handled.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct FillBehavior {
            /// Required. The fulfillment to provide the initial prompt that the agent
            /// can present to the user in order to fill the parameter.
            #[prost(message, optional, tag = "3")]
            pub initial_prompt_fulfillment: ::std::option::Option<super::super::Fulfillment>,
            /// The handlers for parameter-level events, used to provide reprompt for
            /// the parameter or transition to a different page/flow. The supported
            /// events are:
            /// *   `sys.no-match-<N>`, where N can be from 1 to 6
            /// *   `sys.no-match-default`
            /// *   `sys.no-input-<N>`, where N can be from 1 to 6
            /// *   `sys.no-input-default`
            /// *   `sys.invalid-parameter`
            ///
            /// `initial_prompt_fulfillment` provides the first prompt for the
            /// parameter.
            ///
            /// If the user's response does not fill the parameter, a
            /// no-match/no-input event will be triggered, and the fulfillment
            /// associated with the `sys.no-match-1`/`sys.no-input-1` handler (if
            /// defined) will be called to provide a prompt. The
            /// `sys.no-match-2`/`sys.no-input-2` handler (if defined) will respond to
            /// the next no-match/no-input event, and so on.
            ///
            /// A `sys.no-match-default` or `sys.no-input-default` handler will be used
            /// to handle all following no-match/no-input events after all numbered
            /// no-match/no-input handlers for the parameter are consumed.
            ///
            /// A `sys.invalid-parameter` handler can be defined to handle the case
            /// where the parameter values have been `invalidated` by webhook. For
            /// example, if the user's response fill the parameter, however the
            /// parameter was invalidated by webhook, the fulfillment associated with
            /// the `sys.invalid-parameter` handler (if defined) will be called to
            /// provide a prompt.
            ///
            /// If the event handler for the corresponding event can't be found on the
            /// parameter, `initial_prompt_fulfillment` will be re-prompted.
            #[prost(message, repeated, tag = "5")]
            pub reprompt_event_handlers: ::std::vec::Vec<super::super::EventHandler>,
        }
    }
}
/// An event handler specifies an [event][google.cloud.dialogflow.cx.v3beta1.EventHandler.event] that can be handled
/// during a session. When the specified event happens, the following actions are
/// taken in order:
///
/// *   If there is a
/// [`trigger_fulfillment`][google.cloud.dialogflow.cx.v3beta1.EventHandler.trigger_fulfillment] associated with
/// the event, it will be called.
/// *   If there is a [`target_page`][google.cloud.dialogflow.cx.v3beta1.EventHandler.target_page] associated
/// with the event, the session will transition into the specified page.
/// *   If there is a [`target_flow`][google.cloud.dialogflow.cx.v3beta1.EventHandler.target_flow] associated
/// with the event, the session will transition into the specified flow.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventHandler {
    /// Output only. The unique identifier of this event handler.
    #[prost(string, tag = "6")]
    pub name: std::string::String,
    /// Required. The name of the event to handle.
    #[prost(string, tag = "4")]
    pub event: std::string::String,
    /// The fulfillment to call when the event occurs.
    /// Handling webhook errors with a fulfillment enabled with webhook could
    /// cause infinite loop. It is invalid to specify such fulfillment for a
    /// handler handling webhooks.
    #[prost(message, optional, tag = "5")]
    pub trigger_fulfillment: ::std::option::Option<Fulfillment>,
    /// The target to transition to, either a page in the same host flow (the flow
    /// that owns this [TransitionRoute][google.cloud.dialogflow.cx.v3beta1.TransitionRoute]), or another flow in the same agent.
    #[prost(oneof = "event_handler::Target", tags = "2, 3")]
    pub target: ::std::option::Option<event_handler::Target>,
}
pub mod event_handler {
    /// The target to transition to, either a page in the same host flow (the flow
    /// that owns this [TransitionRoute][google.cloud.dialogflow.cx.v3beta1.TransitionRoute]), or another flow in the same agent.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Target {
        /// The target page to transition to.
        /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
        /// ID>/flows/<Flow ID>/pages/<Page ID>`.
        #[prost(string, tag = "2")]
        TargetPage(std::string::String),
        /// The target flow to transition to.
        /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
        /// ID>/flows/<Flow ID>`.
        #[prost(string, tag = "3")]
        TargetFlow(std::string::String),
    }
}
/// A transition route specifies a [intent][google.cloud.dialogflow.cx.v3beta1.Intent] that can be matched and/or a
/// data condition that can be evaluated during a session. When a specified
/// transition is matched, the following actions are taken in order:
///
/// *   If there is a
/// [`trigger_fulfillment`][google.cloud.dialogflow.cx.v3beta1.TransitionRoute.trigger_fulfillment] associated with
/// the transition, it will be called.
/// *   If there is a [`target_page`][google.cloud.dialogflow.cx.v3beta1.TransitionRoute.target_page] associated
/// with the transition, the session will transition into the specified page.
/// *   If there is a [`target_flow`][google.cloud.dialogflow.cx.v3beta1.TransitionRoute.target_flow] associated
/// with the transition, the session will transition into the specified flow.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransitionRoute {
    /// Output only. The unique identifier of this transition route.
    #[prost(string, tag = "6")]
    pub name: std::string::String,
    /// The unique identifier of an [Intent][google.cloud.dialogflow.cx.v3beta1.Intent].
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/intents/<Intent ID>`.
    /// Indicates that the transition can only happen when the given intent is
    /// matched.
    /// At least one of `intent` or `condition` must be specified. When both
    /// `intent` and `condition` are specified, the transition can only happen
    /// when both are fulfilled.
    #[prost(string, tag = "1")]
    pub intent: std::string::String,
    /// The condition to evaluate against [form parameters][google.cloud.dialogflow.cx.v3beta1.Form.parameters] or
    /// [session parameters][google.cloud.dialogflow.cx.v3beta1.SessionInfo.parameters].
    ///
    /// See the [conditions
    /// reference](https://cloud.google.com/dialogflow/cx/docs/reference/condition).
    /// At least one of `intent` or `condition` must be specified. When both
    /// `intent` and `condition` are specified, the transition can only happen
    /// when both are fulfilled.
    #[prost(string, tag = "2")]
    pub condition: std::string::String,
    /// The fulfillment to call when the condition is satisfied. At least one of
    /// `trigger_fulfillment` and `target` must be specified. When both are
    /// defined, `trigger_fulfillment` is executed first.
    #[prost(message, optional, tag = "3")]
    pub trigger_fulfillment: ::std::option::Option<Fulfillment>,
    /// The target to transition to, either a page in the same host flow (the flow
    /// that owns this [TransitionRoute][google.cloud.dialogflow.cx.v3beta1.TransitionRoute]), or another flow in the same agent.
    #[prost(oneof = "transition_route::Target", tags = "4, 5")]
    pub target: ::std::option::Option<transition_route::Target>,
}
pub mod transition_route {
    /// The target to transition to, either a page in the same host flow (the flow
    /// that owns this [TransitionRoute][google.cloud.dialogflow.cx.v3beta1.TransitionRoute]), or another flow in the same agent.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Target {
        /// The target page to transition to.
        /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
        /// ID>/flows/<Flow ID>/pages/<Page ID>`.
        #[prost(string, tag = "4")]
        TargetPage(std::string::String),
        /// The target flow to transition to.
        /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
        /// ID>/flows/<Flow ID>`.
        #[prost(string, tag = "5")]
        TargetFlow(std::string::String),
    }
}
/// The request message for [Pages.ListPages][google.cloud.dialogflow.cx.v3beta1.Pages.ListPages].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPagesRequest {
    /// Required. The flow to list all pages for.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/flows/<Flow ID>`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// The language to list pages for. The following fields are language
    /// dependent:
    ///
    /// *  `Page.entry_fulfillment.messages`
    /// *  `Page.form.parameters.fill_behavior.initial_prompt_fulfillment.messages`
    /// *  `Page.form.parameters.fill_behavior.reprompt_event_handlers.messages`
    /// *  `Page.transition_routes.trigger_fulfillment.messages`
    /// *
    /// `Page.transition_route_groups.transition_routes.trigger_fulfillment.messages`
    ///
    /// If not specified, the agent's default language is used.
    /// [Many
    /// languages](https://cloud.google.com/dialogflow/cx/docs/reference/language)
    /// are supported.
    /// Note: languages must be enabled in the agent before they can be used.
    #[prost(string, tag = "2")]
    pub language_code: std::string::String,
    /// The maximum number of items to return in a single page. By default 100 and
    /// at most 1000.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous list request.
    #[prost(string, tag = "4")]
    pub page_token: std::string::String,
}
/// The response message for [Pages.ListPages][google.cloud.dialogflow.cx.v3beta1.Pages.ListPages].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPagesResponse {
    /// The list of pages. There will be a maximum number of items returned based
    /// on the page_size field in the request.
    #[prost(message, repeated, tag = "1")]
    pub pages: ::std::vec::Vec<Page>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// The request message for [Pages.GetPage][google.cloud.dialogflow.cx.v3beta1.Pages.GetPage].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPageRequest {
    /// Required. The name of the page.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/flows/<Flow ID>/pages/<Page ID>`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The language to retrieve the page for. The following fields are language
    /// dependent:
    ///
    /// *  `Page.entry_fulfillment.messages`
    /// *  `Page.form.parameters.fill_behavior.initial_prompt_fulfillment.messages`
    /// *  `Page.form.parameters.fill_behavior.reprompt_event_handlers.messages`
    /// *  `Page.transition_routes.trigger_fulfillment.messages`
    /// *
    /// `Page.transition_route_groups.transition_routes.trigger_fulfillment.messages`
    ///
    /// If not specified, the agent's default language is used.
    /// [Many
    /// languages](https://cloud.google.com/dialogflow/cx/docs/reference/language)
    /// are supported.
    /// Note: languages must be enabled in the agent before they can be used.
    #[prost(string, tag = "2")]
    pub language_code: std::string::String,
}
/// The request message for [Pages.CreatePage][google.cloud.dialogflow.cx.v3beta1.Pages.CreatePage].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePageRequest {
    /// Required. The flow to create a page for.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/flows/<Flow ID>`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The page to create.
    #[prost(message, optional, tag = "2")]
    pub page: ::std::option::Option<Page>,
    /// The language of the following fields in `page`:
    ///
    /// *  `Page.entry_fulfillment.messages`
    /// *  `Page.form.parameters.fill_behavior.initial_prompt_fulfillment.messages`
    /// *  `Page.form.parameters.fill_behavior.reprompt_event_handlers.messages`
    /// *  `Page.transition_routes.trigger_fulfillment.messages`
    /// *
    /// `Page.transition_route_groups.transition_routes.trigger_fulfillment.messages`
    ///
    /// If not specified, the agent's default language is used.
    /// [Many
    /// languages](https://cloud.google.com/dialogflow/cx/docs/reference/language)
    /// are supported.
    /// Note: languages must be enabled in the agent before they can be used.
    #[prost(string, tag = "3")]
    pub language_code: std::string::String,
}
/// The request message for [Pages.UpdatePage][google.cloud.dialogflow.cx.v3beta1.Pages.UpdatePage].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatePageRequest {
    /// Required. The page to update.
    #[prost(message, optional, tag = "1")]
    pub page: ::std::option::Option<Page>,
    /// The language of the following fields in `page`:
    ///
    /// *  `Page.entry_fulfillment.messages`
    /// *  `Page.form.parameters.fill_behavior.initial_prompt_fulfillment.messages`
    /// *  `Page.form.parameters.fill_behavior.reprompt_event_handlers.messages`
    /// *  `Page.transition_routes.trigger_fulfillment.messages`
    /// *
    /// `Page.transition_route_groups.transition_routes.trigger_fulfillment.messages`
    ///
    /// If not specified, the agent's default language is used.
    /// [Many
    /// languages](https://cloud.google.com/dialogflow/cx/docs/reference/language)
    /// are supported.
    /// Note: languages must be enabled in the agent before they can be used.
    #[prost(string, tag = "2")]
    pub language_code: std::string::String,
    /// The mask to control which fields get updated. If the mask is not present,
    /// all fields will be updated.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// The request message for [Pages.DeletePage][google.cloud.dialogflow.cx.v3beta1.Pages.DeletePage].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePageRequest {
    /// Required. The name of the page to delete.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/Flows/<flow ID>/pages/<Page ID>`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// This field has no effect for pages with no incoming transitions.
    /// For pages with incoming transitions:
    ///
    /// *  If `force` is set to false, an error will be returned with message
    ///    indicating the incoming transitions.
    /// *  If `force` is set to true, Dialogflow will remove the page, as well as
    ///    any transitions to the page (i.e. [Target
    ///    page][EventHandler.target_page] in event handlers or [Target
    ///    page][TransitionRoute.target_page] in transition routes that point to
    ///    this page will be cleared).
    #[prost(bool, tag = "2")]
    pub force: bool,
}
#[doc = r" Generated client implementations."]
pub mod pages_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service for managing [Pages][google.cloud.dialogflow.cx.v3beta1.Page]."]
    pub struct PagesClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PagesClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> PagesClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        #[doc = " Returns the list of all pages in the specified flow."]
        pub async fn list_pages(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPagesRequest>,
        ) -> Result<tonic::Response<super::ListPagesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3beta1.Pages/ListPages",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Retrieves the specified page."]
        pub async fn get_page(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPageRequest>,
        ) -> Result<tonic::Response<super::Page>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3beta1.Pages/GetPage",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a page in the specified flow."]
        pub async fn create_page(
            &mut self,
            request: impl tonic::IntoRequest<super::CreatePageRequest>,
        ) -> Result<tonic::Response<super::Page>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3beta1.Pages/CreatePage",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the specified page."]
        pub async fn update_page(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdatePageRequest>,
        ) -> Result<tonic::Response<super::Page>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3beta1.Pages/UpdatePage",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes the specified page."]
        pub async fn delete_page(
            &mut self,
            request: impl tonic::IntoRequest<super::DeletePageRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3beta1.Pages/DeletePage",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for PagesClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for PagesClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "PagesClient {{ ... }}")
        }
    }
}
/// Agent/flow validation message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidationMessage {
    /// The type of the resources where the message is found.
    #[prost(enumeration = "validation_message::ResourceType", tag = "1")]
    pub resource_type: i32,
    /// The names of the resources where the message is found.
    #[prost(string, repeated, tag = "2")]
    pub resources: ::std::vec::Vec<std::string::String>,
    /// The resource names of the resources where the message is found.
    #[prost(message, repeated, tag = "6")]
    pub resource_names: ::std::vec::Vec<ResourceName>,
    /// Indicates the severity of the message.
    #[prost(enumeration = "validation_message::Severity", tag = "3")]
    pub severity: i32,
    /// The message detail.
    #[prost(string, tag = "4")]
    pub detail: std::string::String,
}
pub mod validation_message {
    /// Resource types.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ResourceType {
        /// Unspecified.
        Unspecified = 0,
        /// Agent.
        Agent = 1,
        /// Intent.
        Intent = 2,
        /// Intent training phrase.
        IntentTrainingPhrase = 8,
        /// Intent parameter.
        IntentParameter = 9,
        /// Multiple intents.
        Intents = 10,
        /// Multiple training phrases.
        IntentTrainingPhrases = 11,
        /// Entity type.
        EntityType = 3,
        /// Multiple entity types.
        EntityTypes = 12,
        /// Webhook.
        Webhook = 4,
        /// Flow.
        Flow = 5,
        /// Page.
        Page = 6,
        /// Multiple pages.
        Pages = 13,
        /// Transition route group.
        TransitionRouteGroup = 7,
    }
    /// Severity level.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Severity {
        /// Unspecified.
        Unspecified = 0,
        /// The agent doesn't follow Dialogflow best practices.
        Info = 1,
        /// The agent may not behave as expected.
        Warning = 2,
        /// The agent may experience failures.
        Error = 3,
    }
}
/// Resource name and display name.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceName {
    /// Name.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Display name.
    #[prost(string, tag = "2")]
    pub display_name: std::string::String,
}
/// Settings related to NLU.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NluSettings {
    /// Indicates the type of NLU model.
    #[prost(enumeration = "nlu_settings::ModelType", tag = "1")]
    pub model_type: i32,
    /// To filter out false positive results and still get variety in matched
    /// natural language inputs for your agent, you can tune the machine learning
    /// classification threshold. If the returned score value is less than the
    /// threshold value, then a no-match event will be triggered. The score values
    /// range from 0.0 (completely uncertain) to 1.0 (completely certain). If set
    /// to 0.0, the default of 0.3 is used.
    #[prost(float, tag = "3")]
    pub classification_threshold: f32,
    /// Indicates NLU model training mode.
    #[prost(enumeration = "nlu_settings::ModelTrainingMode", tag = "4")]
    pub model_training_mode: i32,
}
pub mod nlu_settings {
    /// NLU model type.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ModelType {
        /// Not specified. `MODEL_TYPE_STANDARD` will be used.
        Unspecified = 0,
        /// Use standard NLU model.
        Standard = 1,
        /// Use advanced NLU model.
        Advanced = 3,
    }
    /// NLU model training mode.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ModelTrainingMode {
        /// Not specified. `MODEL_TRAINING_MODE_AUTOMATIC` will be used.
        Unspecified = 0,
        /// NLU model training is automatically triggered when a flow gets modified.
        /// User can also manually trigger model training in this mode.
        Automatic = 1,
        /// User needs to manually trigger NLU model training. Best for large flows
        /// whose models take long time to train.
        Manual = 2,
    }
}
/// Flows represents the conversation flows when you build your chatbot agent.
///
/// A flow consists of many pages connected by the transition routes.
/// Conversations always start with the built-in Start Flow (with an all-0 ID).
/// Transition routes can direct the conversation session from the current flow
/// (parent flow) to another flow (sub flow). When the sub flow is finished,
/// Dialogflow will bring the session back to the parent flow, where the sub flow
/// is started.
///
/// Usually, when a transition route is followed by a matched intent, the intent
/// will be "consumed". This means the intent won't activate more transition
/// routes. However, when the followed transition route moves the conversation
/// session into a different flow, the matched intent can be carried over and to
/// be consumed in the target flow.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Flow {
    /// The unique identifier of the flow.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/flows/<Flow ID>`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. The human-readable name of the flow.
    #[prost(string, tag = "2")]
    pub display_name: std::string::String,
    /// The description of the flow. The maximum length is 500 characters. If
    /// exceeded, the request is rejected.
    #[prost(string, tag = "3")]
    pub description: std::string::String,
    /// A flow's transition routes serve two purposes:
    ///
    /// *   They are responsible for matching the user's first utterances in the
    /// flow.
    /// *   They are inherited by every page's [transition
    /// routes][Page.transition_routes] and can support use cases such as the user
    /// saying "help" or "can I talk to a human?", which can be handled in a common
    /// way regardless of the current page. Transition routes defined in the page
    /// have higher priority than those defined in the flow.
    ///
    /// TransitionRoutes are evalauted in the following order:
    ///
    /// *   TransitionRoutes with intent specified..
    /// *   TransitionRoutes with only condition specified.
    ///
    /// TransitionRoutes with intent specified are inherited by pages in the flow.
    #[prost(message, repeated, tag = "4")]
    pub transition_routes: ::std::vec::Vec<TransitionRoute>,
    /// A flow's event handlers serve two purposes:
    ///
    /// *   They are responsible for handling events (e.g. no match,
    /// webhook errors) in the flow.
    /// *   They are inherited by every page's [event
    /// handlers][Page.event_handlers], which can be used to handle common events
    /// regardless of the current page. Event handlers defined in the page
    /// have higher priority than those defined in the flow.
    ///
    /// Unlike [transition_routes][google.cloud.dialogflow.cx.v3beta1.Flow.transition_routes], these handlers are
    /// evaluated on a first-match basis. The first one that matches the event
    /// get executed, with the rest being ignored.
    #[prost(message, repeated, tag = "10")]
    pub event_handlers: ::std::vec::Vec<EventHandler>,
    /// A flow's transition route group serve two purposes:
    ///
    /// *   They are responsible for matching the user's first utterances in the
    /// flow.
    /// *   They are inherited by every page's [transition
    /// route groups][Page.transition_route_groups]. Transition route groups
    /// defined in the page have higher priority than those defined in the flow.
    ///
    /// Format:`projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/flows/<Flow ID>/transitionRouteGroups/<TransitionRouteGroup ID>`.
    #[prost(string, repeated, tag = "15")]
    pub transition_route_groups: ::std::vec::Vec<std::string::String>,
    /// NLU related settings of the flow.
    #[prost(message, optional, tag = "11")]
    pub nlu_settings: ::std::option::Option<NluSettings>,
}
/// The request message for [Flows.CreateFlow][google.cloud.dialogflow.cx.v3beta1.Flows.CreateFlow].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateFlowRequest {
    /// Required. The agent to create a flow for.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The flow to create.
    #[prost(message, optional, tag = "2")]
    pub flow: ::std::option::Option<Flow>,
    /// The language of the following fields in `flow`:
    ///
    /// *  `Flow.event_handlers.trigger_fulfillment.messages`
    /// *  `Flow.transition_routes.trigger_fulfillment.messages`
    ///
    /// If not specified, the agent's default language is used.
    /// [Many
    /// languages](https://cloud.google.com/dialogflow/cx/docs/reference/language)
    /// are supported.
    /// Note: languages must be enabled in the agent before they can be used.
    #[prost(string, tag = "3")]
    pub language_code: std::string::String,
}
/// The request message for [Flows.DeleteFlow][google.cloud.dialogflow.cx.v3beta1.Flows.DeleteFlow].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteFlowRequest {
    /// Required. The name of the flow to delete.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/flows/<Flow ID>`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// This field has no effect for flows with no incoming transitions.
    /// For flows with incoming transitions:
    ///
    /// *  If `force` is set to false, an error will be returned with message
    ///    indicating the incoming transitions.
    /// *  If `force` is set to true, Dialogflow will remove the flow, as well as
    ///    any transitions to the flow (i.e. [Target
    ///    flow][EventHandler.target_flow] in event handlers or [Target
    ///    flow][TransitionRoute.target_flow] in transition routes that point to
    ///    this flow will be cleared).
    #[prost(bool, tag = "2")]
    pub force: bool,
}
/// The request message for [Flows.ListFlows][google.cloud.dialogflow.cx.v3beta1.Flows.ListFlows].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFlowsRequest {
    /// Required. The agent containing the flows.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// The maximum number of items to return in a single page. By default 100 and
    /// at most 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous list request.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
    /// The language to list flows for. The following fields are language
    /// dependent:
    ///
    /// *  `Flow.event_handlers.trigger_fulfillment.messages`
    /// *  `Flow.transition_routes.trigger_fulfillment.messages`
    ///
    /// If not specified, the agent's default language is used.
    /// [Many
    /// languages](https://cloud.google.com/dialogflow/cx/docs/reference/language)
    /// are supported.
    /// Note: languages must be enabled in the agent before they can be used.
    #[prost(string, tag = "4")]
    pub language_code: std::string::String,
}
/// The response message for [Flows.ListFlows][google.cloud.dialogflow.cx.v3beta1.Flows.ListFlows].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFlowsResponse {
    /// The list of flows. There will be a maximum number of items returned based
    /// on the page_size field in the request.
    #[prost(message, repeated, tag = "1")]
    pub flows: ::std::vec::Vec<Flow>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// The response message for [Flows.GetFlow][google.cloud.dialogflow.cx.v3beta1.Flows.GetFlow].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFlowRequest {
    /// Required. The name of the flow to get.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/flows/<Flow ID>`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The language to retrieve the flow for. The following fields are language
    /// dependent:
    ///
    /// *  `Flow.event_handlers.trigger_fulfillment.messages`
    /// *  `Flow.transition_routes.trigger_fulfillment.messages`
    ///
    /// If not specified, the agent's default language is used.
    /// [Many
    /// languages](https://cloud.google.com/dialogflow/cx/docs/reference/language)
    /// are supported.
    /// Note: languages must be enabled in the agent before they can be used.
    #[prost(string, tag = "2")]
    pub language_code: std::string::String,
}
/// The request message for [Flows.UpdateFlow][google.cloud.dialogflow.cx.v3beta1.Flows.UpdateFlow].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateFlowRequest {
    /// Required. The flow to update.
    #[prost(message, optional, tag = "1")]
    pub flow: ::std::option::Option<Flow>,
    /// Required. The mask to control which fields get updated. If `update_mask` is not
    /// specified, an error will be returned.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// The language of the following fields in `flow`:
    ///
    /// *  `Flow.event_handlers.trigger_fulfillment.messages`
    /// *  `Flow.transition_routes.trigger_fulfillment.messages`
    ///
    /// If not specified, the agent's default language is used.
    /// [Many
    /// languages](https://cloud.google.com/dialogflow/cx/docs/reference/language)
    /// are supported.
    /// Note: languages must be enabled in the agent before they can be used.
    #[prost(string, tag = "3")]
    pub language_code: std::string::String,
}
/// The request message for [Flows.TrainFlow][google.cloud.dialogflow.cx.v3beta1.Flows.TrainFlow].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrainFlowRequest {
    /// Required. The flow to train.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/flows/<Flow ID>`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// The request message for [Flows.ValidateFlow][google.cloud.dialogflow.cx.v3beta1.Flows.ValidateFlow].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidateFlowRequest {
    /// Required. The flow to validate.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/flows/<Flow ID>`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// If not specified, the agent's default language is used.
    #[prost(string, tag = "2")]
    pub language_code: std::string::String,
}
/// The request message for [Flows.GetFlowValidationResult][google.cloud.dialogflow.cx.v3beta1.Flows.GetFlowValidationResult].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFlowValidationResultRequest {
    /// Required. The flow name.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/flows/<Flow ID>/validationResult`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// If not specified, the agent's default language is used.
    #[prost(string, tag = "2")]
    pub language_code: std::string::String,
}
/// The response message for [Flows.GetFlowValidationResult][google.cloud.dialogflow.cx.v3beta1.Flows.GetFlowValidationResult].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlowValidationResult {
    /// The unique identifier of the flow validation result.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/flows/<Flow ID>/validationResult`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Contains all validation messages.
    #[prost(message, repeated, tag = "2")]
    pub validation_messages: ::std::vec::Vec<ValidationMessage>,
    /// Last time the flow was validated.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::std::option::Option<::prost_types::Timestamp>,
}
#[doc = r" Generated client implementations."]
pub mod flows_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service for managing [Flows][google.cloud.dialogflow.cx.v3beta1.Flow]."]
    pub struct FlowsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl FlowsClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> FlowsClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        #[doc = " Creates a flow in the specified agent."]
        pub async fn create_flow(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateFlowRequest>,
        ) -> Result<tonic::Response<super::Flow>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3beta1.Flows/CreateFlow",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a specified flow."]
        pub async fn delete_flow(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteFlowRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3beta1.Flows/DeleteFlow",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns the list of all flows in the specified agent."]
        pub async fn list_flows(
            &mut self,
            request: impl tonic::IntoRequest<super::ListFlowsRequest>,
        ) -> Result<tonic::Response<super::ListFlowsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3beta1.Flows/ListFlows",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Retrieves the specified flow."]
        pub async fn get_flow(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFlowRequest>,
        ) -> Result<tonic::Response<super::Flow>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3beta1.Flows/GetFlow",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the specified flow."]
        pub async fn update_flow(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateFlowRequest>,
        ) -> Result<tonic::Response<super::Flow>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3beta1.Flows/UpdateFlow",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Trains the specified flow. Note that only the flow in 'draft' environment"]
        #[doc = " is trained."]
        pub async fn train_flow(
            &mut self,
            request: impl tonic::IntoRequest<super::TrainFlowRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3beta1.Flows/TrainFlow",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Validates the specified flow and creates or updates validation results."]
        #[doc = " Please call this API after the training is completed to get the complete"]
        #[doc = " validation results."]
        pub async fn validate_flow(
            &mut self,
            request: impl tonic::IntoRequest<super::ValidateFlowRequest>,
        ) -> Result<tonic::Response<super::FlowValidationResult>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3beta1.Flows/ValidateFlow",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the latest flow validation result. Flow validation is performed"]
        #[doc = " when ValidateFlow is called."]
        pub async fn get_flow_validation_result(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFlowValidationResultRequest>,
        ) -> Result<tonic::Response<super::FlowValidationResult>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3beta1.Flows/GetFlowValidationResult",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for FlowsClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for FlowsClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "FlowsClient {{ ... }}")
        }
    }
}
/// An intent represents a user's intent to interact with a conversational agent.
///
/// You can provide information for the Dialogflow API to use to match user input
/// to an intent by adding training phrases (i.e., examples of user input) to
/// your intent.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Intent {
    /// The unique identifier of the intent.
    /// Required for the [Intents.UpdateIntent][google.cloud.dialogflow.cx.v3beta1.Intents.UpdateIntent] method. [Intents.CreateIntent][google.cloud.dialogflow.cx.v3beta1.Intents.CreateIntent]
    /// populates the name automatically.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/intents/<Intent ID>`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. The human-readable name of the intent, unique within the agent.
    #[prost(string, tag = "2")]
    pub display_name: std::string::String,
    /// The collection of training phrases the agent is trained on to identify the
    /// intent.
    #[prost(message, repeated, tag = "3")]
    pub training_phrases: ::std::vec::Vec<intent::TrainingPhrase>,
    /// The collection of parameters associated with the intent.
    #[prost(message, repeated, tag = "4")]
    pub parameters: ::std::vec::Vec<intent::Parameter>,
    /// The priority of this intent. Higher numbers represent higher
    /// priorities.
    ///
    /// - If the supplied value is unspecified or 0, the service
    ///   translates the value to 500,000, which corresponds to the
    ///   `Normal` priority in the console.
    /// - If the supplied value is negative, the intent is ignored
    ///   in runtime detect intent requests.
    #[prost(int32, tag = "5")]
    pub priority: i32,
    /// Indicates whether this is a fallback intent. Currently only default
    /// fallback intent is allowed in the agent, which is added upon agent
    /// creation.
    /// Adding training phrases to fallback intent is useful in the case of
    /// requests that are mistakenly matched, since training phrases assigned to
    /// fallback intents act as negative examples that triggers no-match event.
    #[prost(bool, tag = "6")]
    pub is_fallback: bool,
    /// The key/value metadata to label an intent. Labels can contain
    /// lowercase letters, digits and the symbols '-' and '_'. International
    /// characters are allowed, including letters from unicase alphabets. Keys must
    /// start with a letter. Keys and values can be no longer than 63 characters
    /// and no more than 128 bytes.
    ///
    /// Prefix "sys-" is reserved for Dialogflow defined labels. Currently allowed
    /// Dialogflow defined labels include:
    /// * sys-head
    /// * sys-contextual
    /// The above labels do not require value. "sys-head" means the intent is a
    /// head intent. "sys-contextual" means the intent is a contextual intent.
    #[prost(map = "string, string", tag = "7")]
    pub labels: ::std::collections::HashMap<std::string::String, std::string::String>,
    /// Human readable description for better understanding an intent like its
    /// scope, content, result etc. Maximum character limit: 140 characters.
    #[prost(string, tag = "8")]
    pub description: std::string::String,
}
pub mod intent {
    /// Represents an example that the agent is trained on to identify the intent.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TrainingPhrase {
        /// Output only. The unique identifier of the training phrase.
        #[prost(string, tag = "1")]
        pub id: std::string::String,
        /// Required. The ordered list of training phrase parts.
        /// The parts are concatenated in order to form the training phrase.
        ///
        /// Note: The API does not automatically annotate training phrases like the
        /// Dialogflow Console does.
        ///
        /// Note: Do not forget to include whitespace at part boundaries, so the
        /// training phrase is well formatted when the parts are concatenated.
        ///
        /// If the training phrase does not need to be annotated with parameters,
        /// you just need a single part with only the [Part.text][google.cloud.dialogflow.cx.v3beta1.Intent.TrainingPhrase.Part.text] field set.
        ///
        /// If you want to annotate the training phrase, you must create multiple
        /// parts, where the fields of each part are populated in one of two ways:
        ///
        /// -   `Part.text` is set to a part of the phrase that has no parameters.
        /// -   `Part.text` is set to a part of the phrase that you want to annotate,
        ///     and the `parameter_id` field is set.
        #[prost(message, repeated, tag = "2")]
        pub parts: ::std::vec::Vec<training_phrase::Part>,
        /// Indicates how many times this example was added to the intent.
        #[prost(int32, tag = "3")]
        pub repeat_count: i32,
    }
    pub mod training_phrase {
        /// Represents a part of a training phrase.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Part {
            /// Required. The text for this part.
            #[prost(string, tag = "1")]
            pub text: std::string::String,
            /// The [parameter][google.cloud.dialogflow.cx.v3beta1.Intent.Parameter] used to annotate this part of the
            /// training phrase. This field is required for annotated parts of the
            /// training phrase.
            #[prost(string, tag = "2")]
            pub parameter_id: std::string::String,
        }
    }
    /// Represents an intent parameter.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Parameter {
        /// Required. The unique identifier of the parameter. This field
        /// is used by [training phrases][google.cloud.dialogflow.cx.v3beta1.Intent.TrainingPhrase] to annotate their
        /// [parts][google.cloud.dialogflow.cx.v3beta1.Intent.TrainingPhrase.Part].
        #[prost(string, tag = "1")]
        pub id: std::string::String,
        /// Required. The entity type of the parameter.
        /// Format: `projects/-/locations/-/agents/-/entityTypes/<System Entity Type
        /// ID>` for system entity types (for example,
        /// `projects/-/locations/-/agents/-/entityTypes/sys.date`), or
        /// `projects/<Project ID>/locations/<Location ID>/agents/<Agent
        /// ID>/entityTypes/<Entity Type ID>` for developer entity types.
        #[prost(string, tag = "2")]
        pub entity_type: std::string::String,
        /// Indicates whether the parameter represents a list of values.
        #[prost(bool, tag = "3")]
        pub is_list: bool,
        /// Indicates whether the parameter content should be redacted in log. If
        /// redaction is enabled, the parameter content will be replaced by parameter
        /// name during logging.
        /// Note: the parameter content is subject to redaction if either parameter
        /// level redaction or [entity type level redaction][google.cloud.dialogflow.cx.v3beta1.EntityType.redact] is
        /// enabled.
        #[prost(bool, tag = "4")]
        pub redact: bool,
    }
}
/// The request message for [Intents.ListIntents][google.cloud.dialogflow.cx.v3beta1.Intents.ListIntents].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListIntentsRequest {
    /// Required. The agent to list all intents for.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// The language to list intents for. The following fields are language
    /// dependent:
    ///
    /// *   `Intent.training_phrases.parts.text`
    ///
    /// If not specified, the agent's default language is used.
    /// [Many
    /// languages](https://cloud.google.com/dialogflow/cx/docs/reference/language)
    /// are supported.
    /// Note: languages must be enabled in the agent before they can be used.
    #[prost(string, tag = "2")]
    pub language_code: std::string::String,
    /// The resource view to apply to the returned intent.
    #[prost(enumeration = "IntentView", tag = "5")]
    pub intent_view: i32,
    /// The maximum number of items to return in a single page. By default 100 and
    /// at most 1000.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous list request.
    #[prost(string, tag = "4")]
    pub page_token: std::string::String,
}
/// The response message for [Intents.ListIntents][google.cloud.dialogflow.cx.v3beta1.Intents.ListIntents].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListIntentsResponse {
    /// The list of intents. There will be a maximum number of items returned based
    /// on the page_size field in the request.
    #[prost(message, repeated, tag = "1")]
    pub intents: ::std::vec::Vec<Intent>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// The request message for [Intents.GetIntent][google.cloud.dialogflow.cx.v3beta1.Intents.GetIntent].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetIntentRequest {
    /// Required. The name of the intent.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/intents/<Intent ID>`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The language to retrieve the intent for. The following fields are language
    /// dependent:
    ///
    /// *   `Intent.training_phrases.parts.text`
    ///
    /// If not specified, the agent's default language is used.
    /// [Many
    /// languages](https://cloud.google.com/dialogflow/cx/docs/reference/language)
    /// are supported.
    /// Note: languages must be enabled in the agent before they can be used.
    #[prost(string, tag = "2")]
    pub language_code: std::string::String,
}
/// The request message for [Intents.CreateIntent][google.cloud.dialogflow.cx.v3beta1.Intents.CreateIntent].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateIntentRequest {
    /// Required. The agent to create an intent for.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The intent to create.
    #[prost(message, optional, tag = "2")]
    pub intent: ::std::option::Option<Intent>,
    /// The language of the following fields in `intent`:
    ///
    /// *   `Intent.training_phrases.parts.text`
    ///
    /// If not specified, the agent's default language is used.
    /// [Many
    /// languages](https://cloud.google.com/dialogflow/cx/docs/reference/language)
    /// are supported.
    /// Note: languages must be enabled in the agent before they can be used.
    #[prost(string, tag = "3")]
    pub language_code: std::string::String,
}
/// The request message for [Intents.UpdateIntent][google.cloud.dialogflow.cx.v3beta1.Intents.UpdateIntent].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateIntentRequest {
    /// Required. The intent to update.
    #[prost(message, optional, tag = "1")]
    pub intent: ::std::option::Option<Intent>,
    /// The language of the following fields in `intent`:
    ///
    /// *   `Intent.training_phrases.parts.text`
    ///
    /// If not specified, the agent's default language is used.
    /// [Many
    /// languages](https://cloud.google.com/dialogflow/cx/docs/reference/language)
    /// are supported.
    /// Note: languages must be enabled in the agent before they can be used.
    #[prost(string, tag = "2")]
    pub language_code: std::string::String,
    /// The mask to control which fields get updated. If the mask is not present,
    /// all fields will be updated.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// The request message for [Intents.DeleteIntent][google.cloud.dialogflow.cx.v3beta1.Intents.DeleteIntent].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteIntentRequest {
    /// Required. The name of the intent to delete.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/intents/<Intent ID>`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Represents the options for views of an intent.
/// An intent can be a sizable object. Therefore, we provide a resource view that
/// does not return training phrases in the response.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum IntentView {
    /// Not specified. Treated as INTENT_VIEW_FULL.
    Unspecified = 0,
    /// Training phrases field is not populated in the response.
    Partial = 1,
    /// All fields are populated.
    Full = 2,
}
#[doc = r" Generated client implementations."]
pub mod intents_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service for managing [Intents][google.cloud.dialogflow.cx.v3beta1.Intent]."]
    pub struct IntentsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl IntentsClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> IntentsClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        #[doc = " Returns the list of all intents in the specified agent."]
        pub async fn list_intents(
            &mut self,
            request: impl tonic::IntoRequest<super::ListIntentsRequest>,
        ) -> Result<tonic::Response<super::ListIntentsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3beta1.Intents/ListIntents",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Retrieves the specified intent."]
        pub async fn get_intent(
            &mut self,
            request: impl tonic::IntoRequest<super::GetIntentRequest>,
        ) -> Result<tonic::Response<super::Intent>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3beta1.Intents/GetIntent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates an intent in the specified agent."]
        pub async fn create_intent(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateIntentRequest>,
        ) -> Result<tonic::Response<super::Intent>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3beta1.Intents/CreateIntent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the specified intent."]
        pub async fn update_intent(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateIntentRequest>,
        ) -> Result<tonic::Response<super::Intent>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3beta1.Intents/UpdateIntent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes the specified intent."]
        pub async fn delete_intent(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteIntentRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3beta1.Intents/DeleteIntent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for IntentsClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for IntentsClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "IntentsClient {{ ... }}")
        }
    }
}
/// Entities are extracted from user input and represent parameters that are
/// meaningful to your application. For example, a date range, a proper name
/// such as a geographic location or landmark, and so on. Entities represent
/// actionable data for your application.
///
/// When you define an entity, you can also include synonyms that all map to
/// that entity. For example, "soft drink", "soda", "pop", and so on.
///
/// There are three types of entities:
///
/// *   **System** - entities that are defined by the Dialogflow API for common
///     data types such as date, time, currency, and so on. A system entity is
///     represented by the `EntityType` type.
///
/// *   **Custom** - entities that are defined by you that represent
///     actionable data that is meaningful to your application. For example,
///     you could define a `pizza.sauce` entity for red or white pizza sauce,
///     a `pizza.cheese` entity for the different types of cheese on a pizza,
///     a `pizza.topping` entity for different toppings, and so on. A custom
///     entity is represented by the `EntityType` type.
///
/// *   **User** - entities that are built for an individual user such as
///     favorites, preferences, playlists, and so on. A user entity is
///     represented by the [SessionEntityType][google.cloud.dialogflow.cx.v3beta1.SessionEntityType] type.
///
/// For more information about entity types, see the [Dialogflow
/// documentation](https://cloud.google.com/dialogflow/docs/entities-overview).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntityType {
    /// The unique identifier of the entity type.
    /// Required for [EntityTypes.UpdateEntityType][google.cloud.dialogflow.cx.v3beta1.EntityTypes.UpdateEntityType].
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/entityTypes/<Entity Type ID>`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. The human-readable name of the entity type, unique within the agent.
    #[prost(string, tag = "2")]
    pub display_name: std::string::String,
    /// Required. Indicates the kind of entity type.
    #[prost(enumeration = "entity_type::Kind", tag = "3")]
    pub kind: i32,
    /// Indicates whether the entity type can be automatically expanded.
    #[prost(enumeration = "entity_type::AutoExpansionMode", tag = "4")]
    pub auto_expansion_mode: i32,
    /// The collection of entity entries associated with the entity type.
    #[prost(message, repeated, tag = "5")]
    pub entities: ::std::vec::Vec<entity_type::Entity>,
    /// Collection of exceptional words and phrases that shouldn't be matched.
    /// For example, if you have a size entity type with entry `giant`(an
    /// adjective), you might consider adding `giants`(a noun) as an exclusion.
    /// If the kind of entity type is `KIND_MAP`, then the phrases specified by
    /// entities and excluded phrases should be mutually exclusive.
    #[prost(message, repeated, tag = "6")]
    pub excluded_phrases: ::std::vec::Vec<entity_type::ExcludedPhrase>,
    /// Enables fuzzy entity extraction during classification.
    #[prost(bool, tag = "7")]
    pub enable_fuzzy_extraction: bool,
    /// Indicates whether parameters of the entity type should be redacted in log.
    /// If redaction is enabled, page parameters and intent parameters referring to
    /// the entity type will be replaced by parameter name during logging.
    #[prost(bool, tag = "9")]
    pub redact: bool,
}
pub mod entity_type {
    /// An **entity entry** for an associated entity type.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Entity {
        /// Required. The primary value associated with this entity entry.
        /// For example, if the entity type is *vegetable*, the value could be
        /// *scallions*.
        ///
        /// For `KIND_MAP` entity types:
        ///
        /// *   A canonical value to be used in place of synonyms.
        ///
        /// For `KIND_LIST` entity types:
        ///
        /// *   A string that can contain references to other entity types (with or
        ///     without aliases).
        #[prost(string, tag = "1")]
        pub value: std::string::String,
        /// Required. A collection of value synonyms. For example, if the entity type
        /// is *vegetable*, and `value` is *scallions*, a synonym could be *green
        /// onions*.
        ///
        /// For `KIND_LIST` entity types:
        ///
        /// *   This collection must contain exactly one synonym equal to `value`.
        #[prost(string, repeated, tag = "2")]
        pub synonyms: ::std::vec::Vec<std::string::String>,
    }
    /// An excluded entity phrase that should not be matched.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ExcludedPhrase {
        /// Required. The word or phrase to be excluded.
        #[prost(string, tag = "1")]
        pub value: std::string::String,
    }
    /// Represents kinds of entities.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Kind {
        /// Not specified. This value should be never used.
        Unspecified = 0,
        /// Map entity types allow mapping of a group of synonyms to a canonical
        /// value.
        Map = 1,
        /// List entity types contain a set of entries that do not map to canonical
        /// values. However, list entity types can contain references to other entity
        /// types (with or without aliases).
        List = 2,
        /// Regexp entity types allow to specify regular expressions in entries
        /// values.
        Regexp = 3,
    }
    /// Represents different entity type expansion modes. Automated expansion
    /// allows an agent to recognize values that have not been explicitly listed in
    /// the entity (for example, new kinds of shopping list items).
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AutoExpansionMode {
        /// Auto expansion disabled for the entity.
        Unspecified = 0,
        /// Allows an agent to recognize values that have not been explicitly
        /// listed in the entity.
        Default = 1,
    }
}
/// The request message for [EntityTypes.ListEntityTypes][google.cloud.dialogflow.cx.v3beta1.EntityTypes.ListEntityTypes].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEntityTypesRequest {
    /// Required. The agent to list all entity types for.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// The language to list entity types for. The following fields are language
    /// dependent:
    ///
    /// *   `EntityType.entities.value`
    /// *   `EntityType.entities.synonyms`
    /// *   `EntityType.excluded_phrases.value`
    ///
    /// If not specified, the agent's default language is used.
    /// [Many
    /// languages](https://cloud.google.com/dialogflow/cx/docs/reference/language)
    /// are supported.
    /// Note: languages must be enabled in the agent before they can be used.
    #[prost(string, tag = "2")]
    pub language_code: std::string::String,
    /// The maximum number of items to return in a single page. By default 100 and
    /// at most 1000.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous list request.
    #[prost(string, tag = "4")]
    pub page_token: std::string::String,
}
/// The response message for [EntityTypes.ListEntityTypes][google.cloud.dialogflow.cx.v3beta1.EntityTypes.ListEntityTypes].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEntityTypesResponse {
    /// The list of entity types. There will be a maximum number of items returned
    /// based on the page_size field in the request.
    #[prost(message, repeated, tag = "1")]
    pub entity_types: ::std::vec::Vec<EntityType>,
    /// Token to retrieve the next page of results, or empty if there are no
    /// more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// The request message for [EntityTypes.GetEntityType][google.cloud.dialogflow.cx.v3beta1.EntityTypes.GetEntityType].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEntityTypeRequest {
    /// Required. The name of the entity type.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/entityTypes/<Entity Type ID>`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The language to retrieve the entity type for. The following fields are
    /// language dependent:
    ///
    /// *   `EntityType.entities.value`
    /// *   `EntityType.entities.synonyms`
    /// *   `EntityType.excluded_phrases.value`
    ///
    /// If not specified, the agent's default language is used.
    /// [Many
    /// languages](https://cloud.google.com/dialogflow/cx/docs/reference/language)
    /// are supported.
    /// Note: languages must be enabled in the agent before they can be used.
    #[prost(string, tag = "2")]
    pub language_code: std::string::String,
}
/// The request message for [EntityTypes.CreateEntityType][google.cloud.dialogflow.cx.v3beta1.EntityTypes.CreateEntityType].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateEntityTypeRequest {
    /// Required. The agent to create a entity type for.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The entity type to create.
    #[prost(message, optional, tag = "2")]
    pub entity_type: ::std::option::Option<EntityType>,
    /// The language of the following fields in `entity_type`:
    ///
    /// *   `EntityType.entities.value`
    /// *   `EntityType.entities.synonyms`
    /// *   `EntityType.excluded_phrases.value`
    ///
    /// If not specified, the agent's default language is used.
    /// [Many
    /// languages](https://cloud.google.com/dialogflow/cx/docs/reference/language)
    /// are supported.
    /// Note: languages must be enabled in the agent before they can be used.
    #[prost(string, tag = "3")]
    pub language_code: std::string::String,
}
/// The request message for [EntityTypes.UpdateEntityType][google.cloud.dialogflow.cx.v3beta1.EntityTypes.UpdateEntityType].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateEntityTypeRequest {
    /// Required. The entity type to update.
    #[prost(message, optional, tag = "1")]
    pub entity_type: ::std::option::Option<EntityType>,
    /// The language of the following fields in `entity_type`:
    ///
    /// *   `EntityType.entities.value`
    /// *   `EntityType.entities.synonyms`
    /// *   `EntityType.excluded_phrases.value`
    ///
    /// If not specified, the agent's default language is used.
    /// [Many
    /// languages](https://cloud.google.com/dialogflow/cx/docs/reference/language)
    /// are supported.
    /// Note: languages must be enabled in the agent before they can be used.
    #[prost(string, tag = "2")]
    pub language_code: std::string::String,
    /// The mask to control which fields get updated.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// The request message for [EntityTypes.DeleteEntityType][google.cloud.dialogflow.cx.v3beta1.EntityTypes.DeleteEntityType].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteEntityTypeRequest {
    /// Required. The name of the entity type to delete.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/entityTypes/<Entity Type ID>`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// This field has no effect for entity type not being used.
    /// For entity types that are used by intents or pages:
    ///
    /// *  If `force` is set to false, an error will be returned with message
    ///    indicating the referencing resources.
    /// *  If `force` is set to true, Dialogflow will remove the entity type, as
    ///    well as any references to the entity type (i.e. Page
    ///    [parameter][google.cloud.dialogflow.cx.v3beta1.Form.Parameter] of the entity type will be changed to
    ///    '@sys.any' and intent [parameter][google.cloud.dialogflow.cx.v3beta1.Intent.Parameter] of the entity type
    ///    will be removed).
    #[prost(bool, tag = "2")]
    pub force: bool,
}
#[doc = r" Generated client implementations."]
pub mod entity_types_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service for managing [EntityTypes][google.cloud.dialogflow.cx.v3beta1.EntityType]."]
    pub struct EntityTypesClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl EntityTypesClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> EntityTypesClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        #[doc = " Returns the list of all entity types in the specified agent."]
        pub async fn list_entity_types(
            &mut self,
            request: impl tonic::IntoRequest<super::ListEntityTypesRequest>,
        ) -> Result<tonic::Response<super::ListEntityTypesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3beta1.EntityTypes/ListEntityTypes",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Retrieves the specified entity type."]
        pub async fn get_entity_type(
            &mut self,
            request: impl tonic::IntoRequest<super::GetEntityTypeRequest>,
        ) -> Result<tonic::Response<super::EntityType>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3beta1.EntityTypes/GetEntityType",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates an entity type in the specified agent."]
        pub async fn create_entity_type(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateEntityTypeRequest>,
        ) -> Result<tonic::Response<super::EntityType>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3beta1.EntityTypes/CreateEntityType",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the specified entity type."]
        pub async fn update_entity_type(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateEntityTypeRequest>,
        ) -> Result<tonic::Response<super::EntityType>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3beta1.EntityTypes/UpdateEntityType",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes the specified entity type."]
        pub async fn delete_entity_type(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteEntityTypeRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3beta1.EntityTypes/DeleteEntityType",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for EntityTypesClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for EntityTypesClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "EntityTypesClient {{ ... }}")
        }
    }
}
/// Session entity types are referred to as **User** entity types and are
/// entities that are built for an individual user such as favorites,
/// preferences, playlists, and so on.
///
/// You can redefine a session entity type at the session level to extend or
/// replace a [custom entity type][google.cloud.dialogflow.cx.v3beta1.EntityType] at the user session level (we
/// refer to the entity types defined at the agent level as "custom entity
/// types").
///
/// Note: session entity types apply to all queries, regardless of the language.
///
/// For more information about entity types, see the [Dialogflow
/// documentation](https://cloud.google.com/dialogflow/docs/entities-overview).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SessionEntityType {
    /// Required. The unique identifier of the session entity type.
    /// Format: `projects/<Project ID>/locations/<Location
    /// ID>/agents/<Agent ID>/sessions/<Session ID>/entityTypes/<Entity Type
    /// ID>` or `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/environments/<Environment ID>/sessions/<Session ID>/entityTypes/<Entity
    /// Type ID>`. If `Environment ID` is not specified, we assume default 'draft'
    /// environment.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. Indicates whether the additional data should override or supplement the
    /// custom entity type definition.
    #[prost(enumeration = "session_entity_type::EntityOverrideMode", tag = "3")]
    pub entity_override_mode: i32,
    /// Required. The collection of entities to override or supplement the custom entity
    /// type.
    #[prost(message, repeated, tag = "4")]
    pub entities: ::std::vec::Vec<entity_type::Entity>,
}
pub mod session_entity_type {
    /// The types of modifications for the session entity type.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum EntityOverrideMode {
        /// Not specified. This value should be never used.
        Unspecified = 0,
        /// The collection of session entities overrides the collection of entities
        /// in the corresponding custom entity type.
        Override = 1,
        /// The collection of session entities extends the collection of entities in
        /// the corresponding custom entity type.
        ///
        /// Note: Even in this override mode calls to `ListSessionEntityTypes`,
        /// `GetSessionEntityType`, `CreateSessionEntityType` and
        /// `UpdateSessionEntityType` only return the additional entities added in
        /// this session entity type. If you want to get the supplemented list,
        /// please call [EntityTypes.GetEntityType][google.cloud.dialogflow.cx.v3beta1.EntityTypes.GetEntityType] on the custom entity type
        /// and merge.
        Supplement = 2,
    }
}
/// The request message for [SessionEntityTypes.ListSessionEntityTypes][google.cloud.dialogflow.cx.v3beta1.SessionEntityTypes.ListSessionEntityTypes].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSessionEntityTypesRequest {
    /// Required. The session to list all session entity types from.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/sessions/<Session ID>` or `projects/<Project ID>/locations/<Location
    /// ID>/agents/<Agent ID>/environments/<Environment ID>/sessions/<Session ID>`.
    /// If `Environment ID` is not specified, we assume default 'draft'
    /// environment.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// The maximum number of items to return in a single page. By default 100 and
    /// at most 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous list request.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
}
/// The response message for [SessionEntityTypes.ListSessionEntityTypes][google.cloud.dialogflow.cx.v3beta1.SessionEntityTypes.ListSessionEntityTypes].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSessionEntityTypesResponse {
    /// The list of session entity types. There will be a maximum number of items
    /// returned based on the page_size field in the request.
    #[prost(message, repeated, tag = "1")]
    pub session_entity_types: ::std::vec::Vec<SessionEntityType>,
    /// Token to retrieve the next page of results, or empty if there are no
    /// more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// The request message for [SessionEntityTypes.GetSessionEntityType][google.cloud.dialogflow.cx.v3beta1.SessionEntityTypes.GetSessionEntityType].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSessionEntityTypeRequest {
    /// Required. The name of the session entity type.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/sessions/<Session ID>/entityTypes/<Entity Type ID>` or
    /// `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/environments/<Environment ID>/sessions/<Session ID>/entityTypes/<Entity
    /// Type ID>`. If `Environment ID` is not specified, we assume default 'draft'
    /// environment.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// The request message for [SessionEntityTypes.CreateSessionEntityType][google.cloud.dialogflow.cx.v3beta1.SessionEntityTypes.CreateSessionEntityType].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSessionEntityTypeRequest {
    /// Required. The session to create a session entity type for.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/sessions/<Session ID>` or `projects/<Project ID>/locations/<Location
    /// ID>/agents/<Agent ID>/environments/<Environment ID>/sessions/<Session ID>`.
    /// If `Environment ID` is not specified, we assume default 'draft'
    /// environment.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The session entity type to create.
    #[prost(message, optional, tag = "2")]
    pub session_entity_type: ::std::option::Option<SessionEntityType>,
}
/// The request message for [SessionEntityTypes.UpdateSessionEntityType][google.cloud.dialogflow.cx.v3beta1.SessionEntityTypes.UpdateSessionEntityType].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSessionEntityTypeRequest {
    /// Required. The session entity type to update.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/sessions/<Session ID>/entityTypes/<Entity Type ID>` or
    /// `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/environments/<Environment ID>/sessions/<Session ID>/entityTypes/<Entity
    /// Type ID>`. If `Environment ID` is not specified, we assume default 'draft'
    /// environment.
    #[prost(message, optional, tag = "1")]
    pub session_entity_type: ::std::option::Option<SessionEntityType>,
    /// The mask to control which fields get updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// The request message for [SessionEntityTypes.DeleteSessionEntityType][google.cloud.dialogflow.cx.v3beta1.SessionEntityTypes.DeleteSessionEntityType].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteSessionEntityTypeRequest {
    /// Required. The name of the session entity type to delete.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/sessions/<Session ID>/entityTypes/<Entity Type ID>` or
    /// `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/environments/<Environment ID>/sessions/<Session ID>/entityTypes/<Entity
    /// Type ID>`. If `Environment ID` is not specified, we assume default 'draft'
    /// environment.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod session_entity_types_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service for managing [SessionEntityTypes][google.cloud.dialogflow.cx.v3beta1.SessionEntityType]."]
    pub struct SessionEntityTypesClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SessionEntityTypesClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> SessionEntityTypesClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        #[doc = " Returns the list of all session entity types in the specified session."]
        pub async fn list_session_entity_types(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSessionEntityTypesRequest>,
        ) -> Result<tonic::Response<super::ListSessionEntityTypesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3beta1.SessionEntityTypes/ListSessionEntityTypes",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Retrieves the specified session entity type."]
        pub async fn get_session_entity_type(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSessionEntityTypeRequest>,
        ) -> Result<tonic::Response<super::SessionEntityType>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3beta1.SessionEntityTypes/GetSessionEntityType",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a session entity type."]
        pub async fn create_session_entity_type(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateSessionEntityTypeRequest>,
        ) -> Result<tonic::Response<super::SessionEntityType>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3beta1.SessionEntityTypes/CreateSessionEntityType",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the specified session entity type."]
        pub async fn update_session_entity_type(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSessionEntityTypeRequest>,
        ) -> Result<tonic::Response<super::SessionEntityType>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3beta1.SessionEntityTypes/UpdateSessionEntityType",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes the specified session entity type."]
        pub async fn delete_session_entity_type(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteSessionEntityTypeRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3beta1.SessionEntityTypes/DeleteSessionEntityType",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for SessionEntityTypesClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for SessionEntityTypesClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "SessionEntityTypesClient {{ ... }}")
        }
    }
}
/// The request to detect user's intent.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DetectIntentRequest {
    /// Required. The name of the session this query is sent to.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/sessions/<Session ID>` or `projects/<Project ID>/locations/<Location
    /// ID>/agents/<Agent ID>/environments/<Environment ID>/sessions/<Session ID>`.
    /// If `Environment ID` is not specified, we assume default 'draft'
    /// environment.
    /// It's up to the API caller to choose an appropriate `Session ID`. It can be
    /// a random number or some type of session identifiers (preferably hashed).
    /// The length of the `Session ID` must not exceed 36 characters.
    ///
    /// For more information, see the [sessions
    /// guide](https://cloud.google.com/dialogflow/cx/docs/concept/session).
    ///
    /// Note: Always use agent versions for production traffic.
    /// See [Versions and
    /// environments](https://cloud.google.com/dialogflow/cx/docs/concept/version).
    #[prost(string, tag = "1")]
    pub session: std::string::String,
    /// The parameters of this query.
    #[prost(message, optional, tag = "2")]
    pub query_params: ::std::option::Option<QueryParameters>,
    /// Required. The input specification.
    #[prost(message, optional, tag = "3")]
    pub query_input: ::std::option::Option<QueryInput>,
    /// Instructs the speech synthesizer how to generate the output audio.
    #[prost(message, optional, tag = "4")]
    pub output_audio_config: ::std::option::Option<OutputAudioConfig>,
}
/// The message returned from the DetectIntent method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DetectIntentResponse {
    /// Output only. The unique identifier of the response. It can be used to
    /// locate a response in the training example set or for reporting issues.
    #[prost(string, tag = "1")]
    pub response_id: std::string::String,
    /// The result of the conversational query.
    #[prost(message, optional, tag = "2")]
    pub query_result: ::std::option::Option<QueryResult>,
    /// The audio data bytes encoded as specified in the request.
    /// Note: The output audio is generated based on the values of default platform
    /// text responses found in the
    /// [`query_result.response_messages`][google.cloud.dialogflow.cx.v3beta1.QueryResult.response_messages] field. If
    /// multiple default text responses exist, they will be concatenated when
    /// generating audio. If no default platform text responses exist, the
    /// generated audio content will be empty.
    ///
    /// In some scenarios, multiple output audio fields may be present in the
    /// response structure. In these cases, only the top-most-level audio output
    /// has content.
    #[prost(bytes, tag = "4")]
    pub output_audio: std::vec::Vec<u8>,
    /// The config used by the speech synthesizer to generate the output audio.
    #[prost(message, optional, tag = "5")]
    pub output_audio_config: ::std::option::Option<OutputAudioConfig>,
}
/// The top-level message sent by the client to the
/// [Sessions.StreamingDetectIntent][google.cloud.dialogflow.cx.v3beta1.Sessions.StreamingDetectIntent] method.
///
/// Multiple request messages should be sent in order:
///
/// 1.  The first message must contain
/// [session][google.cloud.dialogflow.cx.v3beta1.StreamingDetectIntentRequest.session],
///     [query_input][google.cloud.dialogflow.cx.v3beta1.StreamingDetectIntentRequest.query_input] plus optionally
///     [query_params][google.cloud.dialogflow.cx.v3beta1.StreamingDetectIntentRequest.query_params]. If the client
///     wants to receive an audio response, it should also contain
///     [output_audio_config][google.cloud.dialogflow.cx.v3beta1.StreamingDetectIntentRequest.output_audio_config].
///
/// 2.  If [query_input][google.cloud.dialogflow.cx.v3beta1.StreamingDetectIntentRequest.query_input] was set to
///     [query_input.audio.config][google.cloud.dialogflow.cx.v3beta1.AudioInput.config], all subsequent messages
///     must contain [query_input.audio.audio][google.cloud.dialogflow.cx.v3beta1.AudioInput.audio] to continue with
///     Speech recognition.
///     If you decide to rather detect an intent from text
///     input after you already started Speech recognition, please send a message
///     with [query_input.text][google.cloud.dialogflow.cx.v3beta1.QueryInput.text].
///
///     However, note that:
///
///     * Dialogflow will bill you for the audio duration so far.
///     * Dialogflow discards all Speech recognition results in favor of the
///       input text.
///     * Dialogflow will use the language code from the first message.
///
/// After you sent all input, you must half-close or abort the request stream.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamingDetectIntentRequest {
    /// The name of the session this query is sent to.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/sessions/<Session ID>` or `projects/<Project ID>/locations/<Location
    /// ID>/agents/<Agent ID>/environments/<Environment ID>/sessions/<Session ID>`.
    /// If `Environment ID` is not specified, we assume default 'draft'
    /// environment.
    /// It's up to the API caller to choose an appropriate `Session ID`. It can be
    /// a random number or some type of session identifiers (preferably hashed).
    /// The length of the `Session ID` must not exceed 36 characters.
    /// Note: session must be set in the first request.
    ///
    /// For more information, see the [sessions
    /// guide](https://cloud.google.com/dialogflow/cx/docs/concept/session).
    ///
    /// Note: Always use agent versions for production traffic.
    /// See [Versions and
    /// environments](https://cloud.google.com/dialogflow/cx/docs/concept/version).
    #[prost(string, tag = "1")]
    pub session: std::string::String,
    /// The parameters of this query.
    #[prost(message, optional, tag = "2")]
    pub query_params: ::std::option::Option<QueryParameters>,
    /// Required. The input specification.
    #[prost(message, optional, tag = "3")]
    pub query_input: ::std::option::Option<QueryInput>,
    /// Instructs the speech synthesizer how to generate the output audio.
    #[prost(message, optional, tag = "4")]
    pub output_audio_config: ::std::option::Option<OutputAudioConfig>,
}
/// The top-level message returned from the `StreamingDetectIntent` method.
///
/// Multiple response messages can be returned in order:
///
/// 1.  If the input was set to streaming audio, the first one or more messages
///     contain `recognition_result`. Each `recognition_result` represents a more
///     complete transcript of what the user said. The last `recognition_result`
///     has `is_final` set to `true`.
///
/// 2.  The last message contains `detect_intent_response`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamingDetectIntentResponse {
    /// The output response.
    #[prost(oneof = "streaming_detect_intent_response::Response", tags = "1, 2")]
    pub response: ::std::option::Option<streaming_detect_intent_response::Response>,
}
pub mod streaming_detect_intent_response {
    /// The output response.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        /// The result of speech recognition.
        #[prost(message, tag = "1")]
        RecognitionResult(super::StreamingRecognitionResult),
        /// The response from detect intent.
        #[prost(message, tag = "2")]
        DetectIntentResponse(super::DetectIntentResponse),
    }
}
/// Contains a speech recognition result corresponding to a portion of the audio
/// that is currently being processed or an indication that this is the end
/// of the single requested utterance.
///
/// Example:
///
/// 1.  transcript: "tube"
///
/// 2.  transcript: "to be a"
///
/// 3.  transcript: "to be"
///
/// 4.  transcript: "to be or not to be"
///     is_final: true
///
/// 5.  transcript: " that's"
///
/// 6.  transcript: " that is"
///
/// 7.  message_type: `END_OF_SINGLE_UTTERANCE`
///
/// 8.  transcript: " that is the question"
///     is_final: true
///
/// Only two of the responses contain final results (#4 and #8 indicated by
/// `is_final: true`). Concatenating these generates the full transcript: "to be
/// or not to be that is the question".
///
/// In each response we populate:
///
/// *  for `TRANSCRIPT`: `transcript` and possibly `is_final`.
///
/// *  for `END_OF_SINGLE_UTTERANCE`: only `message_type`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamingRecognitionResult {
    /// Type of the result message.
    #[prost(enumeration = "streaming_recognition_result::MessageType", tag = "1")]
    pub message_type: i32,
    /// Transcript text representing the words that the user spoke.
    /// Populated if and only if `message_type` = `TRANSCRIPT`.
    #[prost(string, tag = "2")]
    pub transcript: std::string::String,
    /// If `false`, the `StreamingRecognitionResult` represents an
    /// interim result that may change. If `true`, the recognizer will not return
    /// any further hypotheses about this piece of the audio. May only be populated
    /// for `message_type` = `TRANSCRIPT`.
    #[prost(bool, tag = "3")]
    pub is_final: bool,
    /// The Speech confidence between 0.0 and 1.0 for the current portion of audio.
    /// A higher number indicates an estimated greater likelihood that the
    /// recognized words are correct. The default of 0.0 is a sentinel value
    /// indicating that confidence was not set.
    ///
    /// This field is typically only provided if `is_final` is true and you should
    /// not rely on it being accurate or even set.
    #[prost(float, tag = "4")]
    pub confidence: f32,
    /// An estimate of the likelihood that the speech recognizer will
    /// not change its guess about this interim recognition result:
    /// * If the value is unspecified or 0.0, Dialogflow didn't compute the
    ///   stability. In particular, Dialogflow will only provide stability for
    ///   `TRANSCRIPT` results with `is_final = false`.
    /// * Otherwise, the value is in (0.0, 1.0] where 0.0 means completely
    ///   unstable and 1.0 means completely stable.
    #[prost(float, tag = "6")]
    pub stability: f32,
    /// Word-specific information for the words recognized by Speech in
    /// [transcript][google.cloud.dialogflow.cx.v3beta1.StreamingRecognitionResult.transcript]. Populated if and only if `message_type` = `TRANSCRIPT` and
    /// [InputAudioConfig.enable_word_info] is set.
    #[prost(message, repeated, tag = "7")]
    pub speech_word_info: ::std::vec::Vec<SpeechWordInfo>,
    /// Time offset of the end of this Speech recognition result relative to the
    /// beginning of the audio. Only populated for `message_type` =
    /// `TRANSCRIPT`.
    #[prost(message, optional, tag = "8")]
    pub speech_end_offset: ::std::option::Option<::prost_types::Duration>,
}
pub mod streaming_recognition_result {
    /// Type of the response message.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum MessageType {
        /// Not specified. Should never be used.
        Unspecified = 0,
        /// Message contains a (possibly partial) transcript.
        Transcript = 1,
        /// Event indicates that the server has detected the end of the user's speech
        /// utterance and expects no additional speech. Therefore, the server will
        /// not process additional audio (although it may subsequently return
        /// additional results). The client should stop sending additional audio
        /// data, half-close the gRPC connection, and wait for any additional results
        /// until the server closes the gRPC connection. This message is only sent if
        /// [`single_utterance`][google.cloud.dialogflow.cx.v3beta1.InputAudioConfig.single_utterance] was set to
        /// `true`, and is not used otherwise.
        EndOfSingleUtterance = 2,
    }
}
/// Represents the parameters of a conversational query.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParameters {
    /// The time zone of this conversational query from the [time zone
    /// database](https://www.iana.org/time-zones), e.g., America/New_York,
    /// Europe/Paris. If not provided, the time zone specified in the agent is
    /// used.
    #[prost(string, tag = "1")]
    pub time_zone: std::string::String,
    /// The geo location of this conversational query.
    #[prost(message, optional, tag = "2")]
    pub geo_location: ::std::option::Option<super::super::super::super::r#type::LatLng>,
    /// Additional session entity types to replace or extend developer entity types
    /// with. The entity synonyms apply to all languages and persist for the
    /// session of this query.
    #[prost(message, repeated, tag = "3")]
    pub session_entity_types: ::std::vec::Vec<SessionEntityType>,
    /// This field can be used to pass custom data into the webhook associated with
    /// the agent. Arbitrary JSON objects are supported.
    #[prost(message, optional, tag = "4")]
    pub payload: ::std::option::Option<::prost_types::Struct>,
    /// Additional parameters to be put into [session
    /// parameters][SessionInfo.parameters]. To remove a
    /// parameter from the session, clients should explicitly set the parameter
    /// value to null.
    ///
    /// Depending on your protocol or client library language, this is a
    /// map, associative array, symbol table, dictionary, or JSON object
    /// composed of a collection of (MapKey, MapValue) pairs:
    ///
    /// -   MapKey type: string
    /// -   MapKey value: parameter name
    /// -   MapValue type:
    ///     -   If parameter's entity type is a composite entity: map
    ///     -   Else: depending on parameter value type, could be one of string,
    ///     number, boolean, null, list or map
    /// -   MapValue value:
    ///     -   If parameter's entity type is a composite entity:
    ///         map from composite entity property names to property values
    ///     -   Else: parameter value
    #[prost(message, optional, tag = "5")]
    pub parameters: ::std::option::Option<::prost_types::Struct>,
    /// The unique identifier of the [page][google.cloud.dialogflow.cx.v3beta1.Page] to override the [current
    /// page][QueryResult.current_page] in the session. Format: `projects/<Project
    /// ID>/locations/<Location ID>/agents/<Agent ID>/pages/<page ID>`.
    ///
    /// If `current_page` is specified, the previous state of the session will be
    /// ignored by Dialogflow, including the [previous
    /// page][QueryResult.current_page] and the [previous session
    /// parameters][QueryResult.parameters].
    /// In most cases, [current_page][google.cloud.dialogflow.cx.v3beta1.QueryParameters.current_page] and
    /// [parameters][google.cloud.dialogflow.cx.v3beta1.QueryParameters.parameters] should be configured together to
    /// direct a session to a specific state.
    #[prost(string, tag = "6")]
    pub current_page: std::string::String,
    /// Whether to disable webhook calls for this request.
    #[prost(bool, tag = "7")]
    pub disable_webhook: bool,
    /// Configures whether sentiment analysis should be performed. If not
    /// provided, sentiment analysis is not performed.
    #[prost(bool, tag = "8")]
    pub analyze_query_text_sentiment: bool,
    /// This field can be used to pass HTTP headers for a webhook
    /// call. These headers will be sent to webhook along with the headers that
    /// have been configured through Dialogflow web console. The headers defined
    /// within this field will overwrite the headers configured through Dialogflow
    /// console if there is a conflict. Header names are case-insensitive.
    /// Google's specified headers are not allowed. Including: "Host",
    /// "Content-Length", "Connection", "From", "User-Agent", "Accept-Encoding",
    /// "If-Modified-Since", "If-None-Match", "X-Forwarded-For", etc.
    #[prost(map = "string, string", tag = "10")]
    pub webhook_headers: ::std::collections::HashMap<std::string::String, std::string::String>,
}
/// Represents the query input. It can contain one of:
///
/// 1.  A conversational query in the form of text.
///
/// 2.  An intent query that specifies which intent to trigger.
///
/// 3.  Natural language speech audio to be processed.
///
/// 4.  An event to be triggered.
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryInput {
    /// Required. The language of the input. See [Language
    /// Support](https://cloud.google.com/dialogflow/cx/docs/reference/language)
    /// for a list of the currently supported language codes. Note that queries in
    /// the same session do not necessarily need to specify the same language.
    #[prost(string, tag = "4")]
    pub language_code: std::string::String,
    /// Required. The input specification.
    #[prost(oneof = "query_input::Input", tags = "2, 3, 5, 6, 7")]
    pub input: ::std::option::Option<query_input::Input>,
}
pub mod query_input {
    /// Required. The input specification.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Input {
        /// The natural language text to be processed.
        #[prost(message, tag = "2")]
        Text(super::TextInput),
        /// The intent to be triggered.
        #[prost(message, tag = "3")]
        Intent(super::IntentInput),
        /// The natural language speech audio to be processed.
        #[prost(message, tag = "5")]
        Audio(super::AudioInput),
        /// The event to be triggered.
        #[prost(message, tag = "6")]
        Event(super::EventInput),
        /// The DTMF event to be handled.
        #[prost(message, tag = "7")]
        Dtmf(super::DtmfInput),
    }
}
/// Represents the result of a conversational query.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryResult {
    /// The language that was triggered during intent detection.
    /// See [Language
    /// Support](https://cloud.google.com/dialogflow/cx/docs/reference/language)
    /// for a list of the currently supported language codes.
    #[prost(string, tag = "2")]
    pub language_code: std::string::String,
    /// The collected [session parameters][google.cloud.dialogflow.cx.v3beta1.SessionInfo.parameters].
    ///
    /// Depending on your protocol or client library language, this is a
    /// map, associative array, symbol table, dictionary, or JSON object
    /// composed of a collection of (MapKey, MapValue) pairs:
    ///
    /// -   MapKey type: string
    /// -   MapKey value: parameter name
    /// -   MapValue type:
    ///     -   If parameter's entity type is a composite entity: map
    ///     -   Else: depending on parameter value type, could be one of string,
    ///     number, boolean, null, list or map
    /// -   MapValue value:
    ///     -   If parameter's entity type is a composite entity:
    ///         map from composite entity property names to property values
    ///     -   Else: parameter value
    #[prost(message, optional, tag = "3")]
    pub parameters: ::std::option::Option<::prost_types::Struct>,
    /// The list of rich messages returned to the client. Responses vary from
    /// simple text messages to more sophisticated, structured payloads used
    /// to drive complex logic.
    #[prost(message, repeated, tag = "4")]
    pub response_messages: ::std::vec::Vec<ResponseMessage>,
    /// The list of webhook call status in the order of call sequence.
    #[prost(message, repeated, tag = "13")]
    pub webhook_statuses: ::std::vec::Vec<super::super::super::super::rpc::Status>,
    /// The list of webhook payload in [WebhookResponse.payload][google.cloud.dialogflow.cx.v3beta1.WebhookResponse.payload], in
    /// the order of call sequence. If some webhook call fails or doesn't return
    /// any payload, an empty `Struct` would be used instead.
    #[prost(message, repeated, tag = "6")]
    pub webhook_payloads: ::std::vec::Vec<::prost_types::Struct>,
    /// The current [Page][google.cloud.dialogflow.cx.v3beta1.Page]. Some, not all fields are filled in this message,
    /// including but not limited to `name` and `display_name`.
    #[prost(message, optional, tag = "7")]
    pub current_page: ::std::option::Option<Page>,
    /// The [Intent][google.cloud.dialogflow.cx.v3beta1.Intent] that matched the conversational query. Some, not all fields
    /// are filled in this message, including but not limited to: `name` and
    /// `display_name`.
    /// This field is deprecated, please use [QueryResult.match][google.cloud.dialogflow.cx.v3beta1.QueryResult.match] instead.
    #[prost(message, optional, tag = "8")]
    pub intent: ::std::option::Option<Intent>,
    /// The intent detection confidence. Values range from 0.0 (completely
    /// uncertain) to 1.0 (completely certain).
    /// This value is for informational purpose only and is only used to
    /// help match the best intent within the classification threshold.
    /// This value may change for the same end-user expression at any time due to a
    /// model retraining or change in implementation.
    /// This field is deprecated, please use [QueryResult.match][google.cloud.dialogflow.cx.v3beta1.QueryResult.match] instead.
    #[prost(float, tag = "9")]
    pub intent_detection_confidence: f32,
    /// Intent match result, could be an intent or an event.
    #[prost(message, optional, tag = "15")]
    pub r#match: ::std::option::Option<Match>,
    /// The free-form diagnostic info. For example, this field could contain
    /// webhook call latency. The string keys of the Struct's fields map can change
    /// without notice.
    #[prost(message, optional, tag = "10")]
    pub diagnostic_info: ::std::option::Option<::prost_types::Struct>,
    /// The sentiment analyss result, which depends on
    /// [`analyze_query_text_sentiment`]
    /// [google.cloud.dialogflow.cx.v3beta1.QueryParameters.analyze_query_text_sentiment], specified in the request.
    #[prost(message, optional, tag = "17")]
    pub sentiment_analysis_result: ::std::option::Option<SentimentAnalysisResult>,
    /// The original conversational query.
    #[prost(oneof = "query_result::Query", tags = "1, 11, 12, 14")]
    pub query: ::std::option::Option<query_result::Query>,
}
pub mod query_result {
    /// The original conversational query.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Query {
        /// If [natural language text][google.cloud.dialogflow.cx.v3beta1.TextInput] was provided as input, this field
        /// will contain a copy of the text.
        #[prost(string, tag = "1")]
        Text(std::string::String),
        /// If an [intent][google.cloud.dialogflow.cx.v3beta1.IntentInput] was provided as input, this field will
        /// contain a copy of the intent identifier.
        /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
        /// ID>/intents/<Intent ID>`.
        #[prost(string, tag = "11")]
        TriggerIntent(std::string::String),
        /// If [natural language speech audio][google.cloud.dialogflow.cx.v3beta1.AudioInput] was provided as input,
        /// this field will contain the transcript for the audio.
        #[prost(string, tag = "12")]
        Transcript(std::string::String),
        /// If an [event][google.cloud.dialogflow.cx.v3beta1.EventInput] was provided as input, this field will contain
        /// the name of the event.
        #[prost(string, tag = "14")]
        TriggerEvent(std::string::String),
    }
}
/// Represents the natural language text to be processed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextInput {
    /// Required. The UTF-8 encoded natural language text to be processed. Text length must
    /// not exceed 256 characters.
    #[prost(string, tag = "1")]
    pub text: std::string::String,
}
/// Represents the intent to trigger programmatically rather than as a result of
/// natural language processing.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntentInput {
    /// Required. The unique identifier of the intent.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/intents/<Intent ID>`.
    #[prost(string, tag = "1")]
    pub intent: std::string::String,
}
/// Represents the natural speech audio to be processed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AudioInput {
    /// Required. Instructs the speech recognizer how to process the speech audio.
    #[prost(message, optional, tag = "1")]
    pub config: ::std::option::Option<InputAudioConfig>,
    /// The natural language speech audio to be processed.
    /// A single request can contain up to 1 minute of speech audio data.
    /// The [transcribed text][google.cloud.dialogflow.cx.v3beta1.QueryResult.transcript] cannot contain more than 256
    /// bytes.
    ///
    /// For non-streaming audio detect intent, both `config` and `audio` must be
    /// provided.
    /// For streaming audio detect intent, `config` must be provided in
    /// the first request and `audio` must be provided in all following requests.
    #[prost(bytes, tag = "2")]
    pub audio: std::vec::Vec<u8>,
}
/// Represents the event to trigger.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventInput {
    /// Name of the event.
    #[prost(string, tag = "1")]
    pub event: std::string::String,
}
/// Represents the input for dtmf event.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DtmfInput {
    /// The dtmf digits.
    #[prost(string, tag = "1")]
    pub digits: std::string::String,
    /// The finish digit (if any).
    #[prost(string, tag = "2")]
    pub finish_digit: std::string::String,
}
/// Represents one match result of [MatchIntent][].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Match {
    /// The [Intent][google.cloud.dialogflow.cx.v3beta1.Intent] that matched the query. Some, not all fields are filled in
    /// this message, including but not limited to: `name` and `display_name`. Only
    /// filled for [`INTENT`][google.cloud.dialogflow.cx.v3beta1.Match.MatchType] match type.
    #[prost(message, optional, tag = "1")]
    pub intent: ::std::option::Option<Intent>,
    /// The event that matched the query. Only filled for
    /// [`EVENT`][google.cloud.dialogflow.cx.v3beta1.Match.MatchType] match type.
    #[prost(string, tag = "6")]
    pub event: std::string::String,
    /// The collection of parameters extracted from the query.
    ///
    /// Depending on your protocol or client library language, this is a
    /// map, associative array, symbol table, dictionary, or JSON object
    /// composed of a collection of (MapKey, MapValue) pairs:
    ///
    /// -   MapKey type: string
    /// -   MapKey value: parameter name
    /// -   MapValue type:
    ///     -   If parameter's entity type is a composite entity: map
    ///     -   Else: depending on parameter value type, could be one of string,
    ///     number, boolean, null, list or map
    /// -   MapValue value:
    ///     -   If parameter's entity type is a composite entity:
    ///         map from composite entity property names to property values
    ///     -   Else: parameter value
    #[prost(message, optional, tag = "2")]
    pub parameters: ::std::option::Option<::prost_types::Struct>,
    /// Final text input which was matched during MatchIntent. This value can be
    /// different from original input sent in request because of spelling
    /// correction or other processing.
    #[prost(string, tag = "3")]
    pub resolved_input: std::string::String,
    /// Type of this [Match][google.cloud.dialogflow.cx.v3beta1.Match].
    #[prost(enumeration = "r#match::MatchType", tag = "4")]
    pub match_type: i32,
    /// The confidence of this match. Values range from 0.0 (completely uncertain)
    /// to 1.0 (completely certain).
    /// This value is for informational purpose only and is only used to help match
    /// the best intent within the classification threshold. This value may change
    /// for the same end-user expression at any time due to a model retraining or
    /// change in implementation.
    #[prost(float, tag = "5")]
    pub confidence: f32,
}
pub mod r#match {
    /// Type of a Match.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum MatchType {
        /// Not specified. Should never be used.
        Unspecified = 0,
        /// The query was matched to an intent.
        Intent = 1,
        /// The query directly triggered an intent.
        DirectIntent = 2,
        /// The query was used for parameter filling.
        ParameterFilling = 3,
        /// No match was found for the query.
        NoMatch = 4,
        /// Indicates an empty query.
        NoInput = 5,
        /// The query directly triggered an event.
        Event = 6,
    }
}
/// Request of [MatchIntent][].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MatchIntentRequest {
    /// Required. The name of the session this query is sent to.
    /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
    /// ID>/sessions/<Session ID>` or `projects/<Project ID>/locations/<Location
    /// ID>/agents/<Agent ID>/environments/<Environment ID>/sessions/<Session ID>`.
    /// If `Environment ID` is not specified, we assume default 'draft'
    /// environment.
    /// It's up to the API caller to choose an appropriate `Session ID`. It can be
    /// a random number or some type of session identifiers (preferably hashed).
    /// The length of the `Session ID` must not exceed 36 characters.
    ///
    /// For more information, see the [sessions
    /// guide](https://cloud.google.com/dialogflow/cx/docs/concept/session).
    #[prost(string, tag = "1")]
    pub session: std::string::String,
    /// The parameters of this query.
    #[prost(message, optional, tag = "2")]
    pub query_params: ::std::option::Option<QueryParameters>,
    /// Required. The input specification.
    #[prost(message, optional, tag = "3")]
    pub query_input: ::std::option::Option<QueryInput>,
}
/// Response of [MatchIntent][].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MatchIntentResponse {
    /// Match results, if more than one, ordered descendingly by the confidence
    /// we have that the particular intent matches the query.
    #[prost(message, repeated, tag = "4")]
    pub matches: ::std::vec::Vec<Match>,
    /// The current [Page][google.cloud.dialogflow.cx.v3beta1.Page]. Some, not all fields are filled in this message,
    /// including but not limited to `name` and `display_name`.
    #[prost(message, optional, tag = "5")]
    pub current_page: ::std::option::Option<Page>,
    /// The original conversational query.
    #[prost(oneof = "match_intent_response::Query", tags = "1, 2, 3, 6")]
    pub query: ::std::option::Option<match_intent_response::Query>,
}
pub mod match_intent_response {
    /// The original conversational query.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Query {
        /// If [natural language text][google.cloud.dialogflow.cx.v3beta1.TextInput] was provided as input, this field
        /// will contain a copy of the text.
        #[prost(string, tag = "1")]
        Text(std::string::String),
        /// If an [intent][google.cloud.dialogflow.cx.v3beta1.IntentInput] was provided as input, this field will
        /// contain a copy of the intent identifier.
        /// Format: `projects/<Project ID>/locations/<Location ID>/agents/<Agent
        /// ID>/intents/<Intent ID>`.
        #[prost(string, tag = "2")]
        TriggerIntent(std::string::String),
        /// If [natural language speech audio][google.cloud.dialogflow.cx.v3beta1.AudioInput] was provided as input,
        /// this field will contain the transcript for the audio.
        #[prost(string, tag = "3")]
        Transcript(std::string::String),
        /// If an [event][google.cloud.dialogflow.cx.v3beta1.EventInput] was provided as input, this field will
        /// contain a copy of the event name.
        #[prost(string, tag = "6")]
        TriggerEvent(std::string::String),
    }
}
/// Request of [FulfillIntent][]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FulfillIntentRequest {
    /// Must be same as the corresponding MatchIntent request, otherwise the
    /// behavior is undefined.
    #[prost(message, optional, tag = "1")]
    pub match_intent_request: ::std::option::Option<MatchIntentRequest>,
    /// The matched intent/event to fulfill.
    #[prost(message, optional, tag = "2")]
    pub r#match: ::std::option::Option<Match>,
    /// Instructs the speech synthesizer how to generate output audio.
    #[prost(message, optional, tag = "3")]
    pub output_audio_config: ::std::option::Option<OutputAudioConfig>,
}
/// Response of [FulfillIntent][]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FulfillIntentResponse {
    /// Output only. The unique identifier of the response. It can be used to
    /// locate a response in the training example set or for reporting issues.
    #[prost(string, tag = "1")]
    pub response_id: std::string::String,
    /// The result of the conversational query.
    #[prost(message, optional, tag = "2")]
    pub query_result: ::std::option::Option<QueryResult>,
    /// The audio data bytes encoded as specified in the request.
    /// Note: The output audio is generated based on the values of default platform
    /// text responses found in the
    /// [`query_result.response_messages`][google.cloud.dialogflow.cx.v3beta1.QueryResult.response_messages] field. If
    /// multiple default text responses exist, they will be concatenated when
    /// generating audio. If no default platform text responses exist, the
    /// generated audio content will be empty.
    ///
    /// In some scenarios, multiple output audio fields may be present in the
    /// response structure. In these cases, only the top-most-level audio output
    /// has content.
    #[prost(bytes, tag = "3")]
    pub output_audio: std::vec::Vec<u8>,
    /// The config used by the speech synthesizer to generate the output audio.
    #[prost(message, optional, tag = "4")]
    pub output_audio_config: ::std::option::Option<OutputAudioConfig>,
}
/// The result of sentiment analysis. Sentiment analysis inspects user input
/// and identifies the prevailing subjective opinion, especially to determine a
/// user's attitude as positive, negative, or neutral.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SentimentAnalysisResult {
    /// Sentiment score between -1.0 (negative sentiment) and 1.0 (positive
    /// sentiment).
    #[prost(float, tag = "1")]
    pub score: f32,
    /// A non-negative number in the [0, +inf) range, which represents the absolute
    /// magnitude of sentiment, regardless of score (positive or negative).
    #[prost(float, tag = "2")]
    pub magnitude: f32,
}
#[doc = r" Generated client implementations."]
pub mod sessions_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " A session represents an interaction with a user. You retrieve user input"]
    #[doc = " and pass it to the [DetectIntent][google.cloud.dialogflow.cx.v3beta1.Sessions.DetectIntent] method to determine"]
    #[doc = " user intent and respond."]
    pub struct SessionsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SessionsClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> SessionsClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        #[doc = " Processes a natural language query and returns structured, actionable data"]
        #[doc = " as a result. This method is not idempotent, because it may cause session"]
        #[doc = " entity types to be updated, which in turn might affect results of future"]
        #[doc = " queries."]
        #[doc = ""]
        #[doc = " Note: Always use agent versions for production traffic."]
        #[doc = " See [Versions and"]
        #[doc = " environments](https://cloud.google.com/dialogflow/cx/docs/concept/version)."]
        pub async fn detect_intent(
            &mut self,
            request: impl tonic::IntoRequest<super::DetectIntentRequest>,
        ) -> Result<tonic::Response<super::DetectIntentResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3beta1.Sessions/DetectIntent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Processes a natural language query in audio format in a streaming fashion"]
        #[doc = " and returns structured, actionable data as a result. This method is only"]
        #[doc = " available via the gRPC API (not REST)."]
        #[doc = ""]
        #[doc = " Note: Always use agent versions for production traffic."]
        #[doc = " See [Versions and"]
        #[doc = " environments](https://cloud.google.com/dialogflow/cx/docs/concept/version)."]
        pub async fn streaming_detect_intent(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::StreamingDetectIntentRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::StreamingDetectIntentResponse>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3beta1.Sessions/StreamingDetectIntent",
            );
            self.inner
                .streaming(request.into_streaming_request(), path, codec)
                .await
        }
        #[doc = " Returns preliminary intent match results, doesn't change the session"]
        #[doc = " status."]
        pub async fn match_intent(
            &mut self,
            request: impl tonic::IntoRequest<super::MatchIntentRequest>,
        ) -> Result<tonic::Response<super::MatchIntentResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3beta1.Sessions/MatchIntent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Fulfills a matched intent returned by [MatchIntent][google.cloud.dialogflow.cx.v3beta1.Sessions.MatchIntent]."]
        #[doc = " Must be called after [MatchIntent][google.cloud.dialogflow.cx.v3beta1.Sessions.MatchIntent], with input from"]
        #[doc = " [MatchIntentResponse][google.cloud.dialogflow.cx.v3beta1.MatchIntentResponse]. Otherwise, the behavior is undefined."]
        pub async fn fulfill_intent(
            &mut self,
            request: impl tonic::IntoRequest<super::FulfillIntentRequest>,
        ) -> Result<tonic::Response<super::FulfillIntentResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.cx.v3beta1.Sessions/FulfillIntent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for SessionsClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for SessionsClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "SessionsClient {{ ... }}")
        }
    }
}
