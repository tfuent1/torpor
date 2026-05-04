# Storage

## Overview

Torpor uses two storage mechanisms:

- **YAML files** for workspaces, collections, environments, and requests — human-readable, git-friendly, editable outside the TUI
- **SQLite** for request history — local-only, not intended for version control

## YAML File Storage

### Location

Workspace directories can live anywhere on the filesystem. Torpor opens a workspace by pointing to the directory containing `workspace.yaml`. There is no global registry of workspaces — the user manages workspace locations themselves.

### File Layout

```
my-project/
  workspace.yaml
  environments/
    dev.yaml
    staging.yaml
    prod.yaml
  collections/
    users/
      collection.yaml
      create_user.yaml
      get_user.yaml
    auth/
      collection.yaml
      login.yaml
      refresh.yaml
```

### Read/Write

The `src/storage/` module provides `load` and `save` functions for each model type. All functions return `anyhow::Result` and use `serde_saphyr`/`serde_json` for serialization.

Files are written atomically where possible — write to a temp file, then rename — to avoid corrupting a collection file mid-write.

### Git Workflow

Collections and request files are designed to be committed. Environment files can be committed safely because secrets use `keyring` as a placeholder rather than storing real values. The `.gitignore` should exclude the SQLite history database.

Recommended `.gitignore` additions:
```
.torpor/
*.db
```

## SQLite History

### Location

The history database lives at `<workspace_dir>/.torpor/history.db`. It is excluded from version control.

### Schema

```sql
CREATE TABLE history (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    request_url TEXT    NOT NULL,
    method      TEXT    NOT NULL,
    request_raw TEXT    NOT NULL,  -- full request as sent, JSON encoded
    response_raw TEXT   NOT NULL,  -- full response received, JSON encoded
    status_code INTEGER NOT NULL,
    duration_ms INTEGER NOT NULL,
    created_at  TEXT    NOT NULL   -- ISO 8601 timestamp
);
```

### Retention

The workspace settings `history_limit` field controls how many history entries are retained. When the limit is exceeded, the oldest entries are pruned. The default limit is 1000.

## Secret Storage

Secrets are stored in the system keyring via the `keyring` crate. The keyring key is derived from the workspace name and variable name:

```
torpor/<workspace_name>/<variable_name>
```

For example, a workspace named `my-project` with a secret variable `token` would be stored under:

```
torpor/my-project/token
```

This namespacing prevents collisions between secrets from different workspaces.
