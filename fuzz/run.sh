#!/usr/bin/env bash
# Run all abaco fuzz harnesses.
# Usage: ./fuzz/run.sh [iters]   (default: 10000 per harness)
set -euo pipefail

ITERS="${1:-10000}"
CYRIUS="${CYRIUS_HOME:-$HOME/.cyrius}/bin/cyrius"
# Per-harness wall-clock cap, seconds (a hang is a failure, not a wait).
FUZZ_TIMEOUT="${FUZZ_TIMEOUT:-600}"

if ! [[ "$ITERS" =~ ^[1-9][0-9]{0,8}$ ]]; then
    echo "usage: $0 [iterations, 1-999999999]" >&2
    exit 2
fi

mkdir -p build

for f in fuzz/*.fcyr; do
    name=$(basename "$f" .fcyr)
    echo "=== $name ($ITERS iters) ==="
    CYRIUS_DCE=1 "$CYRIUS" build "$f" "build/$name" 2>&1 | tail -2
    timeout "$FUZZ_TIMEOUT" "./build/$name" "$ITERS"
done

echo ""
echo "all fuzzers passed"
