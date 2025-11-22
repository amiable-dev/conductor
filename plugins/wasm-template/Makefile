# MIDIMon WASM Plugin Build System
#
# This Makefile provides commands for building, testing, and optimizing
# WASM plugins for MIDIMon.

.PHONY: help build build-release test clean install-deps optimize inspect size

# Default target
help:
	@echo "MIDIMon WASM Plugin Build System"
	@echo ""
	@echo "Available targets:"
	@echo "  make build           - Build debug .wasm"
	@echo "  make build-release   - Build optimized .wasm (production)"
	@echo "  make test            - Run plugin tests"
	@echo "  make optimize        - Run wasm-opt for size reduction"
	@echo "  make inspect         - Inspect WASM module exports"
	@echo "  make size            - Show binary size"
	@echo "  make clean           - Remove build artifacts"
	@echo "  make install-deps    - Install required tools (rustup, wasm-opt)"

# Build debug version
build:
	@echo "Building debug WASM plugin..."
	cargo build --target wasm32-wasip1
	@cp target/wasm32-wasip1/debug/*.wasm . 2>/dev/null || true
	@echo "✓ Debug build complete: $(shell ls *.wasm 2>/dev/null | head -1)"

# Build release version (optimized)
build-release:
	@echo "Building release WASM plugin..."
	cargo build --release --target wasm32-wasip1
	@cp target/wasm32-wasip1/release/*.wasm . 2>/dev/null || true
	@echo "✓ Release build complete: $(shell ls *.wasm 2>/dev/null | head -1)"
	@echo "Size: $(shell ls -lh *.wasm 2>/dev/null | awk '{print $$5}')"

# Run tests (native, not WASM)
test:
	@echo "Running plugin tests..."
	cargo test

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	cargo clean
	rm -f *.wasm
	@echo "✓ Clean complete"

# Install required dependencies
install-deps:
	@echo "Installing WASM target..."
	rustup target add wasm32-wasip1
	@echo ""
	@echo "Installing wasm-opt (Binaryen)..."
	@if command -v brew >/dev/null 2>&1; then \
		brew install binaryen; \
	elif command -v apt-get >/dev/null 2>&1; then \
		sudo apt-get install -y binaryen; \
	else \
		echo "⚠ Please install binaryen manually for wasm-opt"; \
		echo "  macOS: brew install binaryen"; \
		echo "  Linux: apt-get install binaryen"; \
	fi
	@echo "✓ Dependencies installed"

# Optimize WASM binary with wasm-opt
optimize: build-release
	@echo "Optimizing WASM binary..."
	@if command -v wasm-opt >/dev/null 2>&1; then \
		WASM_FILE=$$(ls *.wasm 2>/dev/null | head -1); \
		if [ -n "$$WASM_FILE" ]; then \
			SIZE_BEFORE=$$(wc -c < "$$WASM_FILE"); \
			wasm-opt -Oz "$$WASM_FILE" -o "$${WASM_FILE%.wasm}.opt.wasm"; \
			SIZE_AFTER=$$(wc -c < "$${WASM_FILE%.wasm}.opt.wasm"); \
			REDUCTION=$$(echo "scale=1; ($$SIZE_BEFORE - $$SIZE_AFTER) * 100 / $$SIZE_BEFORE" | bc); \
			echo "✓ Optimization complete"; \
			echo "  Before: $$(numfmt --to=iec-i --suffix=B $$SIZE_BEFORE)"; \
			echo "  After:  $$(numfmt --to=iec-i --suffix=B $$SIZE_AFTER)"; \
			echo "  Reduction: $${REDUCTION}%"; \
			mv "$${WASM_FILE%.wasm}.opt.wasm" "$$WASM_FILE"; \
		else \
			echo "⚠ No .wasm file found"; \
		fi \
	else \
		echo "⚠ wasm-opt not found. Run 'make install-deps' first."; \
		exit 1; \
	fi

# Inspect WASM module exports
inspect:
	@echo "Inspecting WASM module..."
	@WASM_FILE=$$(ls *.wasm 2>/dev/null | head -1); \
	if [ -n "$$WASM_FILE" ]; then \
		wasm-objdump -x "$$WASM_FILE" | grep -A 20 "Export\["; \
	else \
		echo "⚠ No .wasm file found. Run 'make build' first."; \
	fi

# Show binary size
size:
	@echo "WASM binary sizes:"
	@ls -lh *.wasm 2>/dev/null | awk '{print "  " $$9 ": " $$5}' || echo "  No .wasm files found"
