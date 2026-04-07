#!/bin/sh
CC="${1:-./build/cc2}"
echo "=== abaco tests ==="
cat src/main.cyr | "$CC" > /tmp/abaco_test && chmod +x /tmp/abaco_test && /tmp/abaco_test
echo "exit: $?"
rm -f /tmp/abaco_test
