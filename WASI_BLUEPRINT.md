# WASI Project Blueprint & Autonomous Execution Instructions

## Role & Context

You are an autonomous AI software engineer. Your task is to build "WASI", a lightweight, native WhatsApp web wrapper for Arch Linux. Your user is a 3rd-year Software Engineering student; therefore, your code must be well-structured, thoroughly documented, and adhere to strict software engineering standards.

You have permission to use all available tools (terminal execution, file reading/writing, browser). Execute the following phases sequentially. Do not stop and wait for user permission between steps unless a terminal command fails and you cannot recover after 3 attempts.

## Project Specifications

- **Name:** WASI
- **Author:** Asitha Kanchana (GitHub: https://github.com/AsithaKanchana1)
- **Tech Stack:** Rust, Tauri (pure Rust backend, no heavy JS frameworks), WebKitGTK.
- **Core Requirements:**
  1.  **Low Resource Usage:** Do not use Electron. Rely on the native system webview.
  2.  **Persistent Sessions:** WhatsApp must not log out. Configure Tauri to maintain a persistent data directory for LocalStorage/IndexedDB.
  3.  **User-Agent Spoofing:** Override the default WebKitGTK User-Agent to mimic a modern desktop browser (e.g., latest Chrome on Linux) to prevent WhatsApp from blocking the session.
  4.  **Background Operation:** Implement a system tray using `libayatana-appindicator`.
  5.  **Window Management:** Intercept the window `CloseRequested` event. Hide the window instead of killing the app so WebSockets remain open for notifications.
  6.  **Frontend:** Simply load `https://web.whatsapp.com` in the main Tauri window.

## Autonomous Execution Plan

### Phase 1: Environment & Scaffolding

1.  Verify Arch Linux dependencies are installed. Run: `sudo pacman -S --needed webkit2gtk base-devel curl wget file openssl appmenu-gtk-module gtk3 libappindicator-gtk3 librsvg`. (Assume the user has `sudo` privileges if prompted, but try to handle it gracefully).
2.  Initialize the Tauri project using Cargo. Use vanilla configurations.
3.  Update the `Cargo.toml` to include the author details (Asitha Kanchana) and required dependencies (e.g., `tauri` with `system-tray` features enabled).

### Phase 2: Core Implementation

1.  **Configure `tauri.conf.json`:**
    - Set the app identifier to `com.asithakanchana.wasi`.
    - Configure the system tray icon.
    - Set the default window size to 1024x768.
    - Enable the `macOSPrivateApi` if necessary for backgrounding, though focus is Linux.
2.  **Implement `src-tauri/src/main.rs`:**
    - Write the system tray logic (Show, Quit).
    - Write the window event handler to prevent closure and hide the window.
    - Apply the User-Agent spoofing logic during window creation.
    - Ensure the webview points to `https://web.whatsapp.com`.

### Phase 3: Automated Testing & QA

1.  **Unit Testing:** Write Rust unit tests in `main.rs` to verify that the system tray menu items are correctly constructed and that the application state can be instantiated. Run `cargo test`.
2.  **Build Verification:** Run `cargo tauri build`. Resolve any compiler errors or dependency issues automatically by reading the stderr output and applying fixes.
3.  **QA Checks:**
    - Verify that `libayatana-appindicator` is properly linked.
    - Verify the final binary size is optimized (release build).
    - Check memory safety and handle all `Result/Option` unwraps gracefully (no raw `.unwrap()` that could cause panics in production).

### Execution Rules

- **Self-Correction:** If a build fails, read the error, explain your fix internally, rewrite the file, and re-run the build.
- **No Placeholders:** Write the full code. Do not use `// ... rest of code`.
- **Completion:** Once Phase 3 passes, output a final success message summarizing the build path and instructions on how to run the WASI binary.

BEGIN EXECUTION AT PHASE 1 NOW.
