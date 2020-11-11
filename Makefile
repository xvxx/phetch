# Simple, stupid makefile to make phetch

PREFIX ?= /usr/local
_INSTDIR = $(DESTDIR)$(PREFIX)
BINDIR ?= $(_INSTDIR)/bin
MANDIR ?= $(_INSTDIR)/share/man

PHETCH_RELEASE = target/release/phetch
PHETCH_DEBUG = target/debug/phetch

RSFILES = $(wildcard src/*.rs src/**/*.rs)

.PHONY: all release debug clean install manual scdoc

# Default target
all: release manual

# Release build for distribution
release: $(PHETCH_RELEASE)

# Binary with debugging info
debug: $(PHETCH_DEBUG)
	./target/debug/phetch

# Remove the release directory and its contents
clean:
	@rm -rf target

# Run tests
test:
	cargo clippy --all-features
	cargo test --all-features

# Build the release version
$(PHETCH_RELEASE): $(RSFILES)
	cargo build --release

# Build the debug version
$(PHETCH_DEBUG): $(RSFILES)
	cargo build --no-default-features

# Install phetch and its manual.
install: all
	mkdir -p $(BINDIR) $(MANDIR)/man1
	install -m755 $(PHETCH_RELEASE) $(BINDIR)/phetch
	install -m644 doc/phetch.1 $(MANDIR)/man1/phetch.1

# Undo
uninstall:
	rm -f $(BINDIR)/phetch $(MANDIR)/man1/phetch.1

# Build manual
manual: doc/phetch.1

doc/phetch.1: doc/phetch.1.md scdoc
	scdoc < doc/phetch.1.md > doc/phetch.1

# Must have scdoc installed to build manual.
scdoc:
	@which scdoc || (echo "scdoc(1) not found."; \
		echo "install it: https://repology.org/project/scdoc"; exit 1)
