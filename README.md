<div align="center">

<img src="src-tauri/icons/128x128.png" alt="WASI" width="96"/>

# WASI

A lightweight WhatsApp Web desktop client for Linux, built with Rust and Tauri.  
Uses the native system WebView — no Electron, no heavy dependencies.

[![CI](https://github.com/AsithaKanchana1/W-ASI/actions/workflows/ci.yml/badge.svg)](https://github.com/AsithaKanchana1/W-ASI/actions/workflows/ci.yml)
[![AUR](https://img.shields.io/aur/version/wasi-whatsapp?label=AUR&logo=archlinux&logoColor=white)](https://aur.archlinux.org/packages/wasi-whatsapp)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue)](LICENSE)

</div>

---

## Installation

### AUR (Arch Linux)

```bash
yay -S wasi-whatsapp
```

### AppImage

Download `WASI_*.AppImage` from [Releases](https://github.com/AsithaKanchana1/W-ASI/releases/latest), then:

```bash
chmod +x WASI_*.AppImage && ./WASI_*.AppImage
```

### Build from Source

**Dependencies:**
```bash
sudo pacman -S --needed webkit2gtk-4.1 gtk3 libappindicator-gtk3 \
  librsvg appmenu-gtk-module base-devel rust
```

**Build:**
```bash
git clone https://github.com/AsithaKanchana1/W-ASI.git
cd W-ASI
make build
```

Binary: `src-tauri/target/release/wasi`

---

## Usage

On first launch, scan the WhatsApp QR code. Your session is saved and survives restarts.

| Action | Result |
|---|---|
| Close window | Hides to system tray (app keeps running) |
| Left-click tray icon | Restore window |
| Right-click tray → Quit | Exit |

**Session data** is stored at `~/.local/share/com.asithakanchana.wasi/webkit-profile/`.  
Delete this directory to log out.

---

## Development

```bash
make test      # Run unit tests (headless)
make lint      # cargo clippy -D warnings
make fmt       # cargo fmt
make build     # Build all bundles (AppImage, .deb)
make clean     # Remove build artifacts
```

---

## License

[MIT](LICENSE) © [Asitha Kanchana](https://github.com/AsithaKanchana1)
