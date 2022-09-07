# Google Cognitive APIs

---
[![CI](https://github.com/jabber-tools/google-cognitive-apis/actions/workflows/github-actions-rust-ci.yml/badge.svg)](https://github.com/jabber-tools/google-cognitive-apis/actions/workflows/github-actions-rust-ci.yml)
[![License](https://img.shields.io/badge/License-Apache-blue.svg)](LICENSE-APACHE)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE-MIT)
[![Crates.io][crates-badge]][crates-url]
[![rustdoc][rustdoc-badge]][rustdoc-url]

[crates-badge]: https://img.shields.io/crates/v/google-cognitive-apis.svg
[crates-url]: https://crates.io/crates/google-cognitive-apis
[rustdoc-badge]: https://img.shields.io/badge/rustdoc-0.2.0-green.svg
[rustdoc-url]: https://jabber-tools.github.io/google_cognitive_apis/doc/0.2.0/google_cognitive_apis/index.html


Asynchronous Rust bindings for Google Cloud Platform cognitive gRPC APIs.
Provides high level interfaces wrapping complexity of low-level GRPC implementation. Bidirectional gRPC streaming is supported
with two alternative approaches:

* tokio.rs channels
* asynchronous streams facilitated by crate [async-stream](https://crates.io/crates/async-stream)



Following APIs are currently supported: 

| Cognitive API                                               | Feature name | Status          |
| ----------------------------------------------------- | ------------ | --------------- |
| [**Dialogflow ES**](https://cloud.google.com/dialogflow)        | `dialogflow`     | **Complete**    |
| [**Speech-to-text**](https://cloud.google.com/speech-to-text)   | `speech-to-text`  | **Complete**    |
| [**Text-to-speech**](https://cloud.google.com/text-to-speech) | `text-to-speech`    | **Complete**    |

**IMPORTANT**:  Version 0.2.0 is upgrading underlying GRPC stack (**tonic**, **prost** and **prost-build** libraries). Please not that **prost-build** now requires external Protocol Buffer compiler (protoc).
Details can be found [here](https://docs.rs/prost-build/latest/prost_build/#sourcing-protoc). You can download protoc from [github](https://github.com/protocolbuffers/protobuf/releases).

## Google API proto definitions
Google proto definitions have been taken from [this](https://github.com/googleapis/googleapis) repo.

## Limitations

* Only limited subset of Google cognitive APIs is supported. Feel free to raise PR with new additions! 
* Dialogflow CX is not yet supported.
* For Dialogflow we currently support only *SessionClient* (The purpose of this library is not support different DialogFlow management APIs).
* REST APIs are supported with single purpose: to define structs that will enable deserialization of JSON config structures and their conversion into GRPC counterparts.
Full support for REST APIs will be not introduced.

## Examples

You can find all examples [here](https://github.com/jabber-tools/google-cognitive-apis/tree/main/examples).

## License

Licensed under either Apache-2.0 or MIT license. 