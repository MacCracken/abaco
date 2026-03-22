#!/usr/bin/env bash
set -euo pipefail

# Usage: ./scripts/version-bump.sh 0.2.0

if [ $# -ne 1 ]; then
    echo "Usage: $0 <new-version>"
    exit 1
fi

NEW_VERSION="$1"
OLD_VERSION=$(cat VERSION)

echo "Bumping version: $OLD_VERSION -> $NEW_VERSION"

# Update VERSION file
echo "$NEW_VERSION" > VERSION

# Update Cargo.toml
sed -i "s/^version = \"$OLD_VERSION\"/version = \"$NEW_VERSION\"/" Cargo.toml

echo "Done. Updated VERSION and Cargo.toml."
echo "Don't forget to update CHANGELOG.md!"
