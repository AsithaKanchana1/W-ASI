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
| Close window (✕) | Hides to system tray — app keeps running in background |
| Left-click tray icon | Restore window |
| Right-click tray → Quit | Fully exit |

**Session data** is stored at `~/.local/share/com.asithakanchana.wasi/webkit-profile/`.  
Delete this directory to log out.

### Hyprland (no system tray)

Hyprland has no system tray by default, so the tray icon is invisible.  
The window is still hidden when you close it — WhatsApp stays connected in the background.

**Restore the window** by running `wasi` again from any terminal or keybind.  
If WASI is already running, the second invocation sends a show command to the existing instance via a Unix socket (`/tmp/wasi-ipc.sock`) and exits immediately — no second window opens.

Add this to your `~/.config/hypr/hyprland.conf`:

```ini
# Toggle WASI window (show if hidden, launch if not running)
bind = SUPER, W, exec, wasi
```

**About `Super+Q` (`killactive`):**  
Hyprland's `killactive` sends a close request to the window — WASI intercepts this and hides instead of quitting (to keep WhatsApp connected).  
Your `WIN+Q` bind will **hide** WASI, not kill it. Use `SUPER, W` (above) to get it back.

To **fully quit** WASI from Hyprland, use:

```bash
pkill wasi
```

Or add a dedicated keybind:

```ini
bind = SUPER SHIFT, W, exec, pkill wasi
```

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
