use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Environment {
    pub name: String,
    pub color: Option<String>,
    pub variables: Option<HashMap<String, String>>,
    pub secrets: Option<HashMap<String, SecretValue>>,
    pub meta: Option<Meta>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum SecretValue {
    Keyring,
    Value(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Meta {
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

