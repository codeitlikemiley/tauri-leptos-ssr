/// Holds the in-process Axum server task handle so it can be
/// gracefully aborted when the window is closed.
///
/// To debug the release build:
/// `cargo tauri build -vv`
/// Then go to /Applications -> Show Package Contents -> Contents -> MacOS -> run the binary
struct ServerTask(tauri::async_runtime::JoinHandle<()>);

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_log::Builder::default().build())
        .plugin(tauri_plugin_process::init())
        .setup(|app| {
            use tauri::Manager;
            let app_data_dir = app.path().app_local_data_dir().map_err(|e| {
                Box::<dyn std::error::Error>::from(format!(
                    "Failed to get app local data directory: {}",
                    e
                ))
            })?;
            if !app_data_dir.exists() {
                std::fs::create_dir_all(&app_data_dir).map_err(|e| {
                    Box::<dyn std::error::Error>::from(format!(
                        "Failed to create app local data directory: {}",
                        e
                    ))
                })?;
            }
            std::env::set_var("STORAGE_PATH", app_data_dir.to_string_lossy().to_string());

            #[cfg(not(debug_assertions))]
            {
                use leptos::prelude::get_configuration;
                use tauri::{Manager, Url};

                if std::env::var("LEPTOS_OUTPUT_NAME").is_err() {
                    std::env::set_var("LEPTOS_OUTPUT_NAME", "app");
                }

                let resource_dir = app.path().resource_dir().map_err(|e| {
                    Box::<dyn std::error::Error>::from(format!(
                        "Failed to get resource directory: {}",
                        e
                    ))
                })?;
                let site_root = resource_dir.join("site");
                let cargo_toml_path = resource_dir.join("Cargo.toml");

                std::env::set_var("LEPTOS_SITE_ROOT", site_root.to_string_lossy().to_string());

                let cargo_toml_str = cargo_toml_path.to_str().ok_or_else(|| {
                    Box::<dyn std::error::Error>::from("Cargo.toml path is not valid UTF-8")
                })?;
                let mut conf = get_configuration(Some(cargo_toml_str)).map_err(|e| {
                    Box::<dyn std::error::Error>::from(format!(
                        "Failed to load leptos configuration: {}",
                        e
                    ))
                })?;
                conf.leptos_options.site_root = site_root.to_string_lossy().to_string().into();

                let router = app::build_router(conf.leptos_options);

                let (port, listener) = tauri::async_runtime::block_on(async {
                    let listener = match tokio::net::TcpListener::bind("127.0.0.1:0").await {
                        Ok(l) => l,
                        Err(_) => tokio::net::TcpListener::bind("[::1]:0")
                            .await
                            .map_err(|e| {
                                Box::<dyn std::error::Error>::from(format!(
                                    "Failed to bind tcp listener: {}",
                                    e
                                ))
                            })?,
                    };
                    let port = listener
                        .local_addr()
                        .map_err(|e| {
                            Box::<dyn std::error::Error>::from(format!(
                                "Failed to get local addr: {}",
                                e
                            ))
                        })?
                        .port();
                    Ok::<_, Box<dyn std::error::Error>>((port, listener))
                })?;

                let server_task = tauri::async_runtime::spawn(async move {
                    let _ = axum::serve(listener, router.into_make_service()).await;
                });
                app.manage(ServerTask(server_task));

                // Wait for the server to be ready before navigating
                tauri::async_runtime::block_on(async {
                    let addr = format!("127.0.0.1:{}", port);
                    for _ in 0..50 {
                        if tokio::net::TcpStream::connect(&addr).await.is_ok() {
                            break;
                        }
                        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
                    }
                });

                let window = app.get_webview_window("main").ok_or_else(|| {
                    Box::<dyn std::error::Error>::from("Failed to get main window")
                })?;
                let url = Url::parse(&format!("http://127.0.0.1:{}", port)).map_err(|e| {
                    Box::<dyn std::error::Error>::from(format!("Failed to parse URL: {}", e))
                })?;
                window.navigate(url).map_err(|e| {
                    Box::<dyn std::error::Error>::from(format!("Failed to navigate window: {}", e))
                })?;
            }
            let _ = app;
            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { .. } = event {
                println!("Window close requested, cleaning up...");
                use tauri::Manager;
                if let Some(task) = window.try_state::<ServerTask>() {
                    task.0.abort();
                    println!("Axum server task aborted successfully.");
                }
            }
        })
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
