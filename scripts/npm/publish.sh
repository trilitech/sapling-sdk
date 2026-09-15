#!/bin/bash
set -euo pipefail

# Publishes via npm Trusted Publishing (OIDC) - no NPM_AUTH_TOKEN needed.
# Requires the workflow to grant `permissions: id-token: write` and a
# Trusted Publisher configured on npmjs.com for this package/repo/workflow.
# Uses plain `npm publish` (not `lerna publish`) since OIDC trusted
# publishing is implemented in the npm CLI itself.

cd "$(dirname "$0")/../../packages/sapling-wasm"

PACKAGE_NAME=$(node -pe "require('./package.json').name")
VERSION=$(node -pe "require('./package.json').version")

if npm view "$PACKAGE_NAME@$VERSION" version >/dev/null 2>&1; then
  echo "$PACKAGE_NAME@$VERSION is already published, nothing to do."
  exit 0
fi

if [[ "$VERSION" == *beta* ]]
then
  echo "version is beta, using --tag next"
  npm publish --tag next
else
  npm publish
fi
