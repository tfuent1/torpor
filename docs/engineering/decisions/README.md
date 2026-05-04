# Architecture Decision Records

This directory contains Architecture Decision Records (ADRs) for Torpor. An ADR documents a significant technical decision, the context that led to it, the options considered, and the rationale for the choice made.

## Index

| ADR | Title | Status |
|-----|-------|--------|
| [ADR-001](ADR-001-yaml-file-format.md) | YAML as the collection file format | Accepted |
| [ADR-002](ADR-002-rustls-over-openssl.md) | rustls over openssl for TLS | Accepted |
| [ADR-003](ADR-003-sqlite-for-history.md) | SQLite for request history | Accepted |
| [ADR-004](ADR-004-keyring-for-secrets.md) | System keyring for secret storage | Accepted |

## ADR Format

Each ADR follows this structure:

- **Status**: Accepted / Deprecated / Superseded
- **Context**: The situation that required a decision
- **Options Considered**: What alternatives were evaluated
- **Decision**: What was chosen
- **Rationale**: Why this option was chosen over the others
- **Consequences**: Trade-offs and implications of the decision
