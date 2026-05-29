const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');

try {
  console.log('Building with leptos...');
  execSync('cargo leptos build --release', { 
    stdio: 'inherit',
    cwd: __dirname
  });

  const wasmPath = path.join(__dirname, 'target', 'site', 'pkg', 'app.wasm');
  const bgWasmPath = path.join(__dirname, 'target', 'site', 'pkg', 'app_bg.wasm');

  if (fs.existsSync(wasmPath)) {
    fs.renameSync(wasmPath, bgWasmPath);
    console.log('Renamed app.wasm to app_bg.wasm');
  }
} catch (error) {
  console.error('Build script failed:', error);
  process.exit(1);
}
