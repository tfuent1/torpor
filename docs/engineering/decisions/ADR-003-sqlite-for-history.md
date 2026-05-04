# ADR-003 — SQLite for Request History

**Status:** Accepted

## Context

Torpor needs to store a history of sent requests and received responses for the current workspace. History entries need to be queryable (filter by URL, method, status code, date range) and bounded (oldest entries pruned when a limit is reached). A storage mechanism needed to be chosen.

## Options Considered

### YAML files (one file per history entry)
- Consistent with the rest of Torpor's file-based storage
- Simple to implement
- Poor query performance as history grows
- Pruning by age/limit requires listing, sorting, and deleting files
- Directory with thousands of small files is unwieldy

### Single YAML file (append entries)
- Simple
- Entire file must be loaded into memory to query or prune
- Poor performance at scale
- Not suitable for concurrent writes (unlikely but still undesirable)

### SQLite via sqlx
- Embedded, no separate server process
- Excellent query performance
- Simple schema migrations
- sqlx provides compile-time checked queries and async support
- Single `.db` file per workspace — easy to exclude from git
- Pruning is a simple `DELETE WHERE id < (SELECT id FROM history ORDER BY id DESC LIMIT 1 OFFSET $limit)`

## Decision

SQLite via sqlx with the `runtime-tokio` and `sqlite` features.

## Rationale

History is fundamentally a queryable, append-heavy dataset with a retention policy. This is exactly what a database is for. SQLite is the right tool — it is embedded, requires no server, and produces a single file. YAML files would require reimplementing query and pruning logic that SQLite provides natively.

The history database is explicitly local-only and excluded from version control. It does not share the git-friendly portability requirement of collection files, so there is no reason to use YAML.

## Consequences

- The history database lives at `<workspace_dir>/.torpor/history.db` and is excluded from git via `.gitignore`.
- sqlx requires database migrations. Migration files live in `migrations/` and are embedded into the binary at compile time via sqlx's `migrate!` macro.
- History is not portable between machines — it is a local audit trail only. This is intentional.
- The `sqlx` dependency adds compile time and binary size. This is an accepted trade-off.
