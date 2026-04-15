#!/usr/bin/env bash
# Version bump script — single source of truth for all abaco version references.
# Usage: ./scripts/version-bump.sh 2.1.0

set -euo pipefail

if [ $# -ne 1 ]; then
    echo "Usage: $0 <new-version>"
    echo "Current: $(cat VERSION)"
    exit 1
fi

NEW="$1"
OLD=$(cat VERSION | tr -d '[:space:]')

if [ "$NEW" = "$OLD" ]; then
    echo "Already at $OLD"
    exit 0
fi

# 1. VERSION (source of truth)
echo "$NEW" > VERSION

# 2. cyrius.toml [package] version
sed -i "s/^version = \"$OLD\"/version = \"$NEW\"/" cyrius.toml

# 3. README.md — status line + consumer-snippet `tag = "X.Y.Z"`
sed -i "s/\*\*v$OLD\*\*/\*\*v$NEW\*\*/g" README.md 2>/dev/null || true
sed -i "s/tag  = \"$OLD\"/tag  = \"$NEW\"/" README.md 2>/dev/null || true

# 4. CHANGELOG.md — insert new version header below the "Keep a Changelog"
#    header paragraph if the version isn't already documented.
if ! grep -q "^## \[$NEW\]" CHANGELOG.md 2>/dev/null; then
    awk -v new="$NEW" -v date="$(date +%Y-%m-%d)" '
        BEGIN { inserted = 0 }
        /^## \[/ && !inserted {
            print "## [" new "] — " date
            print ""
            print "### Added"
            print ""
            print "### Changed"
            print ""
            print "### Fixed"
            print ""
            inserted = 1
        }
        { print }
    ' CHANGELOG.md > CHANGELOG.md.tmp && mv CHANGELOG.md.tmp CHANGELOG.md
fi

# 5. SECURITY.md — supported-versions table (major track only)
OLD_MAJOR="${OLD%%.*}"
NEW_MAJOR="${NEW%%.*}"
if [ "$OLD_MAJOR" != "$NEW_MAJOR" ]; then
    echo "  note: major version change ($OLD_MAJOR -> $NEW_MAJOR)"
    echo "  review SECURITY.md supported-versions table manually"
fi

echo "$OLD -> $NEW"
echo ""
echo "Updated:"
echo "  VERSION"
echo "  cyrius.toml"
echo "  README.md (status + consumer tag)"
echo "  CHANGELOG.md (stub section for $NEW)"
echo ""
echo "Still manual:"
echo "  - Fill the CHANGELOG.md Added/Changed/Fixed sections for $NEW"
if [ "$OLD_MAJOR" != "$NEW_MAJOR" ]; then
    echo "  - SECURITY.md supported-versions table (major bump detected)"
fi
echo "  - Commit, tag, push: git tag $NEW && git push --tags"
