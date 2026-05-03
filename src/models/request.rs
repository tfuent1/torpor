use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Request {
    pub name: String,
    pub description: Option<String>,
    pub method: HttpMethod,
    pub url: String,
    pub headers: Option<HashMap<String, String>>,
    pub params: Option<HashMap<String, String>>,
    pub auth: Option<Auth>,
    pub body: Option<Body>,
    pub assertions: Option<Vec<Assertion>>,
    pub extract: Option<Vec<Extract>>,
    pub pre_request: Option<String>,
    pub post_request: Option<String>,
    pub meta: Option<Meta>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
    Head,
    Options,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Auth {
    None,
    Basic {
        username: String,
        password: String,
    },
    Bearer {
        token: String,
    },
    ApiKey {
        key: String,
        value: String,
        location: ApiKeyLocation,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ApiKeyLocation {
    Header,
    Query,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Body {
    #[serde(rename = "type")]
    pub body_type: BodyType,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum BodyType {
    Json,
    FormUrlencoded,
    Multipart,
    Text,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Assertion {
    pub status: Option<u16>,
    pub header: Option<String>,
    pub contains: Option<String>,
    pub json: Option<String>,
    pub equals: Option<serde_yaml::Value>,
    pub exists: Option<bool>,
    pub response_time_ms: Option<ResponseTimeAssertion>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseTimeAssertion {
    pub lt: Option<u64>,
    pub gt: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Extract {
    pub name: String,
    pub json: Option<String>,
    pub header: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Meta {
    pub tags: Option<Vec<String>>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}
