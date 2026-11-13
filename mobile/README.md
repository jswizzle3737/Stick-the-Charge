# Stick the Charge on a phone

This is the existing Rust/egui interface compiled to WebAssembly, with a small browser wrapper for the phone camera and local photo storage. It is a progressive web app (PWA), not an Android APK or an iOS App Store package. The charge catalogue is illustrative and should be checked against the linked current legislation.

## Build

From the parent `app` directory, with Rust installed:

```powershell
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli --version 0.2.130 --locked
cargo build --release --target wasm32-unknown-unknown --lib
wasm-bindgen --target web --out-dir mobile\pkg --no-typescript target\wasm32-unknown-unknown\release\stick_the_charge.wasm
```

The `wasm-bindgen-cli` version must match the version in `Cargo.lock`. After changing the service worker's precached asset list, increment the cache version in `sw.js`.

Run `node scripts/package.mjs` from this directory to copy the runnable assets into `dist/` for static HTTPS hosting.

## Run and install

For a local desktop preview, run `python -m http.server 8765 --bind 127.0.0.1` from this directory and open `http://127.0.0.1:8765/`. A phone cannot reach the computer's localhost address. For phone use, serve this directory from a trusted HTTPS address, then open that address on the phone. Add it to the home screen using the browser's install or share menu. Once it has loaded and cached its assets, it can open offline.

Tap **Photos**, then **Open camera** and **Take photo** to save a camera image. You can also use the phone's camera or image chooser if a live preview is unavailable. Images are kept in this browser's local storage and can be downloaded or deleted in the Photos panel; the camera is turned off after capture or when the panel closes. Clearing browser data or uninstalling the app can remove local photos and saved charges, so download any images you need to keep. Photos are not uploaded by this app.

## Verification

From the parent directory, `cargo test --release` checks the Rust logic. The browser test requires Playwright and Chrome, and a running local server on port 8765. For example, after installing Playwright outside this project, set `PLAYWRIGHT_MODULE` to its module directory and `CHROME_PATH` to Chrome's executable, then run `node tests/browser.cjs` here. It checks touch scrolling and category selection, the charge save flow, opening legislation separately, camera shutdown and permission failure, deletion and offline startup. It also captures, decodes and downloads a non-blank JPEG using Chrome's simulated camera and validates imported PNG bytes. It does not replace a physical phone-camera test.
