use crate::models::request::{Auth, Meta};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Collection {
    pub name: String,
    pub description: Option<String>,
    pub base_url: Option<String>,
    pub auth: Option<Auth>,
    pub headers: Option<HashMap<String, String>>,
    pub order: Option<Vec<String>>,
    pub meta: Option<Meta>,
}
