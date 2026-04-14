#!/usr/bin/env bash
# Run all abaco fuzz harnesses.
# Usage: ./fuzz/run.sh [iters]   (default: 10000 per harness)
set -euo pipefail

ITERS="${1:-10000}"
CYRIUS="${CYRIUS_HOME:-$HOME/.cyrius}/bin/cyrius"

mkdir -p build

echo "=== fuzz_eval ($ITERS iters) ==="
"$CYRIUS" build fuzz/fuzz_eval.cyr build/fuzz_eval 2>&1 | tail -2
./build/fuzz_eval "$ITERS"

echo "=== fuzz_ntheory ($ITERS iters) ==="
"$CYRIUS" build fuzz/fuzz_ntheory.cyr build/fuzz_ntheory 2>&1 | tail -2
./build/fuzz_ntheory "$ITERS"

echo "=== fuzz_units ($ITERS iters) ==="
"$CYRIUS" build fuzz/fuzz_units.cyr build/fuzz_units 2>&1 | tail -2
./build/fuzz_units "$ITERS"

echo ""
echo "all fuzzers passed"
