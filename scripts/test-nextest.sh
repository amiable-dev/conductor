#!/usr/bin/env bash
# Run tests with cargo-nextest for improved output
# Usage: ./scripts/test-nextest.sh [nextest options]

set -euo pipefail

# Color output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if cargo-nextest is installed
if ! command -v cargo-nextest &> /dev/null; then
    echo -e "${RED}Error: cargo-nextest is not installed${NC}"
    echo "Install it with: cargo install cargo-nextest"
    exit 1
fi

echo -e "${GREEN}Running tests with nextest...${NC}"

# Run tests with nextest
cargo nextest run --all-features "$@"

TEST_RESULT=$?

if [ $TEST_RESULT -eq 0 ]; then
    echo ""
    echo -e "${GREEN}All tests passed!${NC}"
else
    echo ""
    echo -e "${RED}Some tests failed.${NC}"
    exit $TEST_RESULT
fi

# Also run doc tests (nextest doesn't support them yet)
echo ""
echo -e "${YELLOW}Running doc tests...${NC}"
cargo test --doc --all-features

DOC_TEST_RESULT=$?

if [ $DOC_TEST_RESULT -eq 0 ]; then
    echo -e "${GREEN}Doc tests passed!${NC}"
else
    echo -e "${RED}Doc tests failed.${NC}"
    exit $DOC_TEST_RESULT
fi
