#!/usr/bin/env bash
# `set -e` alone; `pipefail` triggered SIGPIPE 141 when awk/grep pipelines
# closed early via `head -1` on large CSVs. We tolerate partial pipeline
# results — the benchmark CSV append (above) is the thing we must not miss.
set -eu

# Run Cyrius benchmarks and produce two outputs:
#   1) CSV history (appended each run)   — tracking regressions over time
#   2) Markdown table (overwritten)      — last 3 runs per benchmark for trends
#
# Usage:
#   ./scripts/bench-history.sh                          # defaults
#   ./scripts/bench-history.sh results.csv results.md   # custom paths

HISTORY_FILE="${1:-bench-history.csv}"
MD_FILE="${2:-bench-latest.md}"
TIMESTAMP=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
COMMIT=$(git rev-parse --short HEAD 2>/dev/null || echo "unknown")
# A run on uncommitted changes measures code HEAD does not hold; through 2.4.8
# such rows carried HEAD's hash bare, crediting the new numbers to the old tree.
# Only what is measured counts — this script's own outputs (the tracked CSV and
# markdown) are dirty after every run and would mark every later run of a clean
# HEAD as dirty too.
if [ "$COMMIT" != "unknown" ] \
    && ! git diff --quiet HEAD -- src benches cyrius.cyml cyrius.lock 2>/dev/null; then
    COMMIT="${COMMIT}-dirty"
fi
BRANCH=$(git branch --show-current 2>/dev/null || echo "unknown")
CYRIUS="${CYRIUS_HOME:-$HOME/.cyrius}/bin/cyrius"

# Create CSV header if file doesn't exist
if [ ! -f "$HISTORY_FILE" ]; then
    echo "timestamp,commit,branch,benchmark,avg_ns,min_ns,max_ns" > "$HISTORY_FILE"
fi

echo "Running benchmarks..."
echo "  commit:  $COMMIT"
echo "  branch:  $BRANCH"
echo "  compiler: $CYRIUS"
echo ""

# Run every benchmark file. Each runs on its own so a file that fails to
# build or crashes is named, with its output, instead of `set -e` ending the
# script in silence (through 2.4.8 a bench compile error exited 1 with no
# message).
BENCH_OUTPUT=""
for benchfile in benches/*.bcyr; do
    [ -f "$benchfile" ] || continue
    rc=0
    out=$("$CYRIUS" bench "$benchfile" 2>&1) || rc=$?
    if [ $rc -ne 0 ] || echo "$out" | grep -q 'FAIL: compile error'; then
        echo "$out" >&2
        echo "error: $benchfile failed (exit $rc) — nothing appended" >&2
        exit 1
    fi
    BENCH_OUTPUT="${BENCH_OUTPUT}
${out}"
done
echo "$BENCH_OUTPUT"
echo ""

# Row names must be unique across every benches/*.bcyr: the CSV and the
# markdown table key rows by name alone, so a second row of the same name is
# silently shadowed by the first. Through 2.4.8 `sqrt` existed in both bench.bcyr
# and bench_eval.bcyr and the evaluator's row never reached bench-latest.md.
# Checked before anything is appended, so a collision costs a re-run, not data.
DUPES=$(echo "$BENCH_OUTPUT" | grep -E 'avg.*min=.*max=' | sed -E 's/^[[:space:]]*//; s/:.*//' | sort | uniq -d)
if [ -n "$DUPES" ]; then
    echo "error: duplicate benchmark names (rename one of each in benches/*.bcyr):" >&2
    echo "$DUPES" | sed 's/^/  /' >&2
    exit 1
fi

# ── Helper: parse time string to nanoseconds ────────────────────────────────
to_ns() {
    local val="$1"
    # Input format: "17us", "534ns", "3us", "1.23ms", "0.456s"
    if [[ "$val" == *"ms"* ]]; then
        val="${val%ms}"
        awk "BEGIN {printf \"%.0f\", $val * 1000000}"
    elif [[ "$val" == *"us"* ]]; then
        val="${val%us}"
        awk "BEGIN {printf \"%.0f\", $val * 1000}"
    elif [[ "$val" == *"ns"* ]]; then
        val="${val%ns}"
        echo "$val"
    elif [[ "$val" == *"s"* ]]; then
        val="${val%s}"
        awk "BEGIN {printf \"%.0f\", $val * 1000000000}"
    else
        echo "$val"
    fi
}

# ── Helper: format ns to human-readable ─────────────────────────────────────
human_ns() {
    local ns="$1"
    awk "BEGIN {
        v = $ns
        if      (v < 1000)        printf \"%.0f ns\", v
        else if (v < 1000000)     printf \"%.2f us\", v / 1000
        else if (v < 1000000000)  printf \"%.2f ms\", v / 1000000
        else                      printf \"%.2f s\",  v / 1000000000
    }"
}

# ── Parse bench output and append to CSV ────────────────────────────────────
# Format: "  name: 17us avg (min=16us max=42us) [10000 iters]"
LINES_ADDED=0

while IFS= read -r line; do
    if [[ "$line" == *"avg"*"min="*"max="* ]]; then
        # Extract name (between leading spaces and colon)
        BENCH_NAME=$(echo "$line" | sed -E 's/^[[:space:]]*//' | sed -E 's/:.*//')
        # Extract avg value
        AVG_RAW=$(echo "$line" | sed -E 's/.*: ([^ ]+) avg.*/\1/')
        # Extract min value
        MIN_RAW=$(echo "$line" | sed -E 's/.*min=([^ ]+) .*/\1/')
        # Extract max value
        MAX_RAW=$(echo "$line" | sed -E 's/.*max=([^)]+)\).*/\1/')

        AVG_NS=$(to_ns "$AVG_RAW")
        MIN_NS=$(to_ns "$MIN_RAW")
        MAX_NS=$(to_ns "$MAX_RAW")

        echo "${TIMESTAMP},${COMMIT},${BRANCH},${BENCH_NAME},${AVG_NS},${MIN_NS},${MAX_NS}" >> "$HISTORY_FILE"
        LINES_ADDED=$((LINES_ADDED + 1))
    fi
done <<< "$BENCH_OUTPUT"

# ── Build 3-point tracking markdown from CSV history ────────────────────────
TIMESTAMPS=($(tail -n +2 "$HISTORY_FILE" | awk -F, '{print $1}' | sort -u | tail -3))
NUM_TS=${#TIMESTAMPS[@]}

declare -a COL_LABELS=()
for i in $(seq 0 $((NUM_TS - 1))); do
    ts="${TIMESTAMPS[$i]}"
    col_commit=$(grep "^${ts}," "$HISTORY_FILE" | head -1 | awk -F, '{print $2}')
    short_date=$(echo "$ts" | sed 's/T.*//; s/^20//')
    COL_LABELS+=("${short_date} (${col_commit})")
done

LATEST_TS="${TIMESTAMPS[$((NUM_TS - 1))]}"
BENCH_NAMES=($(grep "^${LATEST_TS}," "$HISTORY_FILE" | awk -F, '{print $4}'))

{
    echo "# Benchmark Results — Last ${NUM_TS} Runs"
    echo ""

    HEADER="| Benchmark |"
    SEP="|-----------|"
    for label in "${COL_LABELS[@]}"; do
        HEADER+=" ${label} |"
        SEP+="------|"
    done
    echo "$HEADER"
    echo "$SEP"

    for bench in "${BENCH_NAMES[@]}"; do
        ROW="| ${bench} |"
        for ts in "${TIMESTAMPS[@]}"; do
            est_ns=$(grep "^${ts},.*,${bench}," "$HISTORY_FILE" | awk -F, '{print $5}' | head -1)
            if [ -n "$est_ns" ]; then
                ROW+=" $(human_ns "$est_ns") |"
            else
                ROW+=" — |"
            fi
        done
        echo "$ROW"
    done

    echo ""
    echo "_Generated by \`scripts/bench-history.sh\` — Cyrius port benchmarks_"
} > "$MD_FILE"

echo ""
echo "Appended ${LINES_ADDED} benchmark entries to ${HISTORY_FILE}"
echo "Latest results written to ${MD_FILE} (${NUM_TS}-point tracking)"
