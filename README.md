# System Monitor

A simple cross-platform system monitor GUI written in Rust using `eframe`/`egui` and `sysinfo`.

Features
- CPU, Memory, Disk, Network, and System information pages
- Per-core CPU graphs and historical data
- Process list with sorting and optional collapsing by process name
- Uses `sysinfo` crate for system metrics

CI
- GitHub Actions workflow runs `cargo fmt -- --check`, `cargo clippy`, and `cargo test` on push/PR to `main`.

Quick start

Prerequisites
- Rust (stable) and `cargo`

Build and run
```bash
cargo run
```

Run tests
```bash
cargo test
```

Formatting and linting
```bash
cargo fmt -- --check
cargo clippy --all-targets --all-features
```

Development notes
- The main application is in `src/main.rs` and UI pages are modularized in `src/`.
- `ProcessList` and `GraphDataList` include unit tests in their modules.
- If you want CI to fail on clippy warnings, change the `ci.yml` clippy step to use `-D warnings`.

Contributing
- Open an issue or submit a pull request with changes. Ensure tests pass and follow Rust formatting and clippy guidelines.