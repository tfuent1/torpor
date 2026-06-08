//! Workspace discovery — ancestor walk, subdirectory scan, and launch history.

use crate::config::Config;
use crate::models::workspace::WorkspaceHandle;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

const SUBDIR_SCAN_DEPTH: usize = 6;

// ---------------------------------------------------------------------------
// Public entry points
// ---------------------------------------------------------------------------

/// Result of the bounded discovery algorithm (normal `torpor [path]` launch).
pub enum DiscoveryResult {
    /// A single workspace was found and loaded. Open it.
    Found(WorkspaceHandle),
    /// Multiple workspaces found — caller should show the picker.
    Multiple(Vec<PathBuf>),
    /// Nothing found — use a default in-memory workspace.
    None,
}

/// Bounded discovery from a root path. Checks launch history, then ancestor
/// walk, then subdirectory scan. Does not print anything or do terminal I/O.
pub fn discover(root: &Path, config: &Config) -> DiscoveryResult {
    let Ok(root) = root.canonicalize() else {
        return DiscoveryResult::None;
    };

    // Step 1 — launch history
    let root_key = root.to_string_lossy().to_string();
    if let Some(recorded) = config.workspace_history.get(&root_key) {
        let recorded_path = PathBuf::from(recorded);

        if recorded_path.exists()
            && let Ok(handle) = load_handle(&recorded_path)
        {
            return DiscoveryResult::Found(handle);
        }
        // Stale entry — will be pruned by caller after this returns
    }

    // Step 2 — ancestor walk (no fan-out into siblings)
    let mut dir = root.as_path();
    loop {
        if let Some(found) = find_wksp_in_dir(dir)
            && let Ok(handle) = load_handle(&found)
        {
            return DiscoveryResult::Found(handle);
        }
        match dir.parent() {
            Some(p) => dir = p,
            None => break,
        }
    }

    // Step 3 — subdirectory scan (rooted at original root, depth-limited)
    let found = scan_subdir(&root, SUBDIR_SCAN_DEPTH);
    match found.len() {
        0 => DiscoveryResult::None,
        1 => {
            if let Ok(handle) = load_handle(&found[0]) {
                DiscoveryResult::Found(handle)
            } else {
                DiscoveryResult::None
            }
        }
        _ => DiscoveryResult::Multiple(found),
    }
}

/// Unbounded scan from `root`. Always returns all found paths (even one).
/// Used by `torpor find`. Does not do terminal I/O.
pub fn scan_unbounded(root: &Path) -> Vec<PathBuf> {
    WalkDir::new(root)
        .follow_links(false)
        .into_iter()
        .filter_map(std::result::Result::ok)
        .filter(|e| is_wksp_file(e.path()))
        .map(|e| e.path().to_path_buf())
        .collect()
}

/// Interactive `torpor find` flow. Runs pre-TUI with normal terminal I/O.
/// Returns a `WorkspaceHandle` to open, or `None` if the user chose to exit.
///
/// Prints to stdout; expects to run before raw mode is entered.
pub fn run_find(root: &Path, config: &mut Config) -> anyhow::Result<Option<WorkspaceHandle>> {
    let found = scan_unbounded(root);

    if found.is_empty() {
        print!(
            "No workspaces found in {}. Create a workspace here? (y/n) ",
            root.display()
        );
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if input.trim().eq_ignore_ascii_case("y") {
            return Ok(Some(WorkspaceHandle::in_memory()));
        }
        return Ok(None);
    }

    let selection = pick_workspace(&found)?;
    let Some(path) = selection else {
        return Ok(None);
    };

    let handle = load_handle(&path)?;
    record_history(config, root, &path);
    Ok(Some(handle))
}

/// Show a numbered picker for the found workspace paths. Returns the chosen
/// path, or `None` if the user aborted with `0` or empty input.
pub fn pick_workspace(paths: &[PathBuf]) -> anyhow::Result<Option<PathBuf>> {
    println!("Found {} workspace(s):\n", paths.len());
    for (i, path) in paths.iter().enumerate() {
        // Show the name from the descriptor if loadable, else show the path
        let label = load_name(path).unwrap_or_else(|| path.display().to_string());
        println!("  {}. {}", i + 1, label);
    }
    println!();
    print!("Select workspace (1-{}, or 0 to cancel): ", paths.len());
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let trimmed = input.trim();

    if trimmed == "0" || trimmed.is_empty() {
        return Ok(None);
    }

    match trimmed.parse::<usize>() {
        Ok(n) if n >= 1 && n <= paths.len() => Ok(Some(paths[n - 1].clone())),
        _ => {
            println!("Invalid selection.");
            Ok(None)
        }
    }
}

/// Update `workspace_history` for `root` to point at `wksp_path`.
/// Caller is responsible for saving the config afterwards.
pub fn record_history(config: &mut Config, root: &Path, wksp_path: &Path) {
    let key = root.to_string_lossy().to_string();
    let value = wksp_path.to_string_lossy().to_string();
    config.workspace_history.insert(key, value);
}

/// Remove stale entries from `workspace_history` (paths that no longer exist).
/// Caller is responsible for saving the config afterwards if this returns `true`.
pub fn prune_history(config: &mut Config) -> bool {
    let before = config.workspace_history.len();
    config
        .workspace_history
        .retain(|_, v| PathBuf::from(v.as_str()).exists());
    config.workspace_history.len() < before
}

// ---------------------------------------------------------------------------
// Private helpers
// ---------------------------------------------------------------------------

/// Check `dir` for exactly one `*.wksp.yaml` file (non-recursive).
fn find_wksp_in_dir(dir: &Path) -> Option<PathBuf> {
    let entries = std::fs::read_dir(dir).ok()?;
    let mut found: Option<PathBuf> = None;
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_file() && is_wksp_file(&path) {
            if found.is_some() {
                // Multiple in same dir — don't auto-pick, fall through to picker
                return None;
            }
            found = Some(path);
        }
    }
    found
}

/// Depth-limited subdirectory scan. Returns all `*.wksp.yaml` paths found.
fn scan_subdir(root: &Path, max_depth: usize) -> Vec<PathBuf> {
    WalkDir::new(root)
        .max_depth(max_depth)
        .follow_links(false)
        .into_iter()
        .filter_map(std::result::Result::ok)
        .filter(|e| is_wksp_file(e.path()))
        .map(|e| e.path().to_path_buf())
        .collect()
}

fn is_wksp_file(path: &Path) -> bool {
    path.is_file()
        && path
            .file_name()
            .and_then(|n| n.to_str())
            .is_some_and(|n| n.ends_with(".wksp.yaml"))
}

fn load_handle(path: &Path) -> anyhow::Result<WorkspaceHandle> {
    let workspace = crate::storage::workspace::load(path)?;
    Ok(WorkspaceHandle::from_path(workspace, path.to_path_buf()))
}

/// Load just the `name` field from a workspace descriptor for display purposes.
/// Returns `None` on any error rather than propagating.
fn load_name(path: &Path) -> Option<String> {
    let ws = crate::storage::workspace::load(path).ok()?;
    Some(ws.name).filter(|n| !n.is_empty())
}

/// Public wrapper around the depth-limited subdirectory scan.
/// Used by the in-app workspace picker to populate its list.
pub fn scan_subdir_pub(root: &Path) -> Vec<PathBuf> {
    scan_subdir(root, SUBDIR_SCAN_DEPTH)
}
