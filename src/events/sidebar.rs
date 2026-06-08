use crate::app::{AppState, PendingAction, PendingDelete};
use crossterm::event::{KeyCode, KeyModifiers};

/// Handles key events while the sidebar has focus.
/// Returns `true` if the event was consumed.
#[allow(clippy::too_many_lines)]
pub fn handle(state: &mut AppState, modifiers: KeyModifiers, code: KeyCode) -> bool {
    // --- Confirmation prompt active ---
    if state.confirm_prompt.is_some() {
        match (modifiers, code) {
            (KeyModifiers::NONE, KeyCode::Char('y')) => {
                state.confirm_prompt = None;
                execute_pending_delete(state);
            }
            (KeyModifiers::NONE, KeyCode::Char('n') | KeyCode::Esc) => {
                state.confirm_prompt = None;
                state.pending_delete = None;
            }
            _ => {}
        }
        return true;
    }

    // --- Input prompt active ---
    if state.input_prompt.is_some() {
        match (modifiers, code) {
            (KeyModifiers::NONE, KeyCode::Esc) => {
                state.input_prompt = None;
                state.pending_action = None;
            }
            (KeyModifiers::NONE, KeyCode::Enter) => {
                let input = state
                    .input_prompt
                    .take()
                    .map(|(_, v)| v)
                    .unwrap_or_default();
                execute_pending_input(state, &input);
            }
            (KeyModifiers::NONE, KeyCode::Backspace) => {
                if let Some((_, ref mut input)) = state.input_prompt {
                    input.pop();
                }
            }
            (KeyModifiers::NONE | KeyModifiers::SHIFT, KeyCode::Char(c)) => {
                if let Some((_, ref mut input)) = state.input_prompt {
                    input.push(c);
                }
            }
            _ => {}
        }
        return true;
    }

    // --- Normal navigation ---
    match (modifiers, code) {
        (KeyModifiers::NONE, KeyCode::Down | KeyCode::Char('j')) => {
            let max = sidebar_item_count(state).saturating_sub(1);
            if state.sidebar_selected < max {
                state.sidebar_selected += 1;
            }
        }
        (KeyModifiers::NONE, KeyCode::Up | KeyCode::Char('k')) => {
            state.sidebar_selected = state.sidebar_selected.saturating_sub(1);
        }
        (KeyModifiers::NONE, KeyCode::Enter) => {
            load_selected_request(state);
        }
        (KeyModifiers::NONE, KeyCode::Char('n')) => match resolve_selected(state) {
            SelectedItem::Collection(col_idx) | SelectedItem::Request(col_idx, _) => {
                state.pending_action = Some(PendingAction::NewRequest(col_idx));
                state.input_prompt = Some(("New request name".to_string(), String::new()));
            }
            SelectedItem::None => {
                state.pending_action = Some(PendingAction::NewCollection);
                state.input_prompt = Some(("New collection name".to_string(), String::new()));
            }
        },
        (KeyModifiers::SHIFT, KeyCode::Char('N')) => {
            state.pending_action = Some(PendingAction::NewCollection);
            state.input_prompt = Some(("New collection name".to_string(), String::new()));
        }
        (KeyModifiers::NONE, KeyCode::Char('r')) => match resolve_selected(state) {
            SelectedItem::Collection(col_idx) => {
                let current = state.loaded_collections[col_idx].collection.name.clone();
                state.pending_action = Some(PendingAction::RenameCollection(col_idx));
                state.input_prompt = Some(("Rename collection".to_string(), current));
            }
            SelectedItem::Request(col_idx, req_idx) => {
                let current = state.loaded_collections[col_idx].requests[req_idx]
                    .0
                    .clone();
                state.pending_action = Some(PendingAction::RenameRequest(col_idx, req_idx));
                state.input_prompt = Some(("Rename request".to_string(), current));
            }
            SelectedItem::None => {}
        },
        (KeyModifiers::NONE, KeyCode::Char('d')) => match resolve_selected(state) {
            SelectedItem::Collection(col_idx) => {
                let name = state.loaded_collections[col_idx].collection.name.clone();
                state.pending_delete = Some(PendingDelete::Collection(col_idx));
                state.confirm_prompt = Some(format!("Delete collection '{name}'?"));
            }
            SelectedItem::Request(col_idx, req_idx) => {
                let name = state.loaded_collections[col_idx].requests[req_idx]
                    .0
                    .clone();
                state.pending_delete = Some(PendingDelete::Request(col_idx, req_idx));
                state.confirm_prompt = Some(format!("Delete request '{name}'?"));
            }
            SelectedItem::None => {}
        },
        _ => return false,
    }
    true
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

enum SelectedItem {
    None,
    Collection(usize),
    Request(usize, usize), // col_idx, req_idx
}

/// Maps `state.sidebar_selected` (flat index) to a collection or request.
fn resolve_selected(state: &AppState) -> SelectedItem {
    if state.loaded_collections.is_empty() {
        return SelectedItem::None;
    }
    let mut flat = 0usize;
    for (col_idx, col) in state.loaded_collections.iter().enumerate() {
        if flat == state.sidebar_selected {
            return SelectedItem::Collection(col_idx);
        }
        flat += 1;
        for req_idx in 0..col.requests.len() {
            if flat == state.sidebar_selected {
                return SelectedItem::Request(col_idx, req_idx);
            }
            flat += 1;
        }
    }
    SelectedItem::None
}

/// Total number of flat sidebar items.
fn sidebar_item_count(state: &AppState) -> usize {
    state
        .loaded_collections
        .iter()
        .map(|c| 1 + c.requests.len())
        .sum()
}

/// Load the selected request into the editor panes.
fn load_selected_request(state: &mut AppState) {
    let SelectedItem::Request(col_idx, req_idx) = resolve_selected(state) else {
        return;
    };
    let req = state.loaded_collections[col_idx].requests[req_idx]
        .1
        .clone();
    state.url.clone_from(&req.url);
    state.method = req.method.clone();
    let body_text = req.body.map(|b| b.content).unwrap_or_default();
    state.set_body_text(&body_text);
    state.headers = req
        .headers
        .map(|h| h.into_iter().collect())
        .unwrap_or_default();
    state.cursor_pos = state.url.len();
    state.active_collection = Some(col_idx);
    state.active_request = Some(req_idx);
    state.status_message = Some(format!(
        "Loaded: {}",
        state.loaded_collections[col_idx].requests[req_idx].0
    ));
}

fn execute_pending_delete(state: &mut AppState) {
    let Some(pending) = state.pending_delete.take() else {
        return;
    };
    match pending {
        PendingDelete::Collection(col_idx) => {
            let Some(root) = state.workspace.root() else {
                return;
            };
            let slug = state.loaded_collections[col_idx].slug.clone();
            let col_dir = root.join("collections").join(&slug);
            if std::fs::remove_dir_all(&col_dir).is_ok() {
                state.status_message = Some(format!("Deleted collection '{slug}'"));
            } else {
                state.status_message = Some(format!("Failed to delete '{slug}'"));
            }
            state.load_collections();
            state.sidebar_selected = state.sidebar_selected.saturating_sub(1);
        }
        PendingDelete::Request(col_idx, req_idx) => {
            let Some(root) = state.workspace.root() else {
                return;
            };
            let slug = state.loaded_collections[col_idx].slug.clone();
            let filename = format!(
                "{}.yaml",
                state.loaded_collections[col_idx].requests[req_idx].0
            );
            let req_path = root.join("collections").join(&slug).join(&filename);
            if std::fs::remove_file(&req_path).is_ok() {
                // Also remove from collection order if present
                update_collection_order_remove(state, col_idx, &filename);
                state.status_message = Some(format!("Deleted request '{filename}'"));
            } else {
                state.status_message = Some(format!("Failed to delete '{filename}'"));
            }
            state.load_collections();
            state.sidebar_selected = state.sidebar_selected.saturating_sub(1);
        }
    }
}

fn execute_pending_input(state: &mut AppState, input: &str) {
    let input = input.trim().to_string();
    if input.is_empty() {
        state.pending_action = None;
        return;
    }

    let Some(action) = state.pending_action.take() else {
        return;
    };

    match action {
        PendingAction::NewCollection => {
            let Some(collections_dir) = state.workspace.collections_dir() else {
                return;
            };
            let slug = slugify(&input);
            let col_dir = collections_dir.join(&slug);
            let col_path = col_dir.join("collection.yaml");
            let collection = crate::models::collection::Collection {
                name: input.clone(),
                description: None,
                base_url: None,
                auth: None,
                headers: None,
                order: None,
                meta: None,
            };
            if std::fs::create_dir_all(&col_dir).is_ok()
                && crate::storage::collection::save(&col_path, &collection).is_ok()
            {
                state.status_message = Some(format!("Created collection '{input}'"));
            }
            state.load_collections();
        }
        PendingAction::NewRequest(col_idx) => {
            let Some(root) = state.workspace.root() else {
                return;
            };
            let slug = state.loaded_collections[col_idx].slug.clone();
            let filename = format!("{}.yaml", slugify(&input));
            let req_path = root.join("collections").join(&slug).join(&filename);
            let request = crate::models::request::Request {
                name: input.clone(),
                description: None,
                method: crate::models::request::HttpMethod::Get,
                url: String::new(),
                headers: None,
                params: None,
                auth: None,
                body: None,
                assertions: None,
                extract: None,
                pre_request: None,
                post_request: None,
                meta: None,
            };
            if crate::storage::request::save(&req_path, &request).is_ok() {
                update_collection_order_add(state, col_idx, &filename);
                state.status_message = Some(format!("Created request '{input}'"));
            }
            state.load_collections();
        }
        PendingAction::RenameCollection(col_idx) => {
            let col_path = state
                .workspace
                .collection_path(&state.loaded_collections[col_idx].slug.clone());
            let Some(col_path) = col_path else { return };
            let mut collection = state.loaded_collections[col_idx].collection.clone();
            collection.name.clone_from(&input);
            if crate::storage::collection::save(&col_path, &collection).is_ok() {
                state.status_message = Some(format!("Renamed to '{input}'"));
            }
            state.load_collections();
        }
        PendingAction::RenameRequest(col_idx, req_idx) => {
            let Some(root) = state.workspace.root() else {
                return;
            };
            let slug = state.loaded_collections[col_idx].slug.clone();
            let old_filename = format!(
                "{}.yaml",
                state.loaded_collections[col_idx].requests[req_idx].0
            );
            let new_filename = format!("{}.yaml", slugify(&input));
            let old_path = root.join("collections").join(&slug).join(&old_filename);
            let new_path = root.join("collections").join(&slug).join(&new_filename);
            if std::fs::rename(&old_path, &new_path).is_ok() {
                // Update request name field and order
                let mut req = state.loaded_collections[col_idx].requests[req_idx]
                    .1
                    .clone();
                req.name.clone_from(&input);
                let _ = crate::storage::request::save(&new_path, &req);
                update_collection_order_rename(state, col_idx, &old_filename, &new_filename);
                state.status_message = Some(format!("Renamed to '{input}'"));
            }
            state.load_collections();
        }
    }
}

/// Convert a display name to a safe filename slug.
fn slugify(name: &str) -> String {
    name.trim()
        .to_lowercase()
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' {
                c
            } else {
                '_'
            }
        })
        .collect()
}

fn update_collection_order_add(state: &mut AppState, col_idx: usize, filename: &str) {
    let Some(col_path) = state
        .workspace
        .collection_path(&state.loaded_collections[col_idx].slug.clone())
    else {
        return;
    };
    let mut collection = state.loaded_collections[col_idx].collection.clone();
    collection
        .order
        .get_or_insert_with(Vec::new)
        .push(filename.to_string());
    let _ = crate::storage::collection::save(&col_path, &collection);
}

fn update_collection_order_remove(state: &mut AppState, col_idx: usize, filename: &str) {
    let Some(col_path) = state
        .workspace
        .collection_path(&state.loaded_collections[col_idx].slug.clone())
    else {
        return;
    };
    let mut collection = state.loaded_collections[col_idx].collection.clone();
    if let Some(ref mut order) = collection.order {
        order.retain(|f| f != filename);
    }
    let _ = crate::storage::collection::save(&col_path, &collection);
}

fn update_collection_order_rename(
    state: &mut AppState,
    col_idx: usize,
    old_filename: &str,
    new_filename: &str,
) {
    let Some(col_path) = state
        .workspace
        .collection_path(&state.loaded_collections[col_idx].slug.clone())
    else {
        return;
    };
    let mut collection = state.loaded_collections[col_idx].collection.clone();
    if let Some(ref mut order) = collection.order {
        for f in order.iter_mut() {
            if f == old_filename {
                *f = new_filename.to_string();
            }
        }
    }
    let _ = crate::storage::collection::save(&col_path, &collection);
}
