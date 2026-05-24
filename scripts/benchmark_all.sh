#!/usr/bin/env bash
set -euo pipefail

BENCHES=(
  hf_single
  hf_batch
  parallel_batch
  template_format
  cache_lookup
  stream_adapter
  end_to_end
)

for bench in "${BENCHES[@]}"; do
  echo "==> cargo bench --bench ${bench}"
  cargo bench --bench "${bench}"
done
