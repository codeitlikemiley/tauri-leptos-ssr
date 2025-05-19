use tauri::{Manager, Url};
use tauri_plugin_shell::ShellExt;
use tauri_plugin_shell::process::{CommandEvent, CommandChild};
use std::sync::{Arc, Mutex};

// Global storage for the child process
static LEPTOS_SSR_SIDECAR_PROCESS: std::sync::OnceLock<Arc<Mutex<Option<CommandChild>>>> = std::sync::OnceLock::new();

pub fn kill_sidecar_process() {
    if let Some(child_handle) = LEPTOS_SSR_SIDECAR_PROCESS.get() {
        if let Ok(mut child_guard) = child_handle.lock() {
            if let Some(child) = child_guard.take() {
                println!("Killing sidecar process...");
                let _ = child.kill();
            }
        }
    }
}

pub fn setup(app: &mut tauri::App) -> tauri::Result<()> {
    // In Production its important to set different ports to avoid conflicts
    let leptos_address = "127.0.0.1:3000";
    let leptos_reload_port = "3001";

    // Initialize the global child storage
    let leptos_ssr_sidecar = Arc::new(Mutex::new(None));
    LEPTOS_SSR_SIDECAR_PROCESS.set(leptos_ssr_sidecar.clone())
        .expect("Failed to set global child storage");

    println!("Setting up correct resource directory for Leptos SSR sidecar...");
    
    // If you need to add new resources, check tauri.conf.json and add it to resources array
    let resource_dir = app.path().resource_dir().expect("Failed to get resource dir");
    println!("Resource directory: {:?}", resource_dir);
    
    let site_path = resource_dir.join("site");
    println!("LEPTOS_SITE_ROOT set to: {}", site_path.display());

    // Set up ENV variables for the Leptos SSR sidecar app process
    let leptos_ssr_process = app.shell().sidecar("server").expect("Sidecar app (leptos ssr server) not bundled with Tauri app")
        .env("LEPTOS_SITE_ROOT", site_path.to_string_lossy().to_string())
        .env("LEPTOS_SITE_ADDR", leptos_address)
        .env("LEPTOS_RELOAD_PORT", leptos_reload_port);

    // Start the Leptos server as a sidecar
    let (mut rx, child) = leptos_ssr_process
        .spawn()
        .expect("Failed to spawn Leptos SSR sidecar process");

    // Store the child for cleanup
    if let Ok(mut guard) = leptos_ssr_sidecar.lock() {
        *guard = Some(child);
    }

    // Listen to sidecar output
    tauri::async_runtime::spawn(async move {
        while let Some(event) = rx.recv().await {
            match event {
                CommandEvent::Stdout(line) => {
                    let output = String::from_utf8_lossy(&line);
                    println!("Server stdout: {}", output);
                }
                CommandEvent::Stderr(line) => {
                    let output = String::from_utf8_lossy(&line);
                    println!("Server stderr: {}", output);
                }
                CommandEvent::Terminated(payload) => {
                    println!("Server terminated with code: {:?}", payload.code);
                    break;
                }
                _ => {}
            }
        }
    });

    // Navigate to the server URL
    let window = app.get_webview_window("main").unwrap();
    window.navigate(Url::parse(&format!("http://{}", leptos_address)).unwrap())?;
    println!("Tauri App with Leptos SSR as sidecar started successfully.");

    Ok(())
}