use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Workspace {
    pub name: String,
    pub description: Option<String>,
    pub default_environment: Option<String>,
    pub settings: Option<WorkspaceSettings>,
    pub meta: Option<Meta>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WorkspaceSettings {
    pub follow_redirects: Option<bool>,
    pub timeout_ms: Option<u64>,
    pub ssl_verify: Option<bool>,
    pub history_limit: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Meta {
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl Workspace {
    /// Creates an unnamed, unsaved workspace for use before the user has
    /// chosen a name. Nothing is written to disk until first save.
    pub fn default_in_memory() -> Self {
        Self {
            name: String::new(),
            description: None,
            default_environment: None,
            settings: None,
            meta: None,
        }
    }
}

/// Runtime handle that pairs a loaded [`Workspace`] descriptor with its root
/// directory on disk. All path resolution for collections, environments, and
/// history goes through this type.
///
/// When no workspace file exists yet (first launch / default workspace), `path`
/// is `None` and `is_dirty` is `true`. On first `Ctrl+S` the app prompts for a
/// name, sets `path`, and writes to disk.
#[derive(Debug, Clone)]
pub struct WorkspaceHandle {
    /// The parsed workspace descriptor.
    pub workspace: Workspace,
    /// Absolute path to the `*.wksp.yaml` file, or `None` for an in-memory
    /// workspace not yet saved to disk.
    pub path: Option<PathBuf>,
    /// `true` when the in-memory state has not yet been saved to disk.
    pub is_dirty: bool,
}

impl WorkspaceHandle {
    /// Create a handle for a workspace loaded from disk.
    pub fn from_path(workspace: Workspace, path: PathBuf) -> Self {
        Self {
            workspace,
            path: Some(path),
            is_dirty: false,
        }
    }

    /// Create a handle for a brand-new in-memory workspace.
    pub fn in_memory() -> Self {
        Self {
            workspace: Workspace::default_in_memory(),
            path: None,
            is_dirty: true,
        }
    }

    /// The workspace root directory (parent of the `*.wksp.yaml` file).
    /// Returns `None` for an in-memory workspace with no path set yet.
    pub fn root(&self) -> Option<&Path> {
        self.path.as_deref()?.parent()
    }

    /// Absolute path to `collections/` under the workspace root.
    pub fn collections_dir(&self) -> Option<PathBuf> {
        Some(self.root()?.join("collections"))
    }

    /// Absolute path to `environments/` under the workspace root.
    pub fn environments_dir(&self) -> Option<PathBuf> {
        Some(self.root()?.join("environments"))
    }

    /// Absolute path to `.torpor/history.db` under the workspace root.
    pub fn history_db_path(&self) -> Option<PathBuf> {
        Some(self.root()?.join(".torpor").join("history.db"))
    }

    /// Absolute path to a specific collection descriptor.
    /// `collection_slug` is the directory name (e.g. `"users"`).
    pub fn collection_path(&self, collection_slug: &str) -> Option<PathBuf> {
        Some(
            self.collections_dir()?
                .join(collection_slug)
                .join("collection.yaml"),
        )
    }

    /// Absolute path to a specific request file.
    /// `collection_slug` is the directory name; `request_file` is the
    /// filename including extension (e.g. `"create_user.yaml"`).
    pub fn request_path(&self, collection_slug: &str, request_file: &str) -> Option<PathBuf> {
        Some(
            self.collections_dir()?
                .join(collection_slug)
                .join(request_file),
        )
    }

    /// Absolute path to a specific environment file.
    /// `env_file` is the filename including extension (e.g. `"dev.yaml"`).
    pub fn environment_path(&self, env_file: &str) -> Option<PathBuf> {
        Some(self.environments_dir()?.join(env_file))
    }
}
