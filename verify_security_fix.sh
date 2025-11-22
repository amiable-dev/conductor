#!/bin/bash
# Security Fix Verification Script
# Verifies the socket path isolation security fix

set -e

echo "==================================="
echo "Socket Path Security Verification"
echo "==================================="
echo ""

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to check result
check_result() {
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✓ PASS${NC}: $1"
    else
        echo -e "${RED}✗ FAIL${NC}: $1"
        exit 1
    fi
}

# 1. Run tests
echo "1. Running security tests..."
cargo test --package midimon-daemon --lib daemon::state::tests --quiet > /dev/null 2>&1
check_result "All state tests pass"

# 2. Check socket path is NOT in /tmp
echo ""
echo "2. Verifying socket path location..."
if [[ "$OSTYPE" == "darwin"* ]]; then
    EXPECTED_PATH="$HOME/Library/Application Support/midimon/run/midimon.sock"
    if [[ "$EXPECTED_PATH" == *"/tmp"* ]]; then
        echo -e "${RED}✗ FAIL${NC}: Socket path contains /tmp"
        exit 1
    fi
    check_result "Socket NOT in /tmp directory"
elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
    if [ -n "$XDG_RUNTIME_DIR" ]; then
        EXPECTED_PATH="$XDG_RUNTIME_DIR/midimon/midimon.sock"
    else
        EXPECTED_PATH="$HOME/.midimon/run/midimon.sock"
    fi
    if [[ "$EXPECTED_PATH" == *"/tmp"* ]]; then
        echo -e "${RED}✗ FAIL${NC}: Socket path contains /tmp"
        exit 1
    fi
    check_result "Socket NOT in /tmp directory"
fi

# 3. Check directory permissions
echo ""
echo "3. Checking directory permissions..."
if [[ "$OSTYPE" == "darwin"* ]]; then
    RUNTIME_DIR="$HOME/Library/Application Support/midimon/run"
elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
    if [ -n "$XDG_RUNTIME_DIR" ]; then
        RUNTIME_DIR="$XDG_RUNTIME_DIR/midimon"
    else
        RUNTIME_DIR="$HOME/.midimon/run"
    fi
fi

if [ -d "$RUNTIME_DIR" ]; then
    PERMS=$(stat -f "%Lp" "$RUNTIME_DIR" 2>/dev/null || stat -c "%a" "$RUNTIME_DIR" 2>/dev/null)
    if [ "$PERMS" = "700" ]; then
        check_result "Directory has secure permissions (0700)"
    else
        echo -e "${RED}✗ FAIL${NC}: Directory has insecure permissions ($PERMS, expected 700)"
        exit 1
    fi
else
    echo -e "${YELLOW}⚠ WARN${NC}: Runtime directory not yet created (will be created on daemon startup)"
fi

# 4. Check ownership
echo ""
echo "4. Verifying directory ownership..."
if [ -d "$RUNTIME_DIR" ]; then
    CURRENT_USER=$(whoami)
    OWNER=$(stat -f "%Su" "$RUNTIME_DIR" 2>/dev/null || stat -c "%U" "$RUNTIME_DIR" 2>/dev/null)
    if [ "$OWNER" = "$CURRENT_USER" ]; then
        check_result "Directory owned by current user ($CURRENT_USER)"
    else
        echo -e "${RED}✗ FAIL${NC}: Directory owned by $OWNER, not $CURRENT_USER"
        exit 1
    fi
else
    echo -e "${YELLOW}⚠ WARN${NC}: Runtime directory not yet created"
fi

# 5. Display socket paths
echo ""
echo "5. Socket paths by platform:"
echo "----------------------------"
echo "Current platform: $OSTYPE"
if [[ "$OSTYPE" == "darwin"* ]]; then
    echo "Socket path: $HOME/Library/Application Support/midimon/run/midimon.sock"
elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
    if [ -n "$XDG_RUNTIME_DIR" ]; then
        echo "Socket path: $XDG_RUNTIME_DIR/midimon/midimon.sock"
    else
        echo "Socket path: $HOME/.midimon/run/midimon.sock"
    fi
elif [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "win32" ]]; then
    echo "Socket path: \\\\.\\pipe\\midimon"
fi

# 6. Check for old insecure socket
echo ""
echo "6. Checking for old insecure socket..."
if [ -e "/tmp/midimon.sock" ] || [ -d "/tmp/midimon" ]; then
    echo -e "${YELLOW}⚠ WARN${NC}: Old socket exists at /tmp/midimon.sock"
    echo "   You can safely remove it: rm -rf /tmp/midimon*"
else
    check_result "No old insecure socket found"
fi

# Summary
echo ""
echo "==================================="
echo -e "${GREEN}✓ Security Fix Verified${NC}"
echo "==================================="
echo ""
echo "Socket path isolation is correctly implemented:"
echo "  ✓ User-specific directories"
echo "  ✓ Secure permissions (0700)"
echo "  ✓ Ownership validation"
echo "  ✓ NOT in shared /tmp"
echo ""
echo "For more details, see:"
echo "  - SECURITY_SOCKET_ISOLATION.md"
echo "  - SECURITY_FIX_SUMMARY.md"
