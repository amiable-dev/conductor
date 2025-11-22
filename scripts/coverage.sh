#!/usr/bin/env bash
# Generate code coverage report for MIDIMon
# Usage: ./scripts/coverage.sh [--html|--open|--lcov]

set -euo pipefail

# Color output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if cargo-llvm-cov is installed
if ! command -v cargo-llvm-cov &> /dev/null; then
    echo -e "${RED}Error: cargo-llvm-cov is not installed${NC}"
    echo "Install it with: cargo install cargo-llvm-cov"
    exit 1
fi

# Parse arguments
HTML_MODE=false
OPEN_MODE=false
LCOV_MODE=false

for arg in "$@"; do
    case $arg in
        --html)
            HTML_MODE=true
            ;;
        --open)
            OPEN_MODE=true
            HTML_MODE=true
            ;;
        --lcov)
            LCOV_MODE=true
            ;;
        --help)
            echo "Usage: $0 [--html|--open|--lcov]"
            echo ""
            echo "Options:"
            echo "  --html    Generate HTML coverage report"
            echo "  --open    Generate HTML report and open in browser"
            echo "  --lcov    Generate lcov.info file for CI/Codecov"
            echo ""
            echo "Default: Display coverage summary in terminal"
            exit 0
            ;;
        *)
            echo -e "${RED}Unknown option: $arg${NC}"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

echo -e "${GREEN}Running tests with coverage...${NC}"

# Generate coverage based on mode
if [ "$LCOV_MODE" = true ]; then
    echo -e "${YELLOW}Generating lcov.info...${NC}"
    cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info
    echo -e "${GREEN}Coverage report saved to: lcov.info${NC}"
elif [ "$HTML_MODE" = true ]; then
    echo -e "${YELLOW}Generating HTML coverage report...${NC}"
    cargo llvm-cov --all-features --workspace --html
    echo -e "${GREEN}HTML coverage report saved to: target/llvm-cov/html/index.html${NC}"

    if [ "$OPEN_MODE" = true ]; then
        echo -e "${YELLOW}Opening coverage report in browser...${NC}"
        if command -v open &> /dev/null; then
            open target/llvm-cov/html/index.html
        elif command -v xdg-open &> /dev/null; then
            xdg-open target/llvm-cov/html/index.html
        else
            echo -e "${YELLOW}Could not open browser automatically${NC}"
            echo "Open manually: target/llvm-cov/html/index.html"
        fi
    fi
else
    # Default: show summary in terminal
    cargo llvm-cov --all-features --workspace
fi

echo ""
echo -e "${GREEN}Coverage generation complete!${NC}"
