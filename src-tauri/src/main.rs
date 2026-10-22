#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use tauri::{
    webview::{PermissionKind, PermissionResponse},
    State,
};

#[tauri::command]
fn begin_camera_permission_request(camera_request: State<'_, Arc<AtomicBool>>) {
    camera_request.store(true, Ordering::SeqCst);
}

#[tauri::command]
fn clear_camera_permission_request(camera_request: State<'_, Arc<AtomicBool>>) {
    camera_request.store(false, Ordering::SeqCst);
}

fn main() {
    let camera_request = Arc::new(AtomicBool::new(false));
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .manage(Arc::clone(&camera_request))
        .invoke_handler(tauri::generate_handler![
            begin_camera_permission_request,
            clear_camera_permission_request
        ])
        .on_permission_request(move |_, kind| match kind {
            PermissionKind::Camera if camera_request.swap(false, Ordering::SeqCst) => {
                PermissionResponse::Allow
            }
            _ => PermissionResponse::Default,
        })
        .run(tauri::generate_context!())
        .expect("falha ao iniciar o aplicativo Libris");
}