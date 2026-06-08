use crate::models::request::HttpMethod;
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
}
