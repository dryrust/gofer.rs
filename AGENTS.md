# Agent guide

## Scope and architecture
- Work inside this repository; do not inspect parent directories.
- Rust 2021 workspace; one crate, `lib/gofer`; minimum Rust 1.81. Preserve `#![deny(unsafe_code)]`.
- Blocking URL reader: `open` returns `Result<Box<dyn Read>>`; `open_buffered` wraps it. `Url` is `dogma::Uri`.
- Source map (`lib/gofer/src/`):
  - `lib.rs`: public re-exports and `RequestConfig`.
  - `open.rs`: scheme dispatch; `open_with_config` currently supports only HTTP(S), Git, IPFS.
  - `schemes.rs`, `schemes/*.rs`: feature-gated, publicly re-exported protocol modules.
  - `schemes/request.rs`: shared ureq 3 agent/header/body handling for HTTP, Git, IPFS.
  - `error.rs`: `Error`, `Result`, conversions, optional miette diagnostics.
  - `features.rs`: enabled-feature inventory; `main.rs`: URL-to-stdout CLI.
- Git URLs map GitHub/GitLab files to HTTPS; IPFS uses an HTTPS gateway.
- Examples: `lib/gofer/examples/`; integration tests: `lib/gofer/tests/`; CI: `.github/workflows/ci.yaml`.

## Editing rules
- Preserve the synchronous `Read` API and streaming where supported; share HTTP plumbing through `schemes/request.rs`.
- For protocol/feature changes, keep `lib/gofer/Cargo.toml`, module gates, dispatch, `FEATURES`, and error variants/conversions consistent. Keep protocol dependencies optional.
- Add rustdoc to every public symbol you introduce or change, including modules, fields, and variants. Explain feature requirements, URL syntax, errors, and buffering when relevant.
- Put detailed documentation in module/type rustdoc. README additions require exceptional value; prefer concise corrections.
- For behavior changes, add focused regression tests: unit tests beside helpers, integration tests under `lib/gofer/tests/`. Use data URLs, temporary files, local servers, and subprocesses for stdin/CLI; keep new tests and runnable doctests independent of the internet.
- For releases, synchronize workspace `Cargo.toml`, `VERSION`, `Cargo.lock`, and `CHANGES.md`; regenerate the lockfile with Cargo.

## Checks
Run from the repository root, as relevant to the change:
```sh
cargo fmt --all -- --check
cargo test --workspace --locked
cargo clippy --workspace --locked --all-targets --all-features
cargo check --workspace --locked --lib --no-default-features --features std
```
- Feature changes: also run `cargo test -p gofer --locked --no-default-features --features std,http` (replace `http` with the changed feature).
- Rustdoc changes: `cargo doc --workspace --locked --no-deps`; the test command above includes doctests.
- Dependency/API changes: verify Rust 1.81 compatibility. CI currently covers Linux and Windows.
- Report commands run, failures, and unavailable checks accurately, including pre-existing failures.

## Known gaps (update when fixed)
- `std` is currently required; bare `--no-default-features` fails. `all` enables `std` through `file`/`stdin`, so CI's “no std” label is misleading.
- `all` excludes `unstable` (`ftps`, `scp`); Cargo's `--all-features` includes them. FTPS currently ignores its secure flag; SCP panics via `todo!()`.
- Percent-encoded `file:` paths currently fail to open; `FEATURES` omits `ipfs`.
- README doctests perform network I/O. Their workspace-relative `include_str!` path breaks doctests in the packaged crate.
