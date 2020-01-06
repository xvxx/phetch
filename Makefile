# Simple, stupid makefile to make phetch

PHETCH_RELEASE = target/release/phetch
PHETCH_DEBUG = target/debug/phetch

RSFILES = $(wildcard src/*.rs src/**/*.rs)

.PHONY: release debug clean

# Default target
release: $(PHETCH_RELEASE)

# Binary with debugging info
debug: $(PHETCH_DEBUG)

# Remove the release directory and its contents
clean:
	@rm -rf target

# Build and strip the release version
$(PHETCH_RELEASE): $(RSFILES)
	cargo build --release
	strip $@

# Build the debug version
$(PHETCH_DEBUG): $(RSFILES)
	cargo build 
