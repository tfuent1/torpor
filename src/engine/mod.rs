use crate::app::ResponseState;
use crate::models::request::{ApiKeyLocation, Auth, Body, BodyType, HttpMethod, Request};
use anyhow::Context;
use std::sync::Arc;
use std::time::Instant;

/// Wraps a `reqwest::Client` behind an `Arc` so it can be cheaply shared
/// with spawned Tokio tasks without exposing the field directly.
pub struct RequestEngine {
    client: Arc<reqwest::Client>,
}

impl RequestEngine {
    /// Creates a new `RequestEngine` with a shared `reqwest::Client`.
    pub fn new() -> anyhow::Result<Self> {
        let client = Arc::new(
            reqwest::Client::builder()
                .build()
                .context("failed to build reqwest client")?,
        );
        Ok(Self { client })
    }

    /// Returns a cheap clone of the inner `Arc<Client>` for use in spawned tasks.
    pub fn client(&self) -> Arc<reqwest::Client> {
        Arc::clone(&self.client)
    }

    /// Sends a request using `&self`. Delegates to `send_with_client`.
    #[allow(dead_code)]
    pub async fn send(&self, request: &Request) -> anyhow::Result<ResponseState> {
        Self::send_with_client(&self.client, request).await
    }

    /// Core send logic. Takes a `&reqwest::Client` directly so it can be called
    /// from a spawned task that holds an `Arc<Client>` rather than `&self`.
    pub async fn send_with_client(
        client: &reqwest::Client,
        request: &Request,
    ) -> anyhow::Result<ResponseState> {
        let method = to_reqwest_method(&request.method);
        let mut builder = client.request(method, &request.url);

        if let Some(headers) = &request.headers {
            for (key, value) in headers {
                builder = builder.header(key.as_str(), value.as_str());
            }
        }

        if let Some(body) = &request.body {
            builder = apply_body(builder, body);
        }

        if let Some(auth) = &request.auth {
            builder = apply_auth(builder, auth);
        }

        let start = Instant::now();
        let response = builder.send().await.context("request failed")?;
        let duration_ms = u64::try_from(start.elapsed().as_millis()).unwrap_or(u64::MAX);

        let status = response.status().as_u16();
        let headers: Vec<(String, String)> = response
            .headers()
            .iter()
            .map(|(k, v)| {
                (
                    k.to_string(),
                    v.to_str().unwrap_or("<non-utf8>").to_string(),
                )
            })
            .collect();

        let body = response
            .text()
            .await
            .context("failed to read response body")?;
        let size_bytes = body.len();

        Ok(ResponseState {
            status,
            duration_ms,
            headers,
            body,
            size_bytes,
        })
    }
}

/// Converts Torpor's `HttpMethod` to a `reqwest::Method`.
fn to_reqwest_method(method: &HttpMethod) -> reqwest::Method {
    match method {
        HttpMethod::Get => reqwest::Method::GET,
        HttpMethod::Post => reqwest::Method::POST,
        HttpMethod::Put => reqwest::Method::PUT,
        HttpMethod::Patch => reqwest::Method::PATCH,
        HttpMethod::Delete => reqwest::Method::DELETE,
        HttpMethod::Head => reqwest::Method::HEAD,
        HttpMethod::Options => reqwest::Method::OPTIONS,
    }
}

/// Applies body content and the appropriate Content-Type header.
fn apply_body(builder: reqwest::RequestBuilder, body: &Body) -> reqwest::RequestBuilder {
    match body.body_type {
        BodyType::Json => builder
            .header("Content-Type", "application/json")
            .body(body.content.clone()),
        BodyType::Text => builder
            .header("Content-Type", "text/plain")
            .body(body.content.clone()),
        BodyType::FormUrlencoded => builder
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body.content.clone()),
        BodyType::Multipart => {
            // Full multipart support requires reqwest::multipart::Form.
            // Deferred to a later phase — send as plain text for now.
            builder.body(body.content.clone())
        }
    }
}

/// Applies auth configuration to the request builder.
fn apply_auth(builder: reqwest::RequestBuilder, auth: &Auth) -> reqwest::RequestBuilder {
    match auth {
        Auth::None => builder,
        Auth::Bearer { token } => builder.bearer_auth(token),
        Auth::Basic { username, password } => builder.basic_auth(username, Some(password)),
        Auth::ApiKey { key, value, location } => match location {
            ApiKeyLocation::Header => builder.header(key.as_str(), value.as_str()),
            ApiKeyLocation::Query => builder.query(&[(key.as_str(), value.as_str())]),
        },
    }
}
