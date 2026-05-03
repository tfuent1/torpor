use serde::{Deserialize, Serialize};

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

