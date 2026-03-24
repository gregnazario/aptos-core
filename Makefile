# Makefile — Aptos Core developer build utilities
#
# Run `make` or `make help` to see all available targets.

SHELL := /bin/bash
PROJECT_ROOT := $(shell pwd)
SCCACHE_ENV := $(PROJECT_ROOT)/.sccache.env
FRAMEWORK_DIR := $(PROJECT_ROOT)/aptos-move/framework
FRAMEWORK_STAMP := $(PROJECT_ROOT)/.framework-built

# Move source directories to watch for framework changes
MOVE_SOURCES := $(shell find $(FRAMEWORK_DIR) -name '*.move' -not -path '*/build/*' 2>/dev/null)

# ═════════════════════════════════════════════════════════════════════════════
# Help
# ═════════════════════════════════════════════════════════════════════════════

.DEFAULT_GOAL := help

.PHONY: help
help:
	@echo "Aptos Core — Developer Build Utilities"
	@echo ""
	@echo "Getting started:"
	@echo "  make setup              Guided setup checklist for new developers"
	@echo ""
	@echo "Building:"
	@echo "  make check              Quick workspace compile check (no codegen)"
	@echo "  make build              Build aptos-node (debug)"
	@echo "  make build-release      Build aptos-node (release, optimized)"
	@echo "  make build-cli          Build aptos CLI (debug)"
	@echo "  make build-cli-release  Build aptos CLI (release, optimized)"
	@echo "  make framework          Rebuild Move cached packages (auto-detects changes)"
	@echo "  make framework-force    Rebuild Move cached packages unconditionally"
	@echo ""
	@echo "Testing:"
	@echo "  make test               Run targeted unit tests (changed packages only)"
	@echo "  make test-all           Run all unit tests (full workspace)"
	@echo "  make test-doc           Run doc tests"
	@echo "  make test-smoke         Run smoke tests (builds release aptos-node first)"
	@echo "  make test-package P=x   Run tests for a specific package"
	@echo ""
	@echo "Linting:"
	@echo "  make lint               Check lints without modifying (clippy + fmt + sort)"
	@echo "  make fmt                Auto-fix formatting and dependency sort order"
	@echo ""
	@echo "Validation (CI parity):"
	@echo "  make check-licenses           Check dependency licenses (cargo deny)"
	@echo "  make check-cached-packages    Verify Move cached packages are fresh"
	@echo "  make check-vm-features        Verify VM feature flags"
	@echo "  make check-crypto-symbols     CryptoHasher domain separation check"
	@echo "  make lint-shell               Shellcheck on dev_setup.sh"
	@echo "  make check-undeclared-features Find undeclared Cargo feature dependencies"
	@echo "  make show-changed             Show changed files and affected packages"
	@echo ""
	@echo "Pre-push (CI mirror):"
	@echo "  make pre-push           Run lint + validation + targeted tests"
	@echo ""
	@echo "Cleaning:"
	@echo "  make clean              Remove target/ directory"
	@echo "  make clean-all          Remove target/ + sccache local cache + cargo registry"
	@echo ""
	@echo "sccache (shared build cache):"
	@echo "  make sccache-setup      Install sccache + generate .sccache.env (S3 backend)"
	@echo "  make sccache-setup-rust Same, but also wrap rustc (disables incremental)"
	@echo "  make sccache-check      Verify sccache is configured and working"
	@echo "  make sccache-stats      Show sccache hit/miss statistics"
	@echo "  make sccache-clean      Stop sccache server and clear local cache"

# ═════════════════════════════════════════════════════════════════════════════
# Setup (new developer onboarding)
# ═════════════════════════════════════════════════════════════════════════════

.PHONY: setup
setup:
	@echo "═══ Aptos Core Developer Setup ═══"
	@echo ""
	@echo "Follow these steps to set up your development environment."
	@echo "Run each target individually as needed."
	@echo ""
	@echo "Step 1: Install system dependencies"
	@echo "  make setup-deps"
	@echo "  Installs: protoc, build tools, PostgreSQL, JS/TS tools"
	@echo ""
	@echo "Step 2: Verify Rust toolchain"
	@echo "  make setup-rust"
	@echo "  Verifies rust-toolchain.toml is active and shows versions"
	@echo ""
	@echo "Step 3: Set up sccache (shared build cache)"
	@echo "  make sccache-setup"
	@echo "  Then add 'source $(SCCACHE_ENV)' to your shell profile"
	@echo ""
	@echo "Step 4: Verify everything works"
	@echo "  make check"
	@echo "  Quick compile check of the full workspace"
	@echo ""
	@echo "Step 5 (optional): Run a full build"
	@echo "  make build"
	@echo "  Builds aptos-node in debug mode"
	@echo ""
	@echo "For Move Prover tools (z3, cvc5, boogie), also run:"
	@echo "  make setup-prover"

.PHONY: setup-deps
setup-deps:
	@echo "Installing system dependencies..."
	scripts/dev_setup.sh -b -r -P -J -t

.PHONY: setup-prover
setup-prover:
	@echo "Installing Move Prover tools (z3, cvc5, boogie)..."
	scripts/dev_setup.sh -b -y

.PHONY: setup-rust
setup-rust:
	@echo "Rust toolchain:"
	@rustup show active-toolchain
	@echo ""
	@rustc --version
	@cargo --version
	@echo ""
	@if [ -f rust-toolchain.toml ]; then \
		echo "rust-toolchain.toml:"; \
		grep channel rust-toolchain.toml; \
	fi

# ═════════════════════════════════════════════════════════════════════════════
# Building
# ═════════════════════════════════════════════════════════════════════════════

.PHONY: check
check: framework
	cargo check --workspace --locked

.PHONY: build
build: framework
	cargo build -p aptos-node --locked

.PHONY: build-release
build-release: framework
	cargo build -p aptos-node --release --locked

.PHONY: build-cli
build-cli: framework
	cargo build -p aptos --locked

.PHONY: build-cli-release
build-cli-release: framework
	cargo build -p aptos --release --locked

# ─── Framework (Move cached packages) ──────────────────────────────────────

.PHONY: framework
framework: $(FRAMEWORK_STAMP)

# Rebuild cached packages if any .move file is newer than the stamp
$(FRAMEWORK_STAMP): $(MOVE_SOURCES)
	@echo "Move files changed — rebuilding cached packages..."
	cargo build -p aptos-cached-packages --locked
	@touch $(FRAMEWORK_STAMP)

.PHONY: framework-force
framework-force:
	cargo build -p aptos-cached-packages --locked
	@touch $(FRAMEWORK_STAMP)

# ═════════════════════════════════════════════════════════════════════════════
# Testing
# ═════════════════════════════════════════════════════════════════════════════

.PHONY: test
test: framework
	cargo x targeted-unit-tests -vvv --profile ci --cargo-profile ci --locked --no-fail-fast --retries 3

.PHONY: test-all
test-all: framework
	cargo nextest run \
		--profile ci \
		--cargo-profile ci \
		--locked \
		--workspace \
		--exclude smoke-test \
		--exclude aptos-testcases \
		--exclude aptos-keyless-circuit \
		--exclude aptos-batch-encryption \
		--retries 3 \
		--no-fail-fast

.PHONY: test-doc
test-doc: framework
	cargo test --profile ci --locked --doc --workspace

.PHONY: test-smoke
test-smoke: framework
	cargo build --locked --package=aptos-node --features=failpoints,smoke-test --release
	LOCAL_SWARM_NODE_RELEASE=1 cargo nextest run --release --profile smoke-test --package smoke-test

.PHONY: test-package
test-package: framework
ifndef P
	$(error Usage: make test-package P=<package-name>)
endif
	cargo test -p $(P) --locked

# ═════════════════════════════════════════════════════════════════════════════
# Validation (CI parity checks)
# ═════════════════════════════════════════════════════════════════════════════

.PHONY: check-licenses
check-licenses:
	@command -v cargo-deny &>/dev/null || { echo "cargo-deny not found. Install: cargo install cargo-deny --locked"; exit 1; }
	cargo deny check licenses

.PHONY: check-cached-packages
check-cached-packages:
	scripts/cargo_build_aptos_cached_packages.sh --check

.PHONY: check-vm-features
check-vm-features:
	cargo test --profile ci --locked --features check-vm-features -p aptos-node

.PHONY: check-crypto-symbols
check-crypto-symbols:
	python3 scripts/check-cryptohasher-symbols.py

.PHONY: lint-shell
lint-shell:
	@command -v shellcheck &>/dev/null || { echo "shellcheck not found. Install: brew install shellcheck (or apt install shellcheck)"; exit 1; }
	shellcheck scripts/dev_setup.sh

.PHONY: check-undeclared-features
check-undeclared-features:
	scripts/find-packages-with-undeclared-feature-dependencies.sh

.PHONY: show-changed
show-changed:
	cargo x changed-files -vvv
	cargo x affected-packages -vvv

# ═════════════════════════════════════════════════════════════════════════════
# Linting
# ═════════════════════════════════════════════════════════════════════════════

.PHONY: lint
lint:
	scripts/rust_lint.sh --check

.PHONY: fmt
fmt:
	cargo +nightly fmt
	cargo sort --grouped --workspace

# ═════════════════════════════════════════════════════════════════════════════
# Pre-push (mirrors CI checks)
# ═════════════════════════════════════════════════════════════════════════════

.PHONY: pre-push
pre-push: lint check-licenses check-cached-packages check-crypto-symbols test
	@echo ""
	@echo "All pre-push checks passed."

# ═════════════════════════════════════════════════════════════════════════════
# Cleaning
# ═════════════════════════════════════════════════════════════════════════════

.PHONY: clean
clean:
	@echo "Removing target/ directory..."
	cargo clean
	@rm -f $(FRAMEWORK_STAMP)
	@echo "Done. Run 'make clean-all' to also clear caches."

.PHONY: clean-all
clean-all: clean sccache-clean
	@echo "Removing cargo registry cache..."
	@rm -rf "$${HOME}/.cargo/registry/cache/"
	@rm -rf "$${HOME}/.cargo/registry/src/"
	@echo "All caches cleared."

# ═════════════════════════════════════════════════════════════════════════════
# sccache (shared build cache)
# ═════════════════════════════════════════════════════════════════════════════
#
# After running `make sccache-setup`, add this to ~/.zshrc or ~/.bashrc:
#   source /path/to/aptos-core/.sccache.env
#
# If you have multiple checkouts, source each one's .sccache.env — the
# generated file automatically appends to SCCACHE_BASEDIRS so all checkouts
# share cache hits.

.PHONY: sccache-setup
sccache-setup: sccache-install $(SCCACHE_ENV)
	@echo ""
	@echo "Done! Add this to your shell profile (~/.zshrc or ~/.bashrc):"
	@echo ""
	@echo "  source $(SCCACHE_ENV)"
	@echo ""
	@echo "Then restart your shell or run: source $(SCCACHE_ENV)"

.PHONY: sccache-setup-rust
sccache-setup-rust: INCLUDE_RUST=1
sccache-setup-rust: sccache-install $(SCCACHE_ENV)
	@echo ""
	@echo "Done! Add this to your shell profile (~/.zshrc or ~/.bashrc):"
	@echo ""
	@echo "  source $(SCCACHE_ENV)"
	@echo ""
	@echo "Then restart your shell or run: source $(SCCACHE_ENV)"

.PHONY: sccache-install
sccache-install:
	@if command -v sccache &>/dev/null; then \
		echo "sccache already installed: $$(sccache --version)"; \
	else \
		echo "Installing sccache..."; \
		case "$$(uname -s)" in \
			Darwin) \
				if command -v brew &>/dev/null; then \
					brew install sccache; \
				else \
					echo "Homebrew not found. Install via: cargo install sccache --locked"; \
					exit 1; \
				fi;; \
			Linux) \
				cargo install sccache --locked;; \
			*) \
				echo "Unsupported platform: $$(uname -s)"; \
				exit 1;; \
		esac; \
		echo "Installed: $$(sccache --version)"; \
	fi

$(SCCACHE_ENV): FORCE
	@echo "Generating $(SCCACHE_ENV) ..."
	@{ \
		echo "# Auto-generated by: make sccache-setup"; \
		echo "# Checkout: $(PROJECT_ROOT)"; \
		echo "# Re-run 'make sccache-setup' to regenerate."; \
		echo ""; \
		echo "# ── S3 shared cache ──"; \
		echo "export SCCACHE_BUCKET=aptos-sccache-shared"; \
		echo "export SCCACHE_REGION=us-west-2"; \
		echo "export SCCACHE_S3_USE_SSL=true"; \
		echo ""; \
		echo "# ── Path normalization (enables sharing across checkouts) ──"; \
		echo "# Appends this checkout to SCCACHE_BASEDIRS so multiple checkouts"; \
		echo "# share the same cache. Each checkout's .sccache.env adds its own path."; \
		echo 'if [ -z "$${SCCACHE_BASEDIRS:-}" ]; then'; \
		echo "  export SCCACHE_BASEDIRS=$(PROJECT_ROOT)"; \
		echo "else"; \
		echo '  case ":$$SCCACHE_BASEDIRS:" in'; \
		echo "    *\":$(PROJECT_ROOT):\"*) ;;"; \
		echo "    *) export SCCACHE_BASEDIRS=\"\$$SCCACHE_BASEDIRS:$(PROJECT_ROOT)\" ;;"; \
		echo "  esac"; \
		echo "fi"; \
		echo ""; \
		echo "# ── C/C++ compiler wrapping (~95s savings on -sys crates) ──"; \
		echo 'export CC="sccache cc"'; \
		echo 'export CXX="sccache c++"'; \
	} > $(SCCACHE_ENV)
	@if [ "$(INCLUDE_RUST)" = "1" ]; then \
		{ \
			echo ""; \
			echo "# ── Rust compiler wrapping (disables incremental compilation) ──"; \
			echo "export RUSTC_WRAPPER=sccache"; \
		} >> $(SCCACHE_ENV); \
	fi
	@echo "Generated: $(SCCACHE_ENV)"

FORCE:

.PHONY: sccache-check
sccache-check:
	@echo "=== sccache setup check ==="
	@echo ""
	@if command -v sccache &>/dev/null; then \
		echo "[OK] sccache installed: $$(sccache --version)"; \
	else \
		echo "[FAIL] sccache not found. Run: make sccache-setup"; \
		exit 1; \
	fi
	@echo ""
	@if [ -f "$(SCCACHE_ENV)" ]; then \
		echo "[OK] $(SCCACHE_ENV) exists"; \
	else \
		echo "[FAIL] $(SCCACHE_ENV) not found. Run: make sccache-setup"; \
		exit 1; \
	fi
	@echo ""
	@if [ -n "$${SCCACHE_BUCKET:-}" ]; then \
		echo "[OK] SCCACHE_BUCKET=$$SCCACHE_BUCKET"; \
	else \
		echo "[WARN] SCCACHE_BUCKET not set — source $(SCCACHE_ENV) first"; \
	fi
	@if [ -n "$${SCCACHE_BASEDIRS:-}" ]; then \
		echo "[OK] SCCACHE_BASEDIRS=$$SCCACHE_BASEDIRS"; \
	else \
		echo "[WARN] SCCACHE_BASEDIRS not set — source $(SCCACHE_ENV) first"; \
	fi
	@if [ -n "$${CC:-}" ] && echo "$$CC" | grep -q sccache; then \
		echo "[OK] CC=$$CC"; \
	else \
		echo "[WARN] CC not wrapped with sccache"; \
	fi
	@if [ -n "$${CXX:-}" ] && echo "$$CXX" | grep -q sccache; then \
		echo "[OK] CXX=$$CXX"; \
	else \
		echo "[WARN] CXX not wrapped with sccache"; \
	fi
	@if [ -n "$${RUSTC_WRAPPER:-}" ]; then \
		echo "[OK] RUSTC_WRAPPER=$$RUSTC_WRAPPER"; \
	else \
		echo "[INFO] RUSTC_WRAPPER not set (Rust uses incremental compilation)"; \
	fi
	@echo ""
	@if [ -n "$${AWS_ACCESS_KEY_ID:-}" ]; then \
		echo "[OK] AWS_ACCESS_KEY_ID is set"; \
	else \
		echo "[WARN] AWS_ACCESS_KEY_ID not set — S3 sharing disabled (local cache only)"; \
	fi
	@echo ""
	@if sccache --show-stats &>/dev/null; then \
		echo "[OK] sccache server running"; \
		echo ""; \
		sccache --show-stats 2>&1 | head -5; \
	else \
		echo "[INFO] sccache server not running (starts automatically on first compile)"; \
	fi

.PHONY: sccache-stats
sccache-stats:
	@sccache --show-stats

.PHONY: sccache-clean
sccache-clean:
	@echo "Stopping sccache server..."
	-@sccache --stop-server 2>/dev/null
	@echo "Clearing local sccache cache..."
	@rm -rf "$${HOME}/Library/Caches/Mozilla.sccache/" "$${HOME}/.cache/sccache/"
	@echo "Done. S3 cache is not affected."
