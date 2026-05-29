#!/bin/bash

# Build with leptos
cargo leptos build --release

# Fix WASM filename - rename app.wasm to app_bg.wasm
if [ -f target/site/pkg/app.wasm ]; then
    mv target/site/pkg/app.wasm target/site/pkg/app_bg.wasm
    echo "Renamed app.wasm to app_bg.wasm"
fi