Write-Host "Building with leptos..." -ForegroundColor Green
cargo leptos build --release

if ($LASTEXITCODE -ne 0) {
    Write-Host "Leptos build failed!" -ForegroundColor Red
    exit $LASTEXITCODE
}

Write-Host "Getting target triple..." -ForegroundColor Green
$TARGET = rustc -Vv | Select-String "host:" | ForEach-Object {$_.Line.split(" ")[1]}

Write-Host "Building server for target: $TARGET" -ForegroundColor Green
cargo build --release --bin server --target $TARGET

if ($LASTEXITCODE -ne 0) {
    Write-Host "Server build failed!" -ForegroundColor Red
    exit $LASTEXITCODE
}

Write-Host "Renaming server binary to server-$TARGET..." -ForegroundColor Green
$extension = if ($TARGET -like "*windows*") { ".exe" } else { "" }
Copy-Item "target\$TARGET\release\server$extension" "target\release\server-$TARGET$extension"

# Fix WASM filename - rename app.wasm to app_bg.wasm
if (Test-Path "target\site\pkg\app.wasm") {
    Copy-Item "target\site\pkg\app.wasm" "target\site\pkg\app_bg.wasm"
    Write-Host "Created app_bg.wasm from app.wasm" -ForegroundColor Green
}

Write-Host "Built server-$TARGET successfully!" -ForegroundColor Green