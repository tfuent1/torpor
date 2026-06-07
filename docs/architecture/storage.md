# Storage

## Overview

Torpor uses two storage mechanisms:

- **YAML files** for workspaces, collections, environments, and requests —
  human-readable, git-friendly, and editable outside the TUI
- **SQLite** for request history — local-only, not intended for version control

---

## YAML File Storage

### Workspace Directory Structure

Each workspace is a self-contained named directory. The workspace descriptor
uses the `.wksp.yaml` double-extension so it can be identified unambiguously
by the discovery algorithm without conflicting with other YAML files in the
same tree.

```
my-project/
  users-api/
    users-api.wksp.yaml        ← workspace descriptor; parent dir is workspace root
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
  payments-api/
    payments-api.wksp.yaml
    environments/
      dev.yaml
    collections/
      charges/
        collection.yaml
        create_charge.yaml
```

A single parent directory may contain multiple workspaces. Each is fully
independent — its collections, environments, and history are scoped entirely
to its own directory tree.

The `name` field inside the `.wksp.yaml` is the human-readable name shown in
the TUI. The filename and directory name are not required to match the `name`
field, but Torpor creates them to match by convention on first save.

---

### Workspace Descriptor (`{name}.wksp.yaml`)

```yaml
name: Users API
description: API workspace for the users service
default_environment: dev
settings:
  follow_redirects: true
  timeout_ms: 30000
  ssl_verify: true
  history_limit: 1000
meta:
  created_at: "2025-06-07T14:00:00Z"
  updated_at: "2025-06-07T14:00:00Z"
```

---

### Collection (`collections/{name}/collection.yaml`)

```yaml
name: Users
description: User management endpoints
auth:
  type: bearer
  token: "{{token}}"
headers:
  X-App-Version: "1.0"
order:
  - create_user.yaml
  - get_user.yaml
  - update_user.yaml
  - delete_user.yaml
```

---

### Request (`collections/{collection}/{name}.yaml`)

```yaml
name: Create User
description: Creates a new user
method: POST
url: "{{base_url}}/api/users"
headers:
  Content-Type: application/json
body:
  type: json
  content: |
    {
      "name": "{{user_name}}",
      "email": "{{user_email}}"
    }
assertions:
  - status: 201
  - json: "$.data.id"
    exists: true
extract:
  - name: user_id
    json: "$.data.id"
```

---

### Environment (`environments/{name}.yaml`)

```yaml
name: Development
color: green
variables:
  base_url: http://localhost:8000
  user_email: test@example.com
secrets:
  token: keyring
  api_key: keyring
```

Secrets use `keyring` as a placeholder value. The actual secret is stored in
the system keyring under a key derived from the workspace name and variable
name (see Secret Storage below). Environment files are safe to commit — no
real credentials are ever stored in them.

---

### Read/Write

The `src/storage/` module provides `load` and `save` functions for each model
type. All functions return `anyhow::Result` and use `serde-saphyr` for
serialization.

Files are written atomically — written to a temp file then renamed — to avoid
corrupting a workspace file mid-write.

### Git Workflow

All workspace files are designed to be committed to version control. The
`.torpor/` directory (SQLite history database) should be excluded.

Recommended `.gitignore` additions:

```
.torpor/
*.db
```

---

## Workspace Discovery

Torpor finds workspaces by scanning the filesystem rather than maintaining a
global registry. See
[ADR-006](../engineering/decisions/ADR-006-workspace-discovery.md) for the
full rationale and algorithm.

**Summary of startup flow:**

```
torpor [path]
    │
    ├─ resolved root in workspace_history and path still exists?
    │      └─ yes → open recorded workspace directly
    │
    ├─ walk up from root, check each ancestor dir for *.wksp.yaml (no fan-out)
    │      └─ found → open directly
    │
    ├─ scan root + subdirs (depth ≤ 5–6) for *.wksp.yaml
    │      ├─ one found   → open directly
    │      ├─ multiple    → show workspace picker
    │      └─ none found  → create default workspace in memory
    │
    └─ open TUI
           └─ first Ctrl+S with default workspace → prompt for name
                  └─ create {name}/ + {name}/{name}.wksp.yaml on disk
```

**`torpor find [path]`** bypasses the depth limit and launch history, always
shows the picker, and persists discovered paths to `workspace_history` on
selection. If no workspaces are found, prompts to create one.

### Launch History

`~/.config/torpor/config.toml` stores one entry per root directory, mapping
the directory path to the last opened workspace file path:

```toml
[workspace_history]
"/home/tommy/projects/my-api" = "/home/tommy/projects/my-api/users-api/users-api.wksp.yaml"
"/home/tommy/projects/other"  = "/home/tommy/projects/other/payments-api/payments-api.wksp.yaml"
```

Stale entries (workspace file no longer exists on disk) are pruned silently
on launch. On workspace switch the entry for the current root is updated.

---

## SQLite History

### Location

The history database lives at `<workspace_root>/.torpor/history.db`. It is
scoped per workspace and excluded from version control.

### Schema

```sql
CREATE TABLE history (
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    request_url  TEXT    NOT NULL,
    method       TEXT    NOT NULL,
    request_raw  TEXT    NOT NULL,  -- full request as sent, JSON encoded
    response_raw TEXT    NOT NULL,  -- full response received, JSON encoded
    status_code  INTEGER NOT NULL,
    duration_ms  INTEGER NOT NULL,
    created_at   TEXT    NOT NULL   -- ISO 8601 timestamp
);
```

### Retention

The workspace descriptor's `settings.history_limit` field controls how many
history entries are retained. When the limit is exceeded, the oldest entries
are pruned. The default limit is 1000.

---

## Secret Storage

Secrets are stored in the system keyring via the `keyring` crate. The keyring
key is derived from the workspace name and variable name:

```
torpor/<workspace_name>/<variable_name>
```

For example, a workspace named `users-api` with a secret variable `token` is
stored under:

```
torpor/users-api/token
```

This namespacing prevents collisions between workspaces. When a workspace is
renamed, keyring entries are not automatically migrated — this is a known
limitation documented in the workspace rename flow.
