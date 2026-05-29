# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

### Architecture Migration: Sidecar → In-Process SSR

The Leptos SSR server has been moved from an external sidecar binary into the Tauri process itself. This eliminates the need to ship, spawn, and manage a separate companion executable.

#### What Made This Possible

1. **Leptos `build_router()` extraction** — The Axum router setup was refactored out of the standalone `server` crate into a reusable `build_router(leptos_options)` function in the shared `app` crate. This single function returns a fully configured `axum::Router` with Leptos routes, server function handlers, CORS, and static file serving.

2. **Tauri in-process Tokio task** — Instead of spawning a sidecar via `tauri_plugin_shell`, the Tauri `setup()` hook now creates a `TcpListener` bound to `127.0.0.1:0` (OS-assigned port), spawns `axum::serve()` as a Tokio task, and navigates the WebView to the dynamically allocated port.

3. **Tauri resource bundling** — The compiled site assets (`target/site/`) and workspace `Cargo.toml` are bundled as Tauri resources, allowing the in-process server to locate its assets and configuration at runtime without filesystem assumptions.

#### Added

- **`app::build_router()`** — Shared Axum router builder used by both the standalone `server` binary (dev mode) and the Tauri in-process server (release mode).
- **Server functions** — `get_count()` and `increment_count()` with `#[server(prefix = "/api")]` for true SSR state management.
- **Storage abstraction** — `app::storage` module with `#[cfg(feature = "spin")]` for Spin KV and filesystem fallback for Tauri/standalone, using the `STORAGE_PATH` env var.
- **`CHANGELOG.md`** — This file.
- **Dark themed UI** — Spin-counter inspired design with optimistic updates, loading states, and localStorage caching.

#### Changed

- **`src-tauri/src/lib.rs`** — Replaced sidecar spawning with in-process Axum server. The server runs as a managed Tokio task (`ServerTask`) that is aborted on window close. Uses dynamic port binding (`127.0.0.1:0`) with IPv6 fallback.
- **`src-tauri/Cargo.toml`** — Added direct dependencies on `app` (with `ssr` feature), `leptos`, `axum`, and `tokio`. The Tauri binary now compiles the SSR server natively.
- **`src-tauri/tauri.conf.json`** — Removed `externalBin` configuration. Added `resources` mapping to bundle `target/site/` and `Cargo.toml` into the app bundle.
- **`src-tauri/capabilities/default.json`** — Stripped down to `core:default` only. Removed `shell:allow-execute`, `shell:allow-spawn`, and `shell:allow-open` permissions.
- **`build-binaries.sh`** — Simplified to just `cargo leptos build --release` + WASM filename fix. No longer compiles a separate server binary or copies it to a sidecar location.
- **`server/src/main.rs`** — Simplified to use `app::build_router()` instead of duplicating router setup.
- **`README.md`** — Complete rewrite reflecting the new architecture.
- **`PROJECT.md`** — Updated to document the in-process architecture and completed milestones.

#### Removed

- **`src-tauri/src/sidecar.rs`** — Deleted. The sidecar module that spawned the external server binary via `tauri_plugin_shell::ShellExt` is no longer needed.
- **Sidecar binary build step** — `build-binaries.sh` no longer compiles `cargo build --release --bin server --target $TARGET` or copies binaries to `src-tauri/binaries/`.
- **`externalBin` config** — Removed from `tauri.conf.json`.
- **Shell execution permissions** — Removed from `capabilities/default.json`.

#### Benefits

| Aspect | Before (Sidecar) | After (In-Process) |
|---|---|---|
| Binaries shipped | 2 (Tauri + server) | 1 (Tauri only) |
| Bundle size | Larger (duplicate deps) | Smaller (shared deps) |
| Port conflicts | Possible (hardcoded :3000) | None (OS-assigned port) |
| Process management | Complex (spawn, monitor, kill) | Simple (Tokio task abort) |
| Memory | Two processes | One process |
| Startup time | Slower (spawn + wait) | Faster (task + connect) |
| SSR | ✅ Full | ✅ Full |
| Server functions | ✅ Via HTTP to sidecar | ✅ Via HTTP in-process |
