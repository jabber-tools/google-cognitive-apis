# Google Cognitive APIs
[![CI](https://github.com/jabber-tools/google-cognitive-apis/actions/workflows/github-actions-rust-ci.yml/badge.svg)](https://github.com/jabber-tools/google-cognitive-apis/actions/workflows/github-actions-rust-ci.yml)
[![License](https://img.shields.io/badge/License-Apache-blue.svg)](LICENSE-APACHE)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE-MIT)

Asynchronous Rust bindings for Google Cloud Platform cognitive gRPC APIs.
Provides high level interfaces wrapping complexity of low-level GRPC implementation. 
Following APIs are currently supported: 

| Cognitive API                                               | Feature name | Status          |
| ----------------------------------------------------- | ------------ | --------------- |
| [**Dialogflow ES**](https://cloud.google.com/dialogflow)        | `dialogflow`     | **In progress**    |
| [**Speech-to-text**](https://cloud.google.com/speech-to-text)   | `speech-to-text`  | **Complete**    |
| [**Text-to-speech**](https://cloud.google.com/text-to-speech) | `text-to-speech`    | **Complete**    |

## Google API proto definitions
Google proto definitions have been taken from [this](https://github.com/googleapis/googleapis) repo.

## Limitations

* Only limited subset of Google cognitive APIs is supported. Feel free to raise PR with new additions! 
* Dialogflow CX is not yet supported.
* For Dialogflow we currently support only *SessionClient* (The purpose of this library is not support different DialogFlow management APIs).
* REST APIs are supported with single purpose: to define structs that will enable deserialization of JSON config structures and their conversion into GRPC counterparts.
Full support for REST APIs will be not introduced.
* Dialogflow *detect intent streaming* (i.e. receiving audio data, performing speech-to-text followed by intent detection) is not fully supported.
It seems google APIs require half-close operation to be supported on audio stream to find out no more data will arrive and initiate intent detection.
Details can be found [here](https://cloud.google.com/dialogflow/es/docs/how/detect-intent-stream#detect-intent-stream-go).
Thus after streaming in all audio bytes API will simply timeout complaining data needs to be arriving promptly. If you know how to implement half-close with Rust/Tonic toolset let me know (raise PR/issue)!
Until then use *speech-to-text streaming* API and Dialogflow *detect intent* API separately to achieve the same result.

## Examples

You can find all examples [here](https://github.com/jabber-tools/google-cognitive-apis/tree/main/examples).