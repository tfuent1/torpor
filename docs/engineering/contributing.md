# Contributing

## Prerequisites

- Rust stable toolchain (`rustup install stable`)
- `cargo-deny` (`cargo install cargo-deny`)
- `cargo-llvm-cov` for coverage (`cargo install cargo-llvm-cov`)
- On Linux: `libssl-dev` is not required (we use rustls) but `pkg-config` should be installed
- On Linux: `libdbus-1-dev` or `libsecret-1-dev` may be required for the keyring crate depending on your desktop environment

## Setup

```bash
git clone git@github.com:tfuent1/torpor.git
cd torpor
cargo build
```

## Running Locally

```bash
cargo run
```

## Running Tests

```bash
cargo test --all
```

## Code Standards

### Formatting
All code must be formatted with `rustfmt`. Run before committing:
```bash
cargo fmt --all
```

### Lints
All clippy warnings must be resolved. Run:
```bash
cargo clippy --all-targets --all-features
```

### Doc Comments
All public structs, enums, functions, and modules must have `///` doc comments. This is enforced by convention, not by a lint, but PRs without doc comments on public items will not be merged.

### Error Handling
- Use `anyhow::Result` for application-layer error propagation
- Never use `unwrap()` or `expect()` in non-test code
- Use `?` for propagation, explicit `match` or `if let` when recovery is needed

### Tests
- Unit tests live in a `#[cfg(test)]` module at the bottom of the file they test
- Integration tests live in `tests/` at the project root
- Tests must not touch the filesystem without using a temp directory
- Tests must not make real network requests

### No Unsafe Code
Torpor itself contains no `unsafe` code. Do not introduce `unsafe` blocks. Unsafe in dependencies is unavoidable (tokio, libc, etc.) and is tracked by `cargo geiger` in CI.

## Making a Pull Request

1. Branch from `main`
2. Make your changes
3. Run `cargo fmt --all`, `cargo clippy --all-targets --all-features`, and `cargo test --all`
4. Run `cargo deny check`
5. Open a PR against `main`

CI must be green before a PR can be merged.

## Architecture Decisions

Significant technical decisions should be documented as ADRs in `docs/engineering/decisions/`. See the existing ADRs for the format. If your PR makes a decision that future maintainers might question, write an ADR.
