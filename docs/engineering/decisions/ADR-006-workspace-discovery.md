# ADR-006 — Workspace Discovery and Startup Flow

**Status:** Accepted

## Context

Phase 2 introduces workspaces as the top-level organisational unit in Torpor.
A workspace is a named, self-contained directory containing a `.wksp.yaml`
descriptor file, a `collections/` subtree, and an `environments/` subtree.

Torpor needed a model for:

1. How workspaces are identified on disk
2. How Torpor finds workspaces when launched
3. What happens when no workspace exists yet
4. How the user switches between workspaces at runtime
5. How the CLI surface supports power-user discovery workflows

## Options Considered

### Global registry in config

Store a list of known workspace paths in `~/.config/torpor/config.toml`.
Torpor reads this list on startup and presents a picker.

**Rejected.** This is a maintenance burden — paths go stale when directories
are moved or deleted, and the list must be kept in sync manually. It also means
Torpor has no knowledge of workspaces that were created outside of Torpor (e.g.
cloned from a git repository). A developer who clones a repo containing a
workspace would need to explicitly register it before Torpor could see it.

### Filesystem discovery

Torpor discovers workspaces by scanning the filesystem at launch — walking up
the directory tree from cwd (ancestor search), then scanning cwd and
subdirectories to a bounded depth. No global registry required. Workspaces are
identified by the presence of a `*.wksp.yaml` file.

**Accepted.** Discovery is reliable regardless of how the workspace got there —
cloned, created in Torpor, created by a teammate, moved on disk. It requires no
registration step and degrades gracefully (if nothing is found, a default
workspace is created in memory).

## Decision

### Workspace identification

A workspace is identified by the presence of a `*.wksp.yaml` file. The
double-extension convention makes discovery unambiguous via a simple glob and
avoids conflicts with general-purpose YAML files in the same directory tree.

The `name` field inside the `.wksp.yaml` file is the human-readable workspace
name shown in the TUI. The filename and directory name are not required to match
the `name` field, but by convention Torpor creates them to match on first save.

### Directory structure

Each workspace is fully self-contained inside a named directory. The parent
directory of the `*.wksp.yaml` file is the workspace root. All paths within
the workspace (collections, environments, history) are resolved relative to
this root. A single parent directory may contain multiple independent workspaces.

```
my-project/
  users-api/
    users-api.wksp.yaml        ← workspace root is users-api/
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
    payments-api.wksp.yaml     ← workspace root is payments-api/
    environments/
      dev.yaml
    collections/
      charges/
        collection.yaml
        create_charge.yaml
```

### Discovery algorithm

On launch, Torpor performs discovery in this order:

**Step 1 — Check launch history**

If `~/.config/torpor/config.toml` contains a `workspace_history` entry for the
resolved root directory (cwd or CLI-provided path) and that recorded workspace
file still exists on disk, open it directly. Skip steps 2 and 3.

If the recorded path no longer exists, prune the entry and continue to step 2.

**Step 2 — Ancestor walk (git-style)**

Starting at cwd, check each ancestor directory for any `*.wksp.yaml` file —
one directory at a time, no descending into siblings. Stop at the filesystem
root. If found at any level, open that workspace directly. Skip step 3.

This allows `torpor` to be run from any subdirectory within a workspace (e.g.
`~/projects/my-api/src/`) and correctly locate the workspace rooted above it.

**Step 3 — Subdirectory scan**

If no ancestor workspace was found, scan cwd and its subdirectories for
`*.wksp.yaml` files up to a maximum depth of 5–6 levels. The scan is rooted at
cwd regardless of how far the ancestor walk climbed — the two phases are
independent.

- **Zero found** → create a default workspace in memory (see below)
- **One found** → open it directly
- **Multiple found** → open the workspace picker showing all results

### Default workspace behaviour

When no workspace is found, Torpor creates a default workspace in memory only.
Nothing is written to disk. The TUI opens immediately and the user can work
normally. On first save (`Ctrl+S`), the user is prompted for a workspace name.
Torpor then:

1. Creates `{name}/` under the resolved root directory
2. Writes `{name}/{name}.wksp.yaml`
3. Writes the request file into `{name}/collections/...`

The workspace directory structure is always valid by construction. There is no
code path that produces a partially-initialised workspace on disk.

### Launch history

`~/.config/torpor/config.toml` stores one entry per root directory, mapping
the directory to the last opened workspace file path. This allows Torpor to
skip discovery when relaunched from a familiar directory:

```toml
[workspace_history]
"/home/tommy/projects/my-api" = "/home/tommy/projects/my-api/users-api/users-api.wksp.yaml"
"/home/tommy/projects/other"  = "/home/tommy/projects/other/payments-api/payments-api.wksp.yaml"
```

The key is the resolved root directory; the value is the absolute path to the
specific workspace file last opened from that directory. On workspace switch,
the entry for the current root is updated.

### CLI surface

```
torpor                          Normal launch from cwd
torpor ./my-project             Normal launch, target path instead of cwd
torpor find                   Unbounded scan from cwd; always shows picker
torpor find ./my-project      Unbounded scan from target path; always shows picker
torpor secret set <variable>    Phase 3 — populate keyring entry (separate subcommand)
```

`find` bypasses the launch history check and the depth limit. It always
presents the workspace picker regardless of how many workspaces are found,
since the intent is explicitly to browse. Discovered workspace paths are
persisted to `workspace_history` on selection so subsequent normal launches
are instant.

If `find` produces zero results:

```
No workspaces found in {path}. Create a workspace here? (y/n)
```

`y` creates a default workspace in memory and opens the TUI with the normal
first-save name prompt. `n` exits cleanly.

### In-app workspace picker

A workspace picker overlay is available at runtime via a keybind (decided
during Phase 2 implementation). It re-runs the bounded discovery scan from the
current root directory and displays all found workspaces by their `name` field.
A "Search from path..." option triggers an unbounded scan from a user-specified
path, equivalent to `find`. Switching workspaces updates `workspace_history`.

## Consequences

- **Depth limit** — the bounded subdirectory scan is capped at 5–6 levels.
  Workspaces nested deeper will not be found automatically. Users can reach
  them via `torpor find` or the in-app "Search from path..." option.

- **`*.wksp.yaml` is a public contract** — the double-extension convention is
  part of the on-disk format. Changing it in a future version would require a
  migration path.

- **Cloned workspaces are zero-config** — a teammate who clones a repository
  containing a workspace runs `torpor` and is in. No registration step.

- **Ancestor walk is O(depth × files-per-dir)** — practically instantaneous.
  It does not fan out into sibling directories at any level.

- **Stale history entries are pruned silently** — if a recorded workspace path
  no longer exists, the entry is removed and discovery proceeds normally. The
  user is not interrupted.

- **Workspace rename caveat** — renaming a workspace updates the `name` field
  in the `.wksp.yaml` and renames the directory and file, but keyring entries
  (Phase 3) are keyed by workspace name and are not automatically migrated.
  This is a known limitation to be documented in the rename flow.

- **`walkdir` crate** — subdirectory scanning uses `walkdir` for depth-limited
  filesystem traversal. It is MIT licensed, pure Rust, and compatible with
  `deny.toml` constraints.
