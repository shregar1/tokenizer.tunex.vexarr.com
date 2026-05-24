# TuneX Tokenizer

High-performance Rust tokenizer engine for TuneX with batch tokenization, chat template formatting, cache contracts, streaming adapters, distributed tokenization metadata, and Hugging Face tokenizer compatibility.

## Status

`tunex-tokenizer` is in `0.1.x` contract development. APIs are usable by TuneX modules, but compatibility is not yet stable.

## What It Provides

- Hugging Face tokenizer adapter contracts.
- Batch and parallel tokenization helpers.
- Cache-friendly tokenized batch layout.
- Chat templates and special-token formatting.
- Dataset-to-tokenizer streaming adapter.
- Distributed tokenization ownership and replay metadata.
- Observability, benchmarks, and examples.

## What It Does Not Provide

- Custom tokenizer model training.
- Networked distributed runtime.
- GPU kernels.
- Full checkpoint persistence.

## Example

```rust
use tunex_tokenizer::prelude::*;

let config = TokenizerConfig {
    tokenizer_path: Some("tokenizer.json".into()),
    max_seq_len: Some(4096),
    truncate: true,
    ..TokenizerConfig::default()
};

let tokenizer = HfTokenizer::from_config(config)?;
let batch = tokenizer.tokenize_batch(&samples)?;
# Ok::<(), tunex_tokenizer::TokenizerError>(())
```

See `examples/` for runnable usage patterns.

## Quality Gates

```sh
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo doc --all-features --no-deps
cargo bench --no-run
```
