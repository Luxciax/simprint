use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tauri::{AppHandle, Emitter, Manager};

static SPLASHSCREEN_FRONTEND_READY: std::sync::OnceLock<Arc<AtomicBool>> =
    std::sync::OnceLock::new();

fn get_frontend_ready_flag() -> Arc<AtomicBool> {
    SPLASHSCREEN_FRONTEND_READY
        .get_or_init(|| Arc::new(AtomicBool::new(false)))
        .clone()
}

fn is_frontend_ready() -> bool {
    get_frontend_ready_flag().load(Ordering::Acquire)
}

pub fn set_frontend_ready() {
    get_frontend_ready_flag().store(true, Ordering::Release);
}

fn emit_progress(app_handle: &AppHandle, progress: u8, text: &str, status: Option<&str>) {
    if let Some(splash_window) = app_handle.get_webview_window("splashscreen") {
        #[derive(serde::Serialize, Clone)]
        struct ProgressPayload {
            progress: u8,
            text: String,
            status: Option<String>,
        }

        let _ = splash_window.emit(
            "splashscreen-progress",
            ProgressPayload {
                progress,
                text: text.to_string(),
                status: status.map(|s| s.to_string()),
            },
        );
    }
}

fn emit_status_complete(app_handle: &AppHandle, status: &str) {
    if let Some(splash_window) = app_handle.get_webview_window("splashscreen") {
        #[derive(serde::Serialize, Clone)]
        struct StatusCompletePayload {
            status: String,
        }

        let _ = splash_window.emit(
            "splashscreen-status-complete",
            StatusCompletePayload {
                status: status.to_string(),
            },
        );
    }
}

fn emit_ready(app_handle: &AppHandle) {
    if let Some(splash_window) = app_handle.get_webview_window("splashscreen") {
        let _ = splash_window.emit("splashscreen-ready", ());
    }

    if let Some(main_window) = app_handle.get_webview_window("main") {
        let _ = main_window.emit("splashscreen-loading-complete", ());
    }
}

pub fn init_startup(app_handle: AppHandle) {
    tauri::async_runtime::spawn(async move {
        let app_handle_clone = app_handle.clone();

        while !is_frontend_ready() {
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }

        emit_progress(&app_handle_clone, 20, "Initializing...", Some("init"));
        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
        emit_status_complete(&app_handle_clone, "init");

        emit_progress(&app_handle_clone, 45, "Loading local config...", Some("config"));
        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
        emit_status_complete(&app_handle_clone, "config");

        emit_progress(&app_handle_clone, 70, "Starting local no-login mode...", Some("offline"));
        log::info!("No-login build: skipping server connection and app update checks");
        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
        emit_status_complete(&app_handle_clone, "offline");

        if let Err(error) =
            crate::commands::window::create_main_window(app_handle_clone.clone()).await
        {
            log::error!("Failed to create main window: {}", error);
        }

        emit_progress(&app_handle_clone, 100, "Ready", Some("ready"));
        tokio::time::sleep(tokio::time::Duration::from_millis(150)).await;
        emit_status_complete(&app_handle_clone, "ready");
        emit_ready(&app_handle_clone);
    });
}
