# Stick the Charge

A Rust and egui application for browsing selected Canadian federal offences, reading plain-language drafts alongside legislative text, saving charges, and taking or storing photos locally in the phone browser.

## Legal content

The catalogue is incomplete. Explanations are drafts and have not been comprehensively reviewed by a lawyer. Legislative text is an unofficial, dated reproduction and may not reflect later amendments, court decisions, or all related provisions. Check the linked [Justice Laws website](https://laws-lois.justice.gc.ca/) for the official current law.

## Run on a computer

Install Rust, then run:

```sh
cargo run --release
```

Run the Rust tests with `cargo test --release`.

## Phone browser

The `mobile/` directory contains the progressive web app and a prebuilt WebAssembly package. Serve that directory over HTTPS for installation and camera access; for a local preview, run `python -m http.server 8765` from `mobile/`.

To rebuild after changing Rust code, follow [mobile/README.md](mobile/README.md). Saved charges and photos remain in the browser's local storage.

## Legislation data

The source text and its recorded consolidation date are included in `src/legislation_data.rs`. The tools in `tools/` can refresh selected provisions; see [the catalogue status](docs/federal-offence-status.md) for their limits.
