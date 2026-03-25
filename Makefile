# ===========================================
# UniSpec Makefile - Cross-Platform Build
# ===========================================

# --- Configuration ---
NAME := unispec
VERSION := $(shell grep '^version = ' Cargo.toml | sed 's/.*"\(.*\)".*/\1/')

# Installation paths
PREFIX ?= /usr/local
BIN_DIR ?= $(PREFIX)/bin
SHARE_DIR ?= $(PREFIX)/share/$(NAME)

# Config directory (cross-platform)
CONFIG_DIR := $(HOME)/.config/$(NAME)

# Default modes/areas source
AGENT_DIR := .agent

# --- Tool Settings ---
CARGO := cargo

# --- Cross-Platform Helpers ---
RM := rm -rf
CP := cp -r
INSTALL_BIN := install -Dm755
INSTALL_DATA := install -Dm644
MKDIR := mkdir -p

# Detect OS
ifeq ($(OS),Windows_NT)
    DETECTED_OS := windows
    RM := rd /s /q 2>nul
    CP := xcopy /E /I /Y
    MKDIR := mkdir
endif

# --- Targets ---
.PHONY: all build release test clean check clippy fmt install install-system \
        uninstall package run help setup-config init

all: build

# Build targets
build:
	$(CARGO) build

release:
	$(CARGO) build --release --locked

# Test targets
test:
	$(CARGO) test

test-all: test

# Code quality
check:
	$(CARGO) check

clippy:
	$(CARGO) clippy -- -D warnings

fmt:
	$(CARGO) fmt

# Clean
clean:
	$(RM) target
	$(RM) $(NAME)-$(VERSION)-* 2>nul || true

distclean: clean
	$(RM) Cargo.lock

# --- Installation ---
install: release install-bin install-config
	@echo "Installed $(NAME) $(VERSION) to ~/.cargo/bin/"
	@echo "Run '$(NAME)' to start!"

install-bin: release
	@$(MKDIR) $(HOME)/.cargo/bin 2>/dev/null || true
	$(INSTALL_BIN) target/release/$(NAME) $(HOME)/.cargo/bin/$(NAME)

# Initialize user config with defaults from .agent/
install-config:
	@echo "Setting up user config in $(CONFIG_DIR)..."
	@$(MKDIR) $(CONFIG_DIR)/modes 2>/dev/null || true
	@$(MKDIR) $(CONFIG_DIR)/areas 2>/dev/null || true
	@$(MKDIR) $(CONFIG_DIR)/templates 2>/dev/null || true
	@if [ ! -d "$(CONFIG_DIR)/modes/simple" ]; then \
		echo "  Installing default 'simple' mode..."; \
		$(CP) $(AGENT_DIR)/modes/simple $(CONFIG_DIR)/modes/; \
	fi
	@if [ ! -d "$(CONFIG_DIR)/areas/staging" ]; then \
		echo "  Installing default area templates..."; \
		$(CP) $(AGENT_DIR)/areas/* $(CONFIG_DIR)/areas/; \
	fi
	@if [ ! -d "$(CONFIG_DIR)/templates/specs.md" ]; then \
		$(CP) $(AGENT_DIR)/modes/simple/templates/specs.md $(CONFIG_DIR)/templates/ 2>/dev/null || true; \
		$(CP) $(AGENT_DIR)/modes/simple/templates/tasks.md $(CONFIG_DIR)/templates/ 2>/dev/null || true; \
	fi
	@echo "Done!"

# Install system-wide (requires sudo on Unix)
install-system: release
	@echo "Installing $(NAME) $(VERSION) system-wide..."
	$(MKDIR) $(DESTDIR)$(BIN_DIR)
	$(INSTALL_BIN) target/release/$(NAME) $(DESTDIR)$(BIN_DIR)/$(NAME)
	$(MKDIR) $(DESTDIR)$(SHARE_DIR)
	$(CP) $(AGENT_DIR)/* $(DESTDIR)$(SHARE_DIR)/
	@if [ -d "docs" ]; then \
		$(CP) -r docs $(DESTDIR)$(SHARE_DIR)/; \
	fi
	@echo "Installed to $(DESTDIR)$(BIN_DIR)/$(NAME)"
	@echo "Installed data to $(DESTDIR)$(SHARE_DIR)/"
	@echo ""
	@echo "Now setup user config:"
	@echo "  make setup-config"
	@echo ""

# Uninstall
uninstall:
	$(RM) $(HOME)/.cargo/bin/$(NAME) 2>/dev/null || true
	$(RM) $(DESTDIR)$(PREFIX)/bin/$(NAME) 2>/dev/null || true
	$(RM) $(DESTDIR)$(PREFIX)/share/$(NAME) 2>/dev/null || true

# Setup user config (convenience target)
setup-config: install-config

# Alias for first-time setup
init: install

# --- Packaging ---
package: package-linux

package-linux: release
	@echo "Creating Linux package..."
	$(MKDIR) $(NAME)-$(VERSION)-linux-x86_64
	$(CP) target/release/$(NAME) $(NAME)-$(VERSION)-linux-x86_64/
	$(CP) $(AGENT_DIR)/modes $(NAME)-$(VERSION)-linux-x86_64/
	$(CP) $(AGENT_DIR)/areas $(NAME)-$(VERSION)-linux-x86_64/
	$(CP) README.md LICENSE $(NAME)-$(VERSION)-linux-x86_64/ 2>/dev/null || true
	cd $(NAME)-$(VERSION)-linux-x86_64 && tar -czvf ../$(NAME)-$(VERSION)-linux-x86_64.tar.gz .
	$(RM) $(NAME)-$(VERSION)-linux-x86_64
	@echo "Created $(NAME)-$(VERSION)-linux-x86_64.tar.gz"

# --- Development ---
run:
	$(CARGO) run

dev: check test run

shell:
	@echo "Config dir: $(CONFIG_DIR)"
	@echo "Agent dir: $(AGENT_DIR)"
	@$(SHELL)

# --- Help ---
help:
	@echo "UniSpec Build System"
	@echo "===================="
	@echo ""
	@echo "Build Targets:"
	@echo "  make build         Build debug version"
	@echo "  make release       Build release version (recommended)"
	@echo "  make test         Run tests"
	@echo "  make check        Run linter"
	@echo "  make clippy       Run clippy lints"
	@echo "  make fmt          Format code"
	@echo "  make clean        Clean build artifacts"
	@echo ""
	@echo "Installation Targets:"
	@echo "  make install      Install to ~/.cargo/bin/ (recommended for users)"
	@echo "  make install-system  Install system-wide (requires sudo on Unix)"
	@echo "  make setup-config  Setup user config with default modes/areas"
	@echo "  make init         Shortcut for install (build + install + config)"
	@echo ""
	@echo "Packaging Targets:"
	@echo "  make package      Create Linux distribution package"
	@echo ""
	@echo "Development Targets:"
	@echo "  make run         Build and run"
	@echo "  make dev          Check, test, and run"
	@echo ""
	@echo "Configuration:"
	@echo "  PREFIX=$(PREFIX)"
	@echo "  CONFIG_DIR=$(CONFIG_DIR)"
	@echo "  VERSION=$(VERSION)"
	@echo ""
	@echo "Quick Start:"
	@echo "  make release       # Build release version"
	@echo "  make install      # Install to ~/.cargo/bin/"
	@echo "  make init         # Install + setup config"
