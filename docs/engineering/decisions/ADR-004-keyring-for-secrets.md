# ADR-004 — System Keyring for Secret Storage

**Status:** Accepted

## Context

Torpor environment files contain variable definitions including secrets such as API keys and bearer tokens. These files are designed to be committed to version control. A mechanism was needed to store secret values without including them in the YAML files.

## Options Considered

### Store secrets directly in environment YAML files
- Simple — no additional mechanism required
- Secrets are committed to version control — a critical security risk
- Rejected immediately

### Separate secrets file excluded from git (e.g. `.env`)
- Familiar pattern (used by Laravel, Docker Compose, etc.)
- Requires users to manage an additional file
- Easy to accidentally commit if `.gitignore` is misconfigured
- No encryption at rest — secrets are plaintext on disk
- No access control — any process can read the file

### System keyring via the `keyring` crate
- Secrets stored in the OS-provided secure credential store (Keychain on macOS, libsecret/KWallet on Linux, Windows Credential Manager on Windows)
- Encrypted at rest by the OS
- Integrated with OS access control
- Environment files use `keyring` as a placeholder value — safe to commit
- Cross-platform via the `keyring` crate abstraction
- Slightly more friction to set up initially (must populate keyring entries manually or via a `torpor secret set` command)

## Decision

System keyring via the `keyring` crate.

## Rationale

The environment file's git-safety is a non-negotiable design goal. Storing secrets separately in a `.env`-style file is better than inline YAML but still leaves secrets as plaintext on disk with no access control. The system keyring provides encrypted at-rest storage with OS-level access control at no additional infrastructure cost.

The `keyring` crate provides a cross-platform abstraction over the three major OS credential stores, so the implementation is consistent across developer machines.

The keyring namespace convention `torpor/<workspace_name>/<variable_name>` prevents collisions between workspaces and makes it clear which application owns a given credential.

## Consequences

- Initial setup requires populating keyring entries. A `torpor secret set <variable>` CLI subcommand should be provided to make this ergonomic.
- On headless Linux systems (CI, servers), the system keyring may not be available. This is acceptable — Torpor is a developer tool, not a CI tool. Users running Torpor in a headless environment can use the `Value` secret variant in their environment file as an escape hatch, accepting the reduced security posture.
- The `keyring` crate has platform-specific backend dependencies on Linux (libdbus or libsecret). This should be documented in the installation guide.
