# CI/CD

## Overview

Torpor uses GitHub Actions for CI. The pipeline runs on every push to `main` and on every pull request targeting `main`.

## Jobs

### ci — Check, Lint, Test

Runs on `ubuntu-latest`.

| Step | Command | Purpose |
|------|---------|---------|
| Format check | `cargo fmt --all -- --check` | Enforces consistent formatting |
| Clippy | `cargo clippy --all-targets --all-features` | Lints and catches common mistakes |
| Tests | `cargo test --all` | Runs all unit and integration tests |
| Deny check | `cargo-deny-action` | Checks licenses, CVEs, banned crates, duplicate versions |
| Unsafe report | `cargo geiger` | Reports unsafe code usage (informational, non-blocking) |

### coverage — Coverage Report

Runs in parallel with `ci` on `ubuntu-latest`.

| Step | Command | Purpose |
|------|---------|---------|
| Generate report | `cargo llvm-cov --package torpor --html` | Produces HTML coverage report |
| Print summary | `cargo llvm-cov --package torpor` | Prints coverage percentage to log |
| Upload artifact | `actions/upload-artifact` | Stores HTML report for 14 days |

## Environment Variables

| Variable | Value | Purpose |
|----------|-------|---------|
| `CARGO_TERM_COLOR` | `always` | Coloured cargo output in CI logs |
| `RUSTFLAGS` | `-D warnings` | Treat all compiler warnings as errors |
| `CARGO_INCREMENTAL` | `0` | Disable incremental compilation in CI |

## Caching

Cargo registry and build artifacts are cached using `actions/cache` keyed on the `Cargo.lock` hash. This significantly reduces CI run time after the first build.

## cargo-deny Configuration

See `deny.toml` at the repository root. Key policies:

- **Licenses:** Only permissive licenses allowed (MIT, Apache-2.0, ISC, Zlib, BSD-3-Clause, Unicode-3.0, CDLA-Permissive-2.0)
- **Bans:** `openssl` and `native-tls` are banned (see ADR-002)
- **Advisories:** Yanked crates are denied; unmaintained workspace crates are denied
- **Duplicates:** Multiple versions of the same crate produce warnings

## Lints

Lints are configured in `Cargo.toml` rather than in source files to apply consistently across the entire codebase:

```toml
[lints.rust]
unused_imports = "warn"
dead_code = "warn"

[lints.clippy]
all = { level = "warn", priority = -1 }
pedantic = { level = "warn", priority = -1 }
module_name_repetitions = "allow"
must_use_candidate = "allow"
```

Combined with `RUSTFLAGS=-D warnings` in CI, all warnings are treated as errors in the pipeline.
