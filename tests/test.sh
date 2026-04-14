#!/bin/sh
# Legacy entry point — prefer `cyrius test` for the full .tcyr suite.
# Kept for convenience: smoke-compiles src/main.cyr with the installed
# compiler, then delegates to `cyrius test` for the actual test run.
set -e

cd "$(dirname "$0")/.."

echo "=== abaco smoke build ==="
mkdir -p build
cyrius build src/main.cyr build/abaco
echo "abaco: $(wc -c < build/abaco) bytes"

echo ""
echo "=== abaco tests ==="
cyrius test
