# WASI — Makefile
# Convenience targets for building, testing, and running the app.
#
# All build targets set NO_STRIP=1 to work around linuxdeploy bundling an
# old 'strip' binary that cannot handle modern Arch Linux ELF .relr.dyn
# sections. Without this flag, AppImage bundling fails on current Arch.

.PHONY: all test build run clean fmt

## Default: run tests then build all bundles
all: test build

## Run unit tests (headless — no display server required)
test:
	cd src-tauri && cargo test

## Format code
fmt:
	cd src-tauri && cargo fmt --all

## Build the release binary + all bundles (.deb / .rpm / AppImage)
build:
	cd src-tauri && NO_STRIP=1 cargo tauri build

## Build only the AppImage bundle
appimage:
	cd src-tauri && NO_STRIP=1 cargo tauri build --bundles appimage

## Build only the Debian package
deb:
	cd src-tauri && NO_STRIP=1 cargo tauri build --bundles deb

## Run the release binary directly (no display-server check)
run:
	./src-tauri/target/release/wasi

## Remove build artefacts
clean:
	cd src-tauri && cargo clean
