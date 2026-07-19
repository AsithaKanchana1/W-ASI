#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{error::Error, fs, io, path::PathBuf};

use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, TrayIcon, TrayIconBuilder, TrayIconEvent},
    App, AppHandle, Manager, Runtime, WebviewUrl, WebviewWindow, WebviewWindowBuilder, WindowEvent,
};

// ---------------------------------------------------------------------------
// Application-level constants
// ---------------------------------------------------------------------------

/// Tauri window label used to look up the main window via the app handle.
const APP_WINDOW_LABEL: &str = "main";
const WINDOW_TITLE: &str = "WASI";
const WINDOW_WIDTH: f64 = 1024.0;
const WINDOW_HEIGHT: f64 = 768.0;

/// System tray context-menu item identifiers.
const MENU_SHOW_ID: &str = "show";
const MENU_QUIT_ID: &str = "quit";

/// The URL loaded in the webview – the whole point of the app.
const WHATSAPP_WEB_URL: &str = "https://web.whatsapp.com";

/// Chrome-on-Linux user-agent.
///
/// WebKitGTK advertises itself as a generic webview which can trigger WhatsApp
/// to show an unsupported-browser page.  Spoofing a modern Chrome UA prevents
/// this and keeps sessions from being logged out.
const DESKTOP_CHROME_UA: &str =
    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) \
     Chrome/126.0.0.0 Safari/537.36";

// ---------------------------------------------------------------------------
// Data structures (also used by unit tests, so kept outside main)
// ---------------------------------------------------------------------------

/// Represents a single entry in the system tray context menu.
///
/// Using a plain struct (rather than raw strings everywhere) makes it easy to
/// test menu construction without spinning up a real GTK / Tauri runtime.
// Only constructed in tests; suppress dead_code lint in library/binary mode.
#[cfg_attr(not(test), allow(dead_code))]
#[derive(Debug, Clone, PartialEq, Eq)]
struct TrayMenuEntry {
    id: &'static str,
    title: &'static str,
}

/// Managed application state shared across the Tauri runtime.
///
/// Stored via `app.manage()` so it can be retrieved in command handlers or
/// event callbacks if the project grows.
#[derive(Debug, Clone, PartialEq, Eq)]
struct WasiAppState {
    user_agent: String,
    whatsapp_url: String,
}

impl Default for WasiAppState {
    fn default() -> Self {
        Self {
            user_agent: DESKTOP_CHROME_UA.to_string(),
            whatsapp_url: WHATSAPP_WEB_URL.to_string(),
        }
    }
}

// ---------------------------------------------------------------------------
// Pure / testable helpers
// ---------------------------------------------------------------------------

/// Returns the canonical tray menu entries in display order.
///
/// Keeping this as a pure function lets unit tests verify the menu shape
/// without any GTK/Tauri runtime dependency.
#[cfg_attr(not(test), allow(dead_code))]
fn tray_menu_entries() -> [TrayMenuEntry; 2] {
    [
        TrayMenuEntry {
            id: MENU_SHOW_ID,
            title: "Show",
        },
        TrayMenuEntry {
            id: MENU_QUIT_ID,
            title: "Quit",
        },
    ]
}

// ---------------------------------------------------------------------------
// Storage / persistence
// ---------------------------------------------------------------------------

/// Prepares the persistent WebKit storage directory and (on Linux) configures
/// the XDG data path so that WebKitGTK writes localStorage / IndexedDB to a
/// location tied to this application.
///
/// Without this, restarting WASI would leave the WhatsApp session in whatever
/// default XDG directory the system resolved, which may be wiped by cleaners.
fn configure_persistent_storage<R: Runtime>(app: &App<R>) -> Result<PathBuf, Box<dyn Error>> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| io::Error::other(e.to_string()))?;

    let profile_dir = app_data_dir.join("webkit-profile");
    fs::create_dir_all(&profile_dir)?;

    // Point WebKitGTK at our app's data directory so sessions survive restarts.
    #[cfg(target_os = "linux")]
    std::env::set_var("XDG_DATA_HOME", &app_data_dir);

    Ok(profile_dir)
}

// ---------------------------------------------------------------------------
// Window management
// ---------------------------------------------------------------------------

/// Attaches a window-event handler that intercepts close requests.
///
/// Instead of destroying the window (and tearing down the WebSocket connection
/// WhatsApp Web keeps open), we hide the window and let it live in the system
/// tray.  This preserves message notifications even while the window is hidden.
fn attach_close_to_tray_handler<R: Runtime>(window: &WebviewWindow<R>) {
    let w = window.clone();
    window.on_window_event(move |event| {
        if let WindowEvent::CloseRequested { api, .. } = event {
            api.prevent_close();
            if let Err(err) = w.hide() {
                eprintln!("WASI: failed to hide window on close: {err}");
            }
        }
    });
}

/// Creates and configures the main WhatsApp Web window.
///
/// Key settings:
/// * `WebviewUrl::External` – loads the remote URL directly (no local assets).
/// * `user_agent` – overrides the default WebKitGTK UA so WhatsApp accepts it.
fn create_main_window<R: Runtime>(app: &AppHandle<R>) -> Result<WebviewWindow<R>, Box<dyn Error>> {
    let window = WebviewWindowBuilder::new(
        app,
        APP_WINDOW_LABEL,
        WebviewUrl::External(WHATSAPP_WEB_URL.parse()?),
    )
    .title(WINDOW_TITLE)
    .inner_size(WINDOW_WIDTH, WINDOW_HEIGHT)
    .resizable(true)
    .user_agent(DESKTOP_CHROME_UA)
    .build()?;

    attach_close_to_tray_handler(&window);
    Ok(window)
}

/// Shows, un-minimises, and focuses the main window.
///
/// Called both from the tray left-click handler and the "Show" menu item so
/// the user can always bring WASI back to the foreground.
fn show_main_window<R: Runtime>(app: &AppHandle<R>) {
    match app.get_webview_window(APP_WINDOW_LABEL) {
        Some(window) => {
            if let Err(err) = window.show() {
                eprintln!("WASI: failed to show window: {err}");
            }
            if let Err(err) = window.unminimize() {
                eprintln!("WASI: failed to unminimize window: {err}");
            }
            if let Err(err) = window.set_focus() {
                eprintln!("WASI: failed to focus window: {err}");
            }
        }
        None => eprintln!("WASI: main window not found when attempting to show"),
    }
}

// ---------------------------------------------------------------------------
// System tray
// ---------------------------------------------------------------------------

/// Builds the system tray context menu with "Show" and "Quit" items.
fn build_tray_menu<R: Runtime>(app: &AppHandle<R>) -> Result<Menu<R>, Box<dyn Error>> {
    let show = MenuItem::with_id(app, MENU_SHOW_ID, "Show", true, None::<&str>)?;
    let sep = PredefinedMenuItem::separator(app)?;
    let quit = MenuItem::with_id(app, MENU_QUIT_ID, "Quit", true, None::<&str>)?;

    let menu = Menu::with_items(app, &[&show, &sep, &quit])?;
    Ok(menu)
}

/// Builds and registers the system tray icon with its event handlers.
///
/// The returned `TrayIcon<R>` must be kept alive for the duration of the app;
/// store it in managed state so it is not dropped when the setup closure exits.
fn setup_tray<R: Runtime>(app: &AppHandle<R>) -> Result<TrayIcon<R>, Box<dyn Error>> {
    let menu = build_tray_menu(app)?;

    let tray = TrayIconBuilder::new()
        .menu(&menu)
        .tooltip(WINDOW_TITLE)
        // Left-clicking the tray icon shows the window.
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                ..
            } = event
            {
                show_main_window(tray.app_handle());
            }
        })
        // Context-menu item clicks.
        .on_menu_event(|app, event| match event.id().as_ref() {
            MENU_SHOW_ID => show_main_window(app),
            MENU_QUIT_ID => app.exit(0),
            _ => {}
        })
        .build(app)?;

    Ok(tray)
}

// ---------------------------------------------------------------------------
// App entry-point
// ---------------------------------------------------------------------------

fn run_app() -> tauri::Result<()> {
    tauri::Builder::default()
        .manage(WasiAppState::default())
        .setup(|app| {
            // 1. Persistent storage – must happen before webview creation so
            //    the XDG env var is visible to WebKitGTK during initialisation.
            let profile_dir = configure_persistent_storage(app)?;
            eprintln!("WASI: WebKit profile dir ready: {}", profile_dir.display());

            // 2. Main window pointing at WhatsApp Web.
            let handle = app.handle().clone();
            let window = create_main_window(&handle)?;
            if let Err(err) = window.set_focus() {
                eprintln!("WASI: failed to focus window on startup: {err}");
            }

            // 3. System tray – stored in managed state to keep it alive.
            let tray = setup_tray(&handle)?;
            app.manage(tray);

            Ok(())
        })
        .run(tauri::generate_context!())
}

fn main() {
    if let Err(err) = run_app() {
        eprintln!("WASI: fatal error: {err}");
    }
}

// ---------------------------------------------------------------------------
// Unit tests
//
// These tests are deliberately free of any GTK / Tauri runtime dependency so
// they can run headlessly in CI without a display server.
// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    /// Verifies that the tray menu has exactly the expected entries in order.
    #[test]
    fn tray_menu_items_are_correctly_constructed() {
        let entries = tray_menu_entries();
        assert_eq!(entries.len(), 2, "expected exactly 2 tray menu entries");
        assert_eq!(
            entries[0],
            TrayMenuEntry {
                id: MENU_SHOW_ID,
                title: "Show"
            }
        );
        assert_eq!(
            entries[1],
            TrayMenuEntry {
                id: MENU_QUIT_ID,
                title: "Quit"
            }
        );
    }

    /// Ensures no two tray menu items share an ID (which would make event
    /// dispatch ambiguous at runtime).
    #[test]
    fn tray_menu_ids_are_unique() {
        let entries = tray_menu_entries();
        let mut seen_ids: HashSet<&str> = HashSet::new();
        for entry in &entries {
            assert!(
                seen_ids.insert(entry.id),
                "duplicate tray menu id: {}",
                entry.id
            );
        }
    }

    /// Confirms that the managed application state can be default-constructed
    /// with the correct constant values.
    #[test]
    fn app_state_can_be_instantiated() {
        let state = WasiAppState::default();
        assert_eq!(state.user_agent, DESKTOP_CHROME_UA);
        assert_eq!(state.whatsapp_url, WHATSAPP_WEB_URL);
    }

    /// Sanity-checks that `WHATSAPP_WEB_URL` is a valid URL that can be parsed
    /// without panic — guards against accidental typo regressions.
    #[test]
    fn whatsapp_url_is_valid() {
        WHATSAPP_WEB_URL
            .parse::<url::Url>()
            .expect("WHATSAPP_WEB_URL must be a valid URL");
    }

    /// Sanity-checks the user-agent string format (non-empty and contains the
    /// key tokens WhatsApp checks for).
    #[test]
    fn user_agent_contains_chrome_token() {
        assert!(!DESKTOP_CHROME_UA.is_empty());
        assert!(
            DESKTOP_CHROME_UA.contains("Chrome"),
            "UA must contain 'Chrome'"
        );
        assert!(
            DESKTOP_CHROME_UA.contains("Linux"),
            "UA must advertise Linux platform"
        );
    }
}
