#!/usr/bin/env bash
# Run all abaco fuzz harnesses.
# Usage: ./fuzz/run.sh [iters]   (default: 10000 per harness)
set -euo pipefail

ITERS="${1:-10000}"
CYRIUS="${CYRIUS_HOME:-$HOME/.cyrius}/bin/cyrius"

mkdir -p build

for f in fuzz/*.fcyr; do
    name=$(basename "$f" .fcyr)
    echo "=== $name ($ITERS iters) ==="
    CYRIUS_DCE=1 "$CYRIUS" build "$f" "build/$name" 2>&1 | tail -2
    "./build/$name" "$ITERS"
done

echo ""
echo "all fuzzers passed"
