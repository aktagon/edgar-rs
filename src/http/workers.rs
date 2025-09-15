//! Cloudflare Workers HTTP client implementation.

use async_trait::async_trait;
use std::collections::HashMap;
use worker::{Request, RequestInit};

use crate::error::{EdgarApiError, Result};

use super::{HttpClient, HttpResponse};

/// HTTP client implementation for Cloudflare Workers
pub struct WorkerClient;

impl WorkerClient {
    /// Create a new WorkerClient
    pub fn new() -> Self {
        Self
    }
}

#[async_trait(?Send)]
impl HttpClient for WorkerClient {
    async fn get(&self, url: &str, headers: &[(&str, &str)]) -> Result<HttpResponse> {
        // Create request
        let mut init = RequestInit::new();
        init.method = worker::Method::Get;

        // Add headers
        let mut headers_map = worker::Headers::new();
        for (key, value) in headers {
            headers_map
                .set(key, value)
                .map_err(|e| EdgarApiError::request(format!("Failed to set header: {:?}", e)))?;
        }
        init.headers = headers_map;

        let request = Request::new_with_init(url, &init)
            .map_err(|e| EdgarApiError::request(format!("Failed to create request: {:?}", e)))?;

        // Make request using the Fetch API
        let fetch = worker::Fetch::Request(request);
        let mut response = fetch
            .send()
            .await
            .map_err(|e| EdgarApiError::network(&format!("Fetch error: {:?}", e)))?;

        let status = response.status_code();

        // Convert headers
        let mut response_headers = HashMap::new();
        let headers = response.headers();
        // Note: worker::Headers doesn't provide easy iteration, so we'll collect common headers
        let common_headers = [
            "content-type",
            "content-length",
            "retry-after",
            "cache-control",
            "etag",
        ];

        for header_name in &common_headers {
            if let Ok(Some(value)) = headers.get(header_name) {
                response_headers.insert(header_name.to_string(), value);
            }
        }

        // Handle rate limiting
        if status == 429 {
            let retry_after = response_headers
                .get("retry-after")
                .and_then(|s: &String| s.parse::<u64>().ok());

            return Err(EdgarApiError::rate_limit(retry_after));
        }

        // Get response body
        let body = response
            .bytes()
            .await
            .map_err(|e| EdgarApiError::request(format!("Failed to read response body: {:?}", e)))?;

        Ok(HttpResponse {
            status,
            headers: response_headers,
            body,
        })
    }
}