# Data Model

## Overview

Torpor's data model is designed around plain YAML files that are human-readable, git-friendly, and editable outside the TUI. The hierarchy is:

```
Workspace
└── Collections
    └── Requests
Environments (per workspace)
History (SQLite, not YAML)
```

## Workspace

The top-level container. One workspace per project or team. Stored as `workspace.yaml` at the root of the workspace directory.

```yaml
name: My Project
description: API workspace for My Project
default_environment: dev
settings:
  follow_redirects: true
  timeout_ms: 30000
  ssl_verify: true
  history_limit: 1000
```

## Environment

Named sets of variables scoped to a workspace. Stored in `environments/<name>.yaml`. Multiple environments can exist per workspace (dev, staging, prod).

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

Secrets use `keyring` as a placeholder value. The actual secret is stored in the system keyring under a key derived from the workspace name and variable name. This ensures environment files are safe to commit to version control.

## Collection

A named group of related requests. Stored as `collection.yaml` inside a collection directory. Collections define shared auth and headers that cascade down to requests.

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

## Request

An individual HTTP request. Stored as `<name>.yaml` inside a collection directory. This is the core data type.

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

## Inheritance Model

Auth and headers cascade from collection down to request. The request always wins on conflict.

```
Collection auth: Bearer {{token}}
Request auth:    none defined → inherits Bearer {{token}}

Collection headers: X-App-Version: 1.0
Request headers:    Content-Type: application/json
Merged headers:     X-App-Version: 1.0
                    Content-Type: application/json
```

If a request defines its own auth, the collection-level auth is ignored for that request.

## History

Request history is stored in SQLite rather than YAML. History entries are not meant to be edited or committed — they are a local audit trail. Each entry stores the full request as sent (after variable interpolation) and the full response received, with a timestamp.

## Variable Interpolation

Variables are referenced using `{{variable_name}}` syntax anywhere in a request — URL, headers, body, auth values. Interpolation is performed at execution time using the active environment's variable map. Missing variables are left as-is and logged as a warning.
