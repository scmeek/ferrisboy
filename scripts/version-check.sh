#!/bin/sh

set -eu

SCRIPTS_DIR="${SCRIPTS_DIR:-$(dirname -- "$(readlink -f -- "$0")")}"
PROJECT_ROOT="${PROJECT_ROOT:-$(CDPATH='' cd -- "$SCRIPTS_DIR/.." && pwd)}"

. "${SCRIPTS_DIR}/functions.sh"

echo ""

info "Running semantic versioning check..."
LAST_DEVELOP_GIT_HASH=$(git rev-parse main)
SEMVER_CHECK_CMD="cargo semver-checks --all-features --baseline-rev $LAST_DEVELOP_GIT_HASH"
if ! $SEMVER_CHECK_CMD; then
  fail "Semantic versioning check failed.. Run \`$SEMVER_CHECK_CMD\` and fix issues."
fi
success "Semantic versioning check passed."
