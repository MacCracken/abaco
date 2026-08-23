#!/usr/bin/env bash
# Version bump script — keeps every abaco *project-version* reference in sync.
#
# VERSION is the single source of truth. `cyrius.cyml [package].version` derives
# from it via the `${file:VERSION}` template, so this script does NOT edit the
# version there (release-verify treats the template as == VERSION). This script
# only bumps the project version and the docs that inline the number.
#
# NOTE: the Cyrius *toolchain* pin (`cyrius.cyml [package].cyrius = "X.Y.Z"`) is
# a separate concern and is bumped by hand — this script never touches it.
#
# Usage: ./scripts/version-bump.sh 2.3.4

set -euo pipefail

if [ $# -ne 1 ]; then
    echo "Usage: $0 <new-version>"
    echo "Current: $(tr -d '[:space:]' < VERSION)"
    exit 1
fi

NEW="$1"

# Plain semver X.Y.Z only — release.yml's tag filter is plain semver.
if ! printf '%s' "$NEW" | grep -qE '^[0-9]+\.[0-9]+\.[0-9]+$'; then
    echo "error: '$NEW' is not a plain semver X.Y.Z" >&2
    exit 1
fi

# Run from the repo root regardless of where the script is invoked from.
cd "$(dirname "$0")/.."

OLD=$(tr -d '[:space:]' < VERSION)

if [ "$NEW" = "$OLD" ]; then
    echo "Already at $OLD — nothing to do."
    exit 0
fi

CHANGED=""

# 1. VERSION (source of truth). Written with no trailing newline to match the
#    committed convention (VERSION is exactly the semver, N bytes, no LF).
printf '%s' "$NEW" > VERSION
CHANGED="$CHANGED VERSION"

# 2. cyrius.cyml [package].version — normally the `${file:VERSION}` template,
#    which needs no edit. Only rewrite if a *literal* old version is pinned.
if grep -qE "^version = \"$OLD\"" cyrius.cyml; then
    sed -i "s/^version = \"$OLD\"/version = \"$NEW\"/" cyrius.cyml
    CHANGED="$CHANGED cyrius.cyml"
    echo "  cyrius.cyml: literal version $OLD -> $NEW"
else
    echo "  cyrius.cyml: version is \${file:VERSION} template (derives from VERSION — no edit)"
fi

# 3. CHANGELOG.md — insert a stub section above the newest existing entry if the
#    version isn't documented yet. Fill in the sections by hand afterwards.
if ! grep -q "^## \[$NEW\]" CHANGELOG.md 2>/dev/null; then
    awk -v new="$NEW" -v date="$(date +%Y-%m-%d)" '
        BEGIN { inserted = 0 }
        /^## \[/ && !inserted {
            print "## [" new "] — " date
            print ""
            print "### Changed"
            print ""
            print "### Added"
            print ""
            print "### Fixed"
            print ""
            inserted = 1
        }
        { print }
    ' CHANGELOG.md > CHANGELOG.md.tmp && mv CHANGELOG.md.tmp CHANGELOG.md
    CHANGED="$CHANGED CHANGELOG.md"
    echo "  CHANGELOG.md: stub [$NEW] section inserted"
fi

# 4. Consumer-snippet `tag = "X.Y.Z"` — should point at the release just cut.
#    Self-healing: replaces whatever semver is there, not just $OLD.
#    Every file carrying such a tag, not just README.md — through
#    2.4.3 this healed README alone, so docs/guides/consuming-abaco.md
#    silently drifted a release behind and had to be fixed by hand at 2.4.3.
for TAGFILE in README.md docs/guides/consuming-abaco.md; do
    [ -f "$TAGFILE" ] || continue
    if grep -qE '^[[:space:]]*tag = "[0-9]+\.[0-9]+\.[0-9]+"' "$TAGFILE" 2>/dev/null; then
        OLD_TAG=$(grep -oE 'tag = "[0-9]+\.[0-9]+\.[0-9]+"' "$TAGFILE" | head -1 | grep -oE '[0-9]+\.[0-9]+\.[0-9]+')
        if [ "$OLD_TAG" != "$NEW" ]; then
            sed -i -E "s/^([[:space:]]*tag = \")[0-9]+\.[0-9]+\.[0-9]+(\")/\1$NEW\2/" "$TAGFILE"
            CHANGED="$CHANGED $TAGFILE"
            echo "  $TAGFILE: consumer-snippet tag $OLD_TAG -> $NEW"
        fi
    fi
done

# 5. SECURITY.md — the supported-versions table is written at MINOR
#    granularity (rows read "2.4.x", "2.3.x"), so a minor bump needs a table
#    edit. Through 2.4.3 this only warned on a MAJOR bump, which is why the
#    table still said 2.3.x four releases after 2.4.0 shipped.
OLD_MINOR="${OLD%.*}"
NEW_MINOR="${NEW%.*}"

echo ""
echo "$OLD -> $NEW"
echo "Updated:$CHANGED"
echo ""
echo "Still manual:"
echo "  - Fill the CHANGELOG.md [$NEW] Changed/Added/Fixed sections"
echo "  - docs/development/{state,roadmap}.md — bump version + add a release note"
echo "  - Regenerate the bundle if source changed:"
echo "      cyrius distlib abaco && mv dist/abaco-abaco.cyr dist/abaco.cyr"
if [ "$OLD_MINOR" != "$NEW_MINOR" ]; then
    echo "  - SECURITY.md supported-versions table (minor bump $OLD_MINOR -> $NEW_MINOR)"
fi
echo "  - Commit, tag, push (user does this): git tag $NEW && git push --tags"
