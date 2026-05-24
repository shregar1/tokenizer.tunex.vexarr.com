# Contributing

TuneX tokenizer changes should preserve deterministic output, bounded memory behavior, and compatibility with `tunex-core` contracts.

Before opening a pull request:

```sh
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo doc --all-features --no-deps
cargo bench --no-run
```

Keep changes focused. Add tests for contract changes and benchmarks for performance claims.
