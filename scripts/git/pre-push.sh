#!/bin/sh

set -eu

# A Rust hook script to verify what is about to be pushed. Called by "git
# push" after it has checked the remote status, but before anything has been
# pushed. If this script exits with a non-zero status nothing will be pushed.
#
# This hook is called with the following parameters:
#
# $1 -- Name of the remote to which the push is being done
# $2 -- URL to which the push is being done
#
# If pushing without using a named remote those arguments will be equal.
#
# Information about the commits which are being pushed is supplied as lines to
# the standard input in the form:
#
#   <local ref> <local sha1> <remote ref> <remote sha1>
#
# This file is intended to closely follow a format/lint/test CI/CD step.

REMOTE="${1:-}"
URL="${2:-}"

# ANSI color codes for styling
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
MAGENTA='\033[0;35m'
NC='\033[0m' # No Color

info() {
  echo "${MAGENTA}==> $1${NC}"
}

warn() {
  echo "${YELLOW}🚨 $1${NC}"
}

success() {
  echo "${GREEN}🎉 $1${NC}"
}

final_success() {
  echo "${GREEN}✅ $1${NC}"
  exit 0
}

fail() {
  echo "${RED}💥 $1${NC}"
  exit 1
}

if [ "${SKIP_UNCOMMITTED_CHECK:-false}" != "true" ] && [ -n "$(git status --porcelain)" ]; then
  fail "You have uncommitted changes. Please commit or stash them before proceeding."
fi

export RUSTFLAGS="-Dwarnings"
SCOPE="--all-targets --all-features"

CHECK_CMD="cargo check $SCOPE"
info "Checking code compilation with \`$CHECK_CMD\`..."
if ! $CHECK_CMD; then
  fail "Code did not compile. Run \`$CHECK_CMD\` and fix issues."
fi
success "Code compiles."

FMT_CMD="cargo fmt -v --check"
info "Checking formatting with \`$FMT_CMD\`..."
if ! $FMT_CMD; then
  fail "Code is not properly formatted. Run \`$FMT_CMD\` and fix issues."
fi
success "Code is properly formatted."

CLIPPY_CMD="cargo clippy $SCOPE"
info "Running clippy linter with \`$CLIPPY_CMD\`..."
if ! $CLIPPY_CMD; then
  fail "Clippy found issues. Run \`$CLIPPY_CMD\` and fix issues."
fi
success "Clippy linter passed."

TEST_CMD="cargo test $SCOPE"
info "Running tests with \`$TEST_CMD\`..."
if ! $TEST_CMD; then
  fail "Tests failed. Run \`$TEST_CMD\` and fix issues."
fi
success "All tests passed."

echo ""
final_success "Pre-push checks passed. Proceeding with push."
