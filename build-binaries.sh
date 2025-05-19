#!/bin/bash

# Build with leptos
cargo leptos build --release

# Get the target architecture
TARGET=$(rustc -vV | sed -n 's|host: ||p')
# Important we build the server binary for host architecture
cargo build --release --bin server --target $TARGET

# To make the external binary work on each supported architecture,
# a binary with the same name and a -$TARGET_TRIPLE suffix must exist on the specified path.
# For instance, "externalBin": ["binaries/my-sidecar"] requires a src-tauri/binaries/my-sidecar-x86_64-unknown-linux-gnu executable on Linux
# or src-tauri/binaries/my-sidecar-aarch64-apple-darwin on Mac OS with Apple Silicon.
cp target/$TARGET/release/server target/release/server-$TARGET

# Fix WASM filename - rename app.wasm to app_bg.wasm
if [ -f target/site/pkg/app.wasm ]; then
    mv target/site/pkg/app.wasm target/site/pkg/app_bg.wasm
    echo "Renamed app.wasm to app_bg.wasm"
fi

echo "Built server-$TARGET"