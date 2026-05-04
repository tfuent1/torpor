# Testing Strategy

## Overview

Torpor uses a two-tier testing strategy: unit tests colocated with the code they test, and integration tests in the `tests/` directory.

## Unit Tests

Unit tests live in a `#[cfg(test)]` module at the bottom of each source file. They test individual functions and types in isolation.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variable_interpolation() {
        // ...
    }
}
```

Unit tests can access private functions via `use super::*`. They should be fast and have no external dependencies (no filesystem, no network, no database).

### What to Unit Test
- Model serialization/deserialization round trips
- Variable interpolation logic
- Auth and header merge/inheritance logic
- Assertion evaluation
- JSONPath extraction

## Integration Tests

Integration tests live in `tests/` at the project root. They test larger pieces of behaviour from the outside, as a consumer of the public API would.

```
tests/
  storage_roundtrip.rs
  request_execution.rs
  environment_interpolation.rs
```

Integration tests may use the filesystem (via `tempfile` for temp directories) but must not make real network requests. HTTP interactions should be tested against a mock server (consider `wiremock` or `httpmock`).

### What to Integration Test
- Full YAML round-trip: write a request to disk, read it back, verify equality
- Full request execution pipeline against a mock server
- Collection runner: multiple requests in sequence with chaining
- Environment switching mid-session

## What Not to Test

- The TUI rendering layer — Ratatui components are difficult to unit test and the value is low. Focus testing on the business logic that feeds the TUI.
- Third-party library behaviour — do not test that `serde_yaml` serializes correctly. Test that your own types serialize to the expected YAML structure.

## Coverage

Coverage is measured via `cargo-llvm-cov` and reported in CI as an HTML artifact. Coverage is informational — there is no enforced minimum threshold. The goal is to have meaningful coverage on the storage layer, interpolation logic, and request execution pipeline.

## Test Helpers

Common test utilities (building fixture requests, creating temp workspace directories) should live in `src/test_utils.rs` or similar, gated behind `#[cfg(test)]` so they are excluded from the release binary.
