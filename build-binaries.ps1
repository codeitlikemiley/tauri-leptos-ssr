Write-Host "Building with leptos..." -ForegroundColor Green
cargo leptos build --release

if ($LASTEXITCODE -ne 0) {
    Write-Host "Leptos build failed!" -ForegroundColor Red
    exit $LASTEXITCODE
}

if (Test-Path "target\site\pkg\app.wasm") {
    Rename-Item "target\site\pkg\app.wasm" "app_bg.wasm"
    Write-Host "Renamed app.wasm to app_bg.wasm" -ForegroundColor Green
}