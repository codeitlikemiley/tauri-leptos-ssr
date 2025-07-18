# Tauri Leptos SSR and Tailwind 4 Template

This uses leptos ssr embedded as external binary (sidecar) on tauri.

## Getting Started

1. Generate the Template

Note: [cargo-generate](https://cargo-generate.github.io/cargo-generate/installation.html) or [cargo-leptos](https://github.com/leptos-rs/cargo-leptos) must be installed on your system

If it is not yet installed.

```sh
cargo install --locked cargo-generate
cargo install --locked cargo-leptos
```

```sh
# you will be prompted to enter project name
cargo generate --git https://github.com/codeitlikemiley/tauri-leptos-ssr.git
# or with cargo leptos
cargo leptos new --git https://github.com/codeitlikemiley/tauri-leptos-ssr.git

# example output:
# you will be prompted for project name
🤷   Project Name: pikachu
🔧   Destination: /Users/uriah/Code/pikachu ...
🔧   project-name: pikachu ...
🔧   Generating template ...
# you will be prompted to choose an operating system
# this is important to pick the correct build script
✔ 🤷   Operating system (windows/linux/macos) · windows

# change directory to your workspace root directory
cd {{ project-name }}
```

2. Generate Tauri Icons

Note: Without Icons you will have issue with running cargo tauri `{dev | build}`

```sh
# assuming your are on the workspace root directory
cd src-tauri
cargo tauri icon /path/to/icon.png
```

3. Run Build Script

Note: this is required at least to be run once since during tauri compilation it would look for the `server` binary listed on your [tauri.conf.json](./src-tauri/tauri-conf.json)

```json
 "externalBin": [
      "../target/release/server"
    ]
```

During the First Run of `cargo tauri dev` , your `target/release/server` is not found

but if your running `cargo tauri build` that would be generated as the script `build-binaries.sh` would be run

- Linux / MacOS

```sh
# if the script isnt executable yet then run
chmod +x ./build-binaries.sh
# Run the script
./build-binaries.sh
```

- Windows

```powershell
.\build-binaries.ps1
```

4. Development

```sh
cargo tauri dev
```

5. Production Build

```sh
cargo tauri build
```

## Project Structure

```sh
- app # where you add your UI, components and routes
- server # Leptos SSR (sidecar)
- src-tauri # add all your tauri good stuff here and commands
- style # tailwind stuff
```

If you need to run sidecar app process (LEPTOS SSR SERVER) on different port please check [sidecar.rs](./src-tauri/src/sidecar.rs) and update `leptos_address` and `leptos_reload_port`

## Testing Your Project

Install dependencies

Note: add this to your shell configuration file:

- Linux/macOS: .bashrc, .zshrc

```sh
# Linux/MacOS
export PLAYWRIGHT_BROWSER_PATH=0
```

- Windows: PowerShell profile (typically C:\Users\<username>\Documents\PowerShell\Microsoft.PowerShell_profile.ps1)

```powershell
# edit profile
code $PROFILE
# Windows PowerShell
$env:PLAYWRIGHT_BROWSER_PATH=0
```

This will ensure that artifacts like `Chromium`, `ffmpeg` , `webkit`, and `firefox` is scoped under `node_modules/playwright-core/.local-browsers/` instead of global cache directories on Linux/Windows or `$HOME/Library/Caches/ms-playwright/` on macOS

```sh
cd end2end
npm install
npx playwright install
```

Cargo-leptos uses [Playwright](https://playwright.dev) as the end-to-end test tool. 

Prior to the first run of the end-to-end tests run Playwright must be installed. 
In the project's `end2end` directory run `npm install -D playwright @playwright/test` to install playwright and browser specific APIs.

To run the tests during development in the project root run:
```bash
# back to project root directory
cargo leptos end-to-end
```

To run tests for release in the project root run:
```bash
cargo leptos end-to-end --release
```
There are some examples tests are located in `end2end/tests` directory that pass tests with the sample Leptos app.

A web-based report on tests is available by running `npx playwright show-report` in the `end2end` directory.

For Tauri Test using Web Driver [check this out](https://v2.tauri.app/develop/tests/webdriver/)



## Licensing

This template itself is released under the Unlicense. You should replace the LICENSE for your own application with an appropriate license if you plan to release it publicly.
