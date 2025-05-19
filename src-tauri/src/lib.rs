mod sidecar;

/// TO DEBUG This after you do cargo tauri build -vv
/// Go to /Applications -> Show Package Contents -> Contents -> MacOS -> run the binary
/// This would show the stdout and stderr of the sidecar process and the Tauri app
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_log::Builder::default().build())
        .plugin(tauri_plugin_process::init())
        .setup(|app| {
            // Only start sidecar in production
            if cfg!(not(debug_assertions)) {
                sidecar::setup(app)?;
            }
            Ok(())
        })
        .on_window_event(|_, event| match event {
            tauri::WindowEvent::CloseRequested { .. } => {
                println!("Window close requested, cleaning up...");
                // Force exit to ensure everything dies
                // NOTE if we pressed CMD + Q on macOS, this will not work
                // This would only work if we pressed the close button on the window
                sidecar::kill_sidecar_process();
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
