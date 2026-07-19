# WASI — Makefile
# Convenience targets for building, testing, and running the app.
#
# All build targets set NO_STRIP=1 to work around linuxdeploy bundling an
# old 'strip' binary that cannot handle modern Arch Linux ELF .relr.dyn
# sections. Without this flag, AppImage bundling fails on current Arch.

.PHONY: all test build run clean fmt lint appimage deb check-appimage

## Default: run tests then build all bundles
all: test build

## Run unit tests (headless — no display server required)
test:
	cd src-tauri && cargo test

## Lint: clippy (deny warnings) — matches what CI enforces
lint:
	cd src-tauri && cargo clippy --all-targets --all-features -- -D warnings

## Format code
fmt:
	cd src-tauri && cargo fmt --all

## Check formatting without modifying files (same check CI uses)
fmt-check:
	cd src-tauri && cargo fmt --all -- --check

## Build the release binary + all bundles (.deb / .rpm / AppImage)
build:
	cd src-tauri && NO_STRIP=1 cargo tauri build

## Build only the AppImage bundle
appimage:
	cd src-tauri && NO_STRIP=1 cargo tauri build --bundles appimage

## Build only the Debian package
deb:
	cd src-tauri && NO_STRIP=1 cargo tauri build --bundles deb

## QA: verify the AppImage was produced and has a sane size (>1 MB)
check-appimage:
	@APPIMAGE=$$(find src-tauri/target/release/bundle/appimage -name '*.AppImage' 2>/dev/null | head -1); \
	if [ -z "$$APPIMAGE" ]; then \
		echo "❌  AppImage not found. Run 'make appimage' first."; exit 1; \
	fi; \
	SIZE=$$(stat -c%s "$$APPIMAGE"); \
	echo "✅  AppImage: $$APPIMAGE ($$SIZE bytes)"; \
	if [ "$$SIZE" -lt 1000000 ]; then \
		echo "❌  AppImage is suspiciously small ($$SIZE bytes)"; exit 1; \
	fi

## Run the release binary directly (no display-server check)
run:
	./src-tauri/target/release/wasi

## Remove build artefacts
clean:
	cd src-tauri && cargo clean
