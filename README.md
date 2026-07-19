<div align="center">

<img src="src-tauri/icons/128x128.png" alt="WASI logo" width="100"/>

# WASI — WhatsApp Web for Arch Linux

**A lightweight, native WhatsApp Web wrapper built with Rust + Tauri + WebKitGTK.**  
No Electron. No heavy JS framework. Just the real system webview — fast, small, and always running in the tray.

[![CI](https://github.com/AsithaKanchana1/W-ASI/actions/workflows/ci.yml/badge.svg)](https://github.com/AsithaKanchana1/W-ASI/actions/workflows/ci.yml)
[![AUR version](https://img.shields.io/aur/version/wasi-whatsapp?label=AUR&logo=archlinux)](https://aur.archlinux.org/packages/wasi-whatsapp)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange?logo=rust)](https://www.rust-lang.org/)

</div>

---

## ✨ Features

| Feature | Details |
|---|---|
| 🪶 **Lightweight** | Uses the system WebKitGTK webview — not Electron. Binary is < 5 MB. |
| 🔒 **Persistent Sessions** | WhatsApp stays logged in across restarts via dedicated WebKit profile directory. |
| 🕵️ **User-Agent Spoofing** | Mimics Chrome on Linux so WhatsApp never shows the "unsupported browser" page. |
| 🔔 **Background Notifications** | Closing the window hides it to the system tray; WebSocket stays alive for notifications. |
| 🖥️ **System Tray** | Left-click to show, right-click for Show / Quit menu via `libayatana-appindicator`. |
| 📦 **Easy Install** | Available as an AppImage, a `yay`/`paru` AUR package, or a local source build. |

---

## 📸 Screenshots

> _Window and tray icon running on KDE Plasma (Arch Linux)._

<!-- Add screenshots here once the app is running -->
<!--
![WASI main window](docs/screenshot-window.png)
![WASI tray menu](docs/screenshot-tray.png)
-->

---

## 📥 Installation

### Option 1 — AUR (Recommended for Arch Linux)

```bash
# Using yay
yay -S wasi-whatsapp

# Using paru
paru -S wasi-whatsapp
```

> The AUR package builds WASI from the latest GitHub release source tarball.

---

### Option 2 — AppImage (Universal Linux)

1. Download the latest `WASI_*.AppImage` from [Releases](https://github.com/AsithaKanchana1/W-ASI/releases/latest).
2. Make it executable and run:

```bash
chmod +x WASI_*.AppImage
./WASI_*.AppImage
```

> For system tray support on non-AppIndicator desktops, install `libayatana-appindicator`.

---

### Option 3 — Build from Source

#### Prerequisites

Install required Arch Linux packages:

```bash
sudo pacman -S --needed \
  webkit2gtk-4.1 \
  base-devel curl wget file \
  openssl appmenu-gtk-module \
  gtk3 libappindicator-gtk3 \
  librsvg rust cargo
```

Also install the Tauri CLI:

```bash
cargo install tauri-cli --version "^2"
```

#### Build

```bash
git clone https://github.com/AsithaKanchana1/W-ASI.git
cd W-ASI
make build
```

The release binary will be at:
```
src-tauri/target/release/wasi
```

AppImage, `.deb`, and other bundles will be under:
```
src-tauri/target/release/bundle/
```

---

## 🚀 Usage

### Running

```bash
# Run the compiled binary directly
./src-tauri/target/release/wasi

# Or via Makefile
make run
```

On first launch, WhatsApp Web will ask you to scan a QR code. After that, the session is saved permanently.

### System Tray

| Action | Result |
|---|---|
| **Left-click** tray icon | Show / restore the window |
| **Right-click** → Show | Show / restore the window |
| **Right-click** → Quit | Fully exit WASI |
| **Close button** (✕) | Hides the window — app keeps running |

### Session Storage

WASI stores your WhatsApp session at:

```
~/.local/share/com.asithakanchana.wasi/webkit-profile/
```

To reset / log out, delete this directory.

---

## 🔧 Development

```bash
# Run unit tests (no display server required)
make test

# Lint & format
make lint
make fmt

# Build everything (AppImage + .deb)
make build

# Build only AppImage
make appimage

# Clean build artifacts
make clean
```

### Project Structure

```
W-ASI/
├── src-tauri/
│   ├── src/
│   │   └── main.rs          # Core Rust app (tray, window, UA spoof)
│   ├── icons/               # App icons (PNG, ICNS, ICO)
│   ├── Cargo.toml           # Rust dependencies
│   ├── tauri.conf.json      # Tauri configuration
│   └── build.rs             # Tauri build script
├── dist/
│   └── index.html           # Minimal HTML shim (Tauri frontend)
├── aur/
│   ├── PKGBUILD             # Arch Linux AUR build script
│   ├── .SRCINFO             # AUR metadata
│   └── wasi.desktop         # XDG desktop entry
├── .github/
│   └── workflows/
│       └── ci.yml           # GitHub Actions CI/CD pipeline
└── Makefile                 # Build convenience targets
```

---

## 🤝 Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository
2. Create a feature branch: `git checkout -b feat/my-feature`
3. Run tests: `make test`
4. Run linting: `make lint`
5. Commit with a clear message: `git commit -m "feat: describe your change"`
6. Push and open a Pull Request

Please ensure `cargo clippy -- -D warnings` passes before opening a PR — this is enforced in CI.

---

## 🗑️ Uninstall

### AUR

```bash
yay -Rns wasi-whatsapp
```

### AppImage

Simply delete the `.AppImage` file. To remove session data:

```bash
rm -rf ~/.local/share/com.asithakanchana.wasi
```

### Source Build

```bash
make clean
rm -rf ~/.local/share/com.asithakanchana.wasi
```

---

## 📄 License

This project is licensed under the **MIT License** — see the [LICENSE](LICENSE) file for details.

---

<div align="center">
Built with ❤️ by <a href="https://github.com/AsithaKanchana1">Asitha Kanchana</a> · Powered by <a href="https://tauri.app">Tauri</a> + <a href="https://webkitgtk.org">WebKitGTK</a>
</div>
