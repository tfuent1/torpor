# Phase 2 — Collections & Workspaces

## Goal

Organise requests into named collections within workspaces. Enable the
git-friendly team sharing workflow where a developer can clone a repository
and immediately see all requests in the TUI.

---

## Deliverables

### Chunk 1 — File I/O Foundation

- [ ] Finalise `*.wksp.yaml` workspace descriptor load/save
- [ ] Finalise `collection.yaml` load/save
- [ ] Request files load/save scoped to workspace directory tree
- [ ] Atomic file writes (write to temp, rename) for all YAML types
- [ ] Default workspace creation in memory on first launch

### Chunk 2 — Startup & Discovery

- [ ] CLI argument parsing (`torpor [path]`)
- [ ] `find [path]` flag — unbounded scan, always shows picker
- [ ] `find` no-results prompt: `No workspaces found in {path}. Create a workspace here? (y/n)`
- [ ] Launch history check (`workspace_history` in config.toml)
- [ ] Stale `workspace_history` entries pruned on launch
- [ ] Ancestor walk — check each ancestor dir for `*.wksp.yaml` (no fan-out into siblings)
- [ ] Subdirectory scan — find all `*.wksp.yaml` within depth 5–6 of root (rooted at cwd, independent of ancestor walk)
- [ ] Startup routing:
  - Launch history hit → open recorded workspace directly
  - Ancestor walk hit → open directly
  - Subdir scan: one found → open directly
  - Subdir scan: multiple found → show workspace picker
  - Subdir scan: none found → create default workspace in memory
- [ ] Workspace name prompt on first `Ctrl+S` of a default workspace
- [ ] `workspace_history` updated on workspace open and switch

### Chunk 3 — Sidebar

- [ ] Layout split: sidebar (left) + existing request/response panes (right)
- [ ] Sidebar renders workspace tree: collections → requests
- [ ] `Tab` / `Shift+Tab` cycles focus through sidebar, request pane, response pane
- [ ] Sidebar navigation: `j`/`k` or `↑`/`↓` to move through tree items
- [ ] Selecting a request in the sidebar loads it into the request editor
- [ ] Active request highlighted in sidebar
- [ ] Sidebar toggle keybind (TBD)
- [ ] Status bar updated: shows `workspace / collection / request` breadcrumb

### Chunk 4 — Workspace Picker Overlay

- [ ] In-app workspace picker overlay (keybind TBD)
- [ ] Picker runs bounded discovery scan from current root
- [ ] Lists all found workspaces by `name` field from descriptor
- [ ] "Search from path..." option — unbounded scan from user-specified path
- [ ] Select workspace to switch; updates `workspace_history`
- [ ] Option to create a new workspace (prompts for name, creates directory structure)

### Chunk 5 — CRUD & Management

- [ ] Create new collection from TUI
- [ ] Rename collection from TUI
- [ ] Delete collection from TUI (with confirmation prompt)
- [ ] Create new request from TUI (within a selected collection)
- [ ] Rename request from TUI
- [ ] Delete request from TUI (with confirmation prompt)
- [ ] Request ordering within a collection (persisted to `order` field in `collection.yaml`)

### Chunk 6 — Inheritance

- [ ] Collection-level default headers applied to requests in that collection
- [ ] Collection-level auth applied to requests that define no auth
- [ ] Request-level auth overrides collection-level auth entirely
- [ ] Request-level headers merged with collection-level headers (request wins on conflict)

### Chunk 7 — Import (stretch; does not block milestone)

- [ ] Import Postman collection format (v2.1)
- [ ] Import Insomnia collection format (v4)

---

## Workspace File Structure

```
my-project/
  users-api/
    users-api.wksp.yaml
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

---

## Startup Flow

```
torpor [path]
    │
    ├─ path provided? → use that path as root
    │  else           → use cwd as root
    │
    ├─ root in workspace_history and recorded path still exists on disk?
    │      └─ yes → open recorded workspace directly
    │
    ├─ walk up from root, check each ancestor dir for *.wksp.yaml (no fan-out)
    │      └─ found → open that workspace directly
    │
    ├─ scan root + subdirs (depth ≤ 5–6) for *.wksp.yaml
    │      ├─ one found   → open directly
    │      ├─ multiple    → show workspace picker
    │      └─ none found  → create default workspace in memory
    │
    └─ open TUI
           └─ first Ctrl+S with default workspace → prompt for name
                  └─ create {name}/ + {name}/{name}.wksp.yaml on disk

torpor find [path]
    │
    ├─ path provided? → unbounded scan from that path
    │  else           → unbounded scan from cwd
    │
    ├─ workspaces found? → show picker (always, even if only one)
    │      └─ selection → update workspace_history, open workspace
    │
    └─ none found → "No workspaces found in {path}. Create a workspace here? (y/n)"
           ├─ y → create default workspace in memory, open TUI (normal save flow)
           └─ n → exit cleanly
```

---

## CLI Surface

| Invocation | Behaviour |
|---|---|
| `torpor` | Bounded discovery from cwd |
| `torpor ./my-project` | Bounded discovery from `./my-project` |
| `torpor find` | Unbounded scan from cwd; always shows picker |
| `torpor find ./my-project` | Unbounded scan from `./my-project`; always shows picker |
| `torpor secret set <var>` | Phase 3 — populate keyring entry (separate subcommand) |

---

## Key Design Decisions

- **`*.wksp.yaml` double-extension** identifies workspace descriptors
  unambiguously. See
  [ADR-006](../engineering/decisions/ADR-006-workspace-discovery.md).

- **Self-contained workspace directories** — everything belonging to a
  workspace lives under one named directory. Discovery is a glob; no global
  registry to maintain. Cloned workspaces are zero-config.

- **Two-phase discovery is independent** — the ancestor walk checks only
  direct directory contents with no fan-out. The subdirectory scan is always
  rooted at cwd, not at wherever the ancestor walk stopped. The two phases
  never combine into an unbounded traversal.

- **Default workspace is in-memory only** — nothing written to disk until
  first save. The name prompt on first save is the user's natural introduction
  to the workspace concept. The directory structure is always valid by
  construction.

- **`find` is explicit and unbounded** — depth-limited automatic discovery
  keeps startup fast. `find` and the in-app "Search from path..." option
  cover the use case of locating workspaces across a large directory tree.

- **`walkdir` crate** for all filesystem traversal — MIT licensed, pure Rust,
  depth-limiting built in.

- **Import is a stretch goal** — Postman and Insomnia import do not block the
  Phase 2 definition of done.

---

## Definition of Done

A developer can:

1. Run `torpor` in a project directory and have workspaces discovered automatically
2. Organise requests into named collections within a workspace
3. Create, rename, and delete collections and requests from the TUI
4. Switch between multiple workspaces using the in-app picker
5. Commit the workspace directory to git and have a teammate clone it and
   immediately see all collections and requests in their TUI
