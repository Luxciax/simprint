use crate::core::error::Result;
use crate::infrastructure::updater::types::InstallStrategy;
use crate::services::updater::{CheckResult, DownloadResult, PreparedUpdateInfo};
use tauri::AppHandle;

#[tauri::command]
pub async fn check_update_available() -> Result<bool> {
    Ok(false)
}

#[tauri::command]
pub async fn check_updates(_app_handle: AppHandle) -> Result<CheckResult> {
    log::info!("No-login build: app update checks are disabled");
    Ok(CheckResult {
        has_updates: false,
        update_count: None,
        plan_available: false,
    })
}

#[tauri::command]
pub async fn download_updates(
    _app_handle: AppHandle,
    _plan_file: Option<String>,
) -> Result<DownloadResult> {
    log::info!("No-login build: app update downloads are disabled");
    Ok(DownloadResult {
        has_updates: false,
        tasks_file: None,
        install_strategy: InstallStrategy::DirectReplace,
        success_count: 0,
    })
}

#[tauri::command]
pub async fn start_update_install(_app_handle: AppHandle) -> Result<()> {
    log::info!("No-login build: app update install is disabled");
    Ok(())
}

#[tauri::command]
pub async fn start_prepared_update_install(
    _app_handle: AppHandle,
    _kind: Option<String>,
) -> Result<()> {
    log::info!("No-login build: prepared update install is disabled");
    Ok(())
}

#[tauri::command]
pub async fn get_prepared_update() -> Result<Option<PreparedUpdateInfo>> {
    Ok(None)
}
