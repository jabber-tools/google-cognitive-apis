# Google Cognitive APIs

---

Selected Google Cloud APIs exposed via GRPC and REST:

* Dialogflow (NLP)
* Speech-to-text
* Text-to-speech

This version is based on Tokio 0.2 library and will be not extended significantly. All the goodies will go into version based on Tokio 1.0. 

## Google API proto definitions
Google proto definitions have been taken from [this](https://github.com/googleapis/googleapis) repo.

## Examples

### How to build

```rust
cargo build --examples
```

### How to run

```rust
cargo run --example recognizer_sync
```

```rust
cargo run --example recognizer_async
```

```rust
cargo run --example recognizer_streaming
```