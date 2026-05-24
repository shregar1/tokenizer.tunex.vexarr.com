#!/usr/bin/env bash
set -euo pipefail

BENCH="${1:-end_to_end}"

if ! command -v perf >/dev/null 2>&1; then
  echo "perf is required and is generally available on Linux only" >&2
  exit 1
fi

cargo bench --bench "${BENCH}" --no-run
TARGET="$(find target/release/deps -maxdepth 1 -type f -perm -111 -name "${BENCH}-*" | head -n 1)"

if [[ -z "${TARGET}" ]]; then
  echo "could not locate compiled benchmark executable for ${BENCH}" >&2
  exit 1
fi

perf record --call-graph=dwarf "${TARGET}" --bench
perf report
