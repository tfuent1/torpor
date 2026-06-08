use crate::models::collection::Collection;
use crate::models::request::HttpMethod;
use crate::models::request::Request;
use crate::models::workspace::WorkspaceHandle;
use tokio::sync::mpsc;

/// The result type sent back through the async channel after a request completes.
pub type RequestResult = anyhow::Result<ResponseState>;

/// All state captured from a completed HTTP response.
#[derive(Debug, Clone)]
pub struct ResponseState {
    pub status: u16,
    pub duration_ms: u64,
    pub headers: Vec<(String, String)>,
    pub body: String,
    pub size_bytes: usize,
}

/// Which pane currently has keyboard focus.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Focus {
    RequestPane,
    ResponsePane,
    Sidebar,
    UrlBar,
}

/// Which tab is active in the request editor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RequestTab {
    Headers,
    Body,
}

/// Which field is being edited in the headers editor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HeaderField {
    Key,
    Value,
}

/// A collection loaded into memory, paired with its on-disk slug and requests.
#[derive(Debug, Clone)]
pub struct LoadedCollection {
    /// Directory name under `collections/`, e.g. `"users"`.
    pub slug: String,
    /// Parsed collection descriptor.
    pub collection: Collection,
    /// Requests in display order: `(filename_without_ext, Request)`.
    pub requests: Vec<(String, Request)>,
}

/// Tracks what operation is waiting for user input from the sidebar prompt.
#[derive(Debug, Clone)]
pub enum PendingAction {
    NewCollection,
    NewRequest(usize),           // col_idx
    RenameCollection(usize),     // col_idx
    RenameRequest(usize, usize), // col_idx, req_idx
}

/// Tracks what deletion is waiting for confirmation.
#[derive(Debug, Clone)]
pub enum PendingDelete {
    Collection(usize),
    Request(usize, usize), // col_idx, req_idx
}

/// Central application state. The TUI reads from this; all mutations go through it.
#[allow(clippy::struct_excessive_bools)]
pub struct AppState {
    // Request being built
    pub method: HttpMethod,
    pub url: String,
    pub headers: Vec<(String, String)>,
    /// Body stored as lines for cursor-aware editing.
    pub body_lines: Vec<String>,
    pub body_cursor_row: usize,
    pub body_cursor_col: usize,

    // Last completed response
    pub response: Option<ResponseState>,

    // Async channel for receiving request results from the engine
    pub response_tx: mpsc::Sender<RequestResult>,
    pub response_rx: mpsc::Receiver<RequestResult>,

    // UI state
    pub focus: Focus,
    pub active_tab: RequestTab,
    pub status_message: Option<String>,
    pub request_in_flight: bool,
    pub cursor_pos: usize, // URL bar cursor
    pub response_scroll: u16,
    pub theme_selector_open: bool,
    pub theme_selector_index: usize,

    /// Whether the sidebar is visible.
    pub sidebar_open: bool,
    /// Which tree item is highlighted in the sidebar (flat index).
    pub sidebar_selected: usize,

    /// Whether the workspace picker overlay is open.
    pub workspace_picker_open: bool,
    /// Workspaces found by the last picker scan (paths to *.wksp.yaml).
    pub workspace_picker_items: Vec<std::path::PathBuf>,
    /// Which item is highlighted in the workspace picker.
    pub workspace_picker_selected: usize,

    // Headers editor state
    pub header_selected: usize,
    pub header_editing: Option<HeaderField>,
    pub header_edit_buf: String,

    /// The active workspace. Starts as an in-memory default if no workspace
    /// was found on launch; path is set on first save.
    pub workspace: WorkspaceHandle,

    /// Collections loaded from the active workspace.
    pub loaded_collections: Vec<LoadedCollection>,
    /// Index into `loaded_collections` for the highlighted/active collection.
    pub active_collection: Option<usize>,
    /// Index into the active collection's `requests` for the highlighted request.
    pub active_request: Option<usize>,

    /// When `Some`, a single-line input prompt is open (rename / new name).
    /// Contains `(prompt_label, current_input)`.
    pub input_prompt: Option<(String, String)>,
    /// When `Some`, a yes/no confirmation prompt is open.
    pub confirm_prompt: Option<String>,
    /// Pending sidebar action waiting for input prompt completion.
    pub pending_action: Option<PendingAction>,
    /// Pending deletion waiting for confirmation.
    pub pending_delete: Option<PendingDelete>,
}

impl AppState {
    /// Creates a new `AppState` with sensible defaults.
    pub fn new(workspace: WorkspaceHandle) -> Self {
        let (response_tx, response_rx) = mpsc::channel(1);
        Self {
            method: HttpMethod::Get,
            url: String::new(),
            headers: Vec::new(),
            body_lines: vec![String::new()],
            body_cursor_row: 0,
            body_cursor_col: 0,
            response: None,
            response_tx,
            response_rx,
            focus: Focus::UrlBar,
            active_tab: RequestTab::Body,
            status_message: None,
            request_in_flight: false,
            cursor_pos: 0,
            response_scroll: 0,
            theme_selector_open: false,
            theme_selector_index: 0,
            header_selected: 0,
            header_editing: None,
            header_edit_buf: String::new(),
            workspace,
            sidebar_open: true,
            sidebar_selected: 0,
            workspace_picker_open: false,
            workspace_picker_items: Vec::new(),
            workspace_picker_selected: 0,
            loaded_collections: Vec::new(),
            active_collection: None,
            active_request: None,
            input_prompt: None,
            confirm_prompt: None,
            pending_action: None,
            pending_delete: None,
        }
    }

    /// Returns the body as a single string for sending/saving.
    pub fn body_text(&self) -> String {
        self.body_lines.join("\n")
    }

    /// Loads a body string into `body_lines` and resets cursor.
    pub fn set_body_text(&mut self, text: &str) {
        self.body_lines = if text.is_empty() {
            vec![String::new()]
        } else {
            text.split('\n').map(String::from).collect()
        };
        self.body_cursor_row = 0;
        self.body_cursor_col = 0;
    }

    /// Clamps the cursor to valid bounds. Call after any mutation.
    pub fn clamp_body_cursor(&mut self) {
        self.body_cursor_row = self
            .body_cursor_row
            .min(self.body_lines.len().saturating_sub(1));
        let line_len = self.body_lines[self.body_cursor_row].len();
        self.body_cursor_col = self.body_cursor_col.min(line_len);
    }

    /// Scans the workspace's `collections/` directory and loads all
    /// collections and their requests into `loaded_collections`.
    /// Clears and repopulates on every call — call after any CRUD operation.
    pub fn load_collections(&mut self) {
        self.loaded_collections.clear();

        let Some(collections_dir) = self.workspace.collections_dir() else {
            return;
        };
        if !collections_dir.exists() {
            return;
        }

        let mut entries: Vec<_> = match std::fs::read_dir(&collections_dir) {
            Ok(e) => e.flatten().collect(),
            Err(_) => return,
        };
        entries.sort_by_key(std::fs::DirEntry::file_name);

        for entry in entries {
            let col_dir = entry.path();
            if !col_dir.is_dir() {
                continue;
            }
            let slug = entry.file_name().to_string_lossy().to_string();
            let col_path = col_dir.join("collection.yaml");
            if !col_path.exists() {
                continue;
            }
            let Ok(collection) = crate::storage::collection::load(&col_path) else {
                continue;
            };

            // Load requests in order field, then alphabetical for unlisted ones
            let mut requests = Vec::new();
            let order = collection.order.clone().unwrap_or_default();

            // First pass: ordered entries
            for filename in &order {
                let req_path = col_dir.join(filename);
                if let Ok(req) = crate::storage::request::load(&req_path) {
                    let stem = std::path::Path::new(filename)
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or(filename)
                        .to_string();
                    requests.push((stem, req));
                }
            }

            // Second pass: any yaml files not in order list
            if let Ok(req_entries) = std::fs::read_dir(&col_dir) {
                let mut extras: Vec<_> = req_entries
                    .flatten()
                    .filter(|e| {
                        let name = e.file_name().to_string_lossy().to_string();
                        std::path::Path::new(&name)
                            .extension()
                            .is_some_and(|ext| ext.eq_ignore_ascii_case("yaml"))
                            && name != "collection.yaml"
                            && !order.contains(&name)
                    })
                    .collect();
                extras.sort_by_key(std::fs::DirEntry::file_name);
                for extra in extras {
                    if let Ok(req) = crate::storage::request::load(&extra.path()) {
                        let stem = extra
                            .path()
                            .file_stem()
                            .and_then(|s| s.to_str())
                            .unwrap_or_default()
                            .to_string();
                        requests.push((stem, req));
                    }
                }
            }

            self.loaded_collections.push(LoadedCollection {
                slug,
                collection,
                requests,
            });
        }
    }
}
