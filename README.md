# Google Cognitive APIs

---

Selected Google Cloud APIs exposed via GRPC and REST:

* Dialogflow (NLP)
* Speech-to-text
* Text-to-speech

## Google API proto definitions
Google proto definitions have been taken from [this](https://github.com/googleapis/googleapis) repo.

## Examples

```rust
cargo build --examples
```

```rust
cargo run --example recognizer_long_running
```

```rust
cargo run --example recognizer_streaming
```