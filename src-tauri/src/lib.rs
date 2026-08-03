mod actions;
mod export;
mod media;
mod office;
mod preview;
mod psd;
mod scan;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use scan::ScanResult;
use tauri::{AppHandle, Emitter, Manager, State};

struct AppState {
    cancel: Arc<AtomicBool>,
}

#[tauri::command]
async fn scan_folder(
    app: AppHandle,
    state: State<'_, AppState>,
    path: String,
) -> Result<ScanResult, String> {
    app.asset_protocol_scope()
        .allow_directory(&path, true)
        .map_err(|e| format!("не удалось открыть доступ к папке: {e}"))?;

    state.cancel.store(false, Ordering::SeqCst);
    let cancel = state.cancel.clone();
    let handle = app.clone();

    let result = tauri::async_runtime::spawn_blocking(move || {
        scan::scan_folder(
            &path,
            Some(cancel),
            Some(Box::new(move |processed| {
                let _ = handle.emit("scan-progress", processed);
            })),
        )
    })
    .await
    .map_err(|e| e.to_string())?;

    state.cancel.store(false, Ordering::SeqCst);
    Ok(result)
}

#[tauri::command]
fn cancel_scan(state: State<'_, AppState>) {
    state.cancel.store(true, Ordering::SeqCst);
}

#[tauri::command]
fn move_files(items: Vec<actions::MoveItem>) -> actions::MoveReport {
    actions::move_files(&items)
}

#[tauri::command]
fn undo_move(items: Vec<actions::UndoItem>) -> actions::MoveReport {
    actions::undo_move(&items)
}

#[tauri::command]
fn export_inventory(
    rows: Vec<export::ExportRow>,
    format: String,
    dest: String,
    delimiter: Option<String>,
) -> Result<(), String> {
    let delim = delimiter.unwrap_or_else(|| ";".to_string());
    match format.to_lowercase().as_str() {
        "csv" => export::write_csv(&dest, &rows, &delim),
        "json" => export::write_json(&dest, &rows),
        other => Err(format!("неизвестный формат экспорта: {other}")),
    }
}

#[tauri::command]
async fn preview_file(app: AppHandle, path: String) -> Result<preview::PreviewData, String> {
    preview::preview_file(app, path).await
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(AppState {
            cancel: Arc::new(AtomicBool::new(false)),
        })
        .invoke_handler(tauri::generate_handler![
            scan_folder,
            cancel_scan,
            move_files,
            undo_move,
            export_inventory,
            preview_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
