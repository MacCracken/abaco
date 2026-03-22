#!/usr/bin/env bash
set -euo pipefail

# Run criterion benchmarks and append results to a CSV history file.
# Usage:
#   ./scripts/bench-history.sh              # defaults to bench-history.csv
#   ./scripts/bench-history.sh results.csv  # custom output file

HISTORY_FILE="${1:-bench-history.csv}"
TIMESTAMP=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
COMMIT=$(git rev-parse --short HEAD 2>/dev/null || echo "unknown")
BRANCH=$(git branch --show-current 2>/dev/null || echo "unknown")

# Create header if file doesn't exist
if [ ! -f "$HISTORY_FILE" ]; then
    echo "timestamp,commit,branch,benchmark,estimate_ns" > "$HISTORY_FILE"
fi

echo "Running benchmarks..."
echo "  commit: $COMMIT"
echo "  branch: $BRANCH"
echo ""

# Run benchmarks and capture output, stripping ANSI escape codes
BENCH_OUTPUT=$(cargo bench --bench benchmarks 2>&1 | sed 's/\x1b\[[0-9;]*m//g')

# Show full output
echo "$BENCH_OUTPUT"
echo ""

# Parse criterion output and append to CSV.
# Criterion lines look like:
#   eval_simple/addition    time:   [133.91 ns 134.16 ns 134.44 ns]
# We extract the middle value (point estimate) and normalize to nanoseconds.
LINES_ADDED=0
while IFS= read -r line; do
    # Extract benchmark name: everything before "time:"
    BENCH_NAME=$(echo "$line" | sed -E 's/[[:space:]]*time:.*//' | xargs)

    # Extract the bracket contents
    VALS=$(echo "$line" | sed -E 's/.*\[(.+)\]/\1/')
    # Middle value (point estimate) is tokens 3 and 4 (value + unit)
    MEDIAN=$(echo "$VALS" | awk '{print $3}')
    UNIT=$(echo "$VALS" | awk '{print $4}')

    # Normalize to nanoseconds
    case "$UNIT" in
        ps)  NS=$(echo "$MEDIAN" | awk '{printf "%.4f", $1 / 1000}') ;;
        ns)  NS="$MEDIAN" ;;
        µs|us)  NS=$(echo "$MEDIAN" | awk '{printf "%.4f", $1 * 1000}') ;;
        ms)  NS=$(echo "$MEDIAN" | awk '{printf "%.4f", $1 * 1000000}') ;;
        s)   NS=$(echo "$MEDIAN" | awk '{printf "%.4f", $1 * 1000000000}') ;;
        *)   NS="$MEDIAN" ;;
    esac

    echo "${TIMESTAMP},${COMMIT},${BRANCH},${BENCH_NAME},${NS}" >> "$HISTORY_FILE"
    LINES_ADDED=$((LINES_ADDED + 1))
done < <(echo "$BENCH_OUTPUT" | grep 'time:.*\[')

echo "Appended ${LINES_ADDED} benchmark entries to ${HISTORY_FILE}"
