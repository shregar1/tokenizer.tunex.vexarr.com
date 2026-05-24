#!/usr/bin/env bash
set -euo pipefail

BENCH="${1:-end_to_end}"

if ! command -v cargo-flamegraph >/dev/null 2>&1 && ! cargo flamegraph --help >/dev/null 2>&1; then
  echo "cargo-flamegraph is required: cargo install flamegraph" >&2
  exit 1
fi

cargo flamegraph --bench "${BENCH}" -- --bench
