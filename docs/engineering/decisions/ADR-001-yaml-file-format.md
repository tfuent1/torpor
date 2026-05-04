# ADR-001 — YAML as the Collection File Format

**Status:** Accepted

## Context

Torpor stores workspaces, collections, environments, and requests as files on disk. A human-readable, git-friendly file format is a core design goal — collections should be commitable, diffable, and editable outside the TUI. A file format needed to be chosen before implementing the storage layer.

## Options Considered

### TOML
- Native to the Rust ecosystem (used by Cargo)
- Excellent for flat key/value configuration
- Multiline strings are awkward — request bodies stored as TOML values become difficult to read and edit
- Array of tables syntax (`[[requests]]`) is verbose for nested structures

### YAML
- Universally familiar across developer backgrounds (PHP, Python, Go, JS)
- Multiline strings are clean using block scalar syntax (`|`)
- Deeply nested structures read naturally
- Indent-sensitive — subtle bugs possible when editing manually
- `serde_yaml` handles parsing robustly

### Custom Format (Hurl-style)
- Purpose-built for HTTP requests, very clean to read
- Would require writing and maintaining a custom parser
- No existing tooling (editor syntax highlighting, linters, validators)
- Additional learning curve for new users
- Hurl's format is optimised for CLI execution, not interactive collection management

### Split Format (TOML metadata + separate body files)
- Request metadata in TOML, body stored as a separate `.json` file
- Keeps TOML clean, bodies in their native format
- Breaks the "whole request in one file" readability goal
- More files to manage per request

## Decision

YAML.

## Rationale

The multiline string story is the deciding factor. API clients store JSON request bodies constantly. YAML's block scalar syntax handles this cleanly:

```yaml
body:
  type: json
  content: |
    {
      "name": "{{user_name}}",
      "email": "{{user_email}}"
    }
```

The equivalent in TOML requires escape sequences or awkward multiline syntax that degrades readability. The split format was rejected because it breaks the single-file-per-request readability goal. A custom format was rejected because of the parser maintenance burden and lack of tooling.

TOML would be the right choice for a configuration file. For a file that primarily stores HTTP request bodies, YAML wins.

## Consequences

- `serde_yaml` is marked as deprecated by its maintainer. This is a known risk. The deprecation is primarily due to the maintainer's concerns about the `unsafe-libyaml` dependency. An alternative (`serde_yml`) exists and can be evaluated if `serde_yaml` becomes unmaintained. The migration cost would be low.
- YAML's indent sensitivity means hand-edited files can silently misbehave. Good error messages on parse failure mitigate this.
- Developers unfamiliar with YAML have a minor learning curve, but YAML is sufficiently ubiquitous that this is not a meaningful barrier.
