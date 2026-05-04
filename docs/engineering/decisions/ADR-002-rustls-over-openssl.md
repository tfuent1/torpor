# ADR-002 — rustls over OpenSSL for TLS

**Status:** Accepted

## Context

Torpor makes HTTPS requests via reqwest. reqwest supports multiple TLS backends: the system's native TLS (which uses OpenSSL on Linux), or rustls, a pure-Rust TLS implementation. A backend needed to be chosen.

## Options Considered

### native-tls / OpenSSL
- Uses the system's OpenSSL installation on Linux
- Broadly compatible and battle-tested
- Introduces a dependency on a C library
- Requires `libssl-dev` to be installed on the build machine
- Historically a source of CVEs
- Complicates cross-compilation and static linking
- Means the binary is not fully self-contained

### rustls
- Pure Rust implementation — no C dependencies
- Memory-safe by construction
- Simplifies cross-compilation
- Enables fully static binaries
- Slightly less compatible with non-standard TLS configurations (e.g. some legacy cipher suites)
- Actively maintained and widely used in the Rust ecosystem

## Decision

rustls, configured via `reqwest`'s `rustls-tls` feature flag with `default-features = false`.

## Rationale

A single compiled binary with no system dependencies is a core goal for Torpor. OpenSSL as a system dependency undermines this — users on minimal systems may not have `libssl` installed, and cross-compilation becomes significantly more complex. rustls produces a fully self-contained binary and eliminates a class of C memory safety vulnerabilities.

The compatibility trade-off (legacy cipher suites) is not a concern for Torpor's use case. Modern APIs use modern TLS. The small number of developers who need to test against legacy TLS configurations can use curl directly.

## Consequences

- `openssl` and `native-tls` are banned in `deny.toml` to prevent them from being re-introduced as transitive dependencies.
- Developers targeting APIs that require legacy cipher suites or client certificates with non-standard configurations may encounter compatibility issues. This is an accepted limitation.
- The build machine does not need OpenSSL development headers installed.
