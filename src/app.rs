use crate::models::request::HttpMethod;
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
    UrlBar,
    RequestPane,
    ResponsePane,
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
pub struct AppState {
    // Request being built
    pub method: HttpMethod,
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub body: String,

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
    pub cursor_pos: usize,

    // Headers editor state
    pub header_selected: usize, // which row is selected (0-indexed)
    pub header_editing: Option<HeaderField>, // None = navigating, Some = editing
    pub header_edit_buf: String, // buffer for the field being typed into
}

impl AppState {
    /// Creates a new `AppState` with sensible defaults.
    pub fn new() -> Self {
        let (response_tx, response_rx) = mpsc::channel(1);
        Self {
            method: HttpMethod::Get,
            url: String::new(),
            headers: Vec::new(),
            body: String::new(),
            response: None,
            response_tx,
            response_rx,
            focus: Focus::UrlBar,
            active_tab: RequestTab::Body,
            status_message: None,
            request_in_flight: false,
            cursor_pos: 0,
            header_selected: 0,
            header_editing: None,
            header_edit_buf: String::new(),
        }
    }
}
