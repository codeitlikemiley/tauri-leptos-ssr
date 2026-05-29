fn main() {
    let wasm_path = std::path::Path::new("../target/site/pkg/app.wasm");
    let bg_wasm_path = std::path::Path::new("../target/site/pkg/app_bg.wasm");
    if wasm_path.exists() {
        if let Err(e) = std::fs::rename(wasm_path, bg_wasm_path) {
            println!(
                "cargo:warning=Failed to rename app.wasm to app_bg.wasm: {}",
                e
            );
        } else {
            println!("cargo:info=Renamed app.wasm to app_bg.wasm");
        }
    }
    tauri_build::build();
}
