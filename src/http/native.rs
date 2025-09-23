//! Native HTTP client implementation using reqwest.

use async_trait::async_trait;
use log::{error, trace};
use reqwest::{Client, Proxy};
use std::collections::HashMap;
use std::env;
use std::time::Duration;

use crate::error::{EdgarApiError, Result};

use super::{HttpClient, HttpResponse};

/// HTTP client implementation using reqwest
pub struct ReqwestClient {
    client: Client,
}

impl ReqwestClient {
    /// Create a new ReqwestClient with default settings
    pub fn new() -> Result<Self> {
        let mut builder = Client::builder().timeout(Duration::from_secs(30));

        // Check for proxy environment variables and configure if present
        if let Ok(proxy_url) = env::var("HTTP_PROXY").or_else(|_| env::var("http_proxy")) {
            trace!("Configuring HTTP proxy: {}", proxy_url);
            let proxy = Proxy::http(&proxy_url).map_err(|e| EdgarApiError::network(e))?;
            builder = builder.proxy(proxy);
        }

        if let Ok(proxy_url) = env::var("HTTPS_PROXY").or_else(|_| env::var("https_proxy")) {
            trace!("Configuring HTTPS proxy: {}", proxy_url);
            let proxy = Proxy::https(&proxy_url).map_err(|e| EdgarApiError::network(e))?;
            builder = builder.proxy(proxy);
        }

        // For testing with proxy, disable SSL verification if requested
        if env::var("EDGAR_DISABLE_SSL_VERIFY").is_ok() {
            trace!("Disabling SSL verification for proxy testing");
            builder = builder.danger_accept_invalid_certs(true);
        }

        let client = builder.build().map_err(|e| EdgarApiError::network(e))?;

        Ok(Self { client })
    }

    /// Create a new ReqwestClient with custom settings
    pub fn with_client(client: Client) -> Self {
        Self { client }
    }
}

#[async_trait]
impl HttpClient for ReqwestClient {
    async fn get(&self, url: &str, headers: &[(&str, &str)]) -> Result<HttpResponse> {
        trace!("Starting HTTP request to {}", url);

        // Build request
        let mut request_builder = self.client.get(url);
        for (key, value) in headers {
            request_builder = request_builder.header(*key, *value);
        }

        trace!("Sending GET request to {}", url);
        let response = request_builder.send().await.map_err(|e| {
            error!("Network error while requesting {}: {}", url, e);
            EdgarApiError::network(e)
        })?;

        let status = response.status().as_u16();
        trace!("Received response from {} with status code {}", url, status);

        // Convert headers
        let mut response_headers = HashMap::new();
        for (key, value) in response.headers() {
            if let Ok(value_str) = value.to_str() {
                response_headers.insert(key.as_str().to_string(), value_str.to_string());
            }
        }

        // Handle rate limiting
        if status == 429 {
            let retry_after = response_headers
                .get("retry-after")
                .and_then(|s| s.parse::<u64>().ok());

            error!(
                "Rate limited by API (status 429). Retry-After: {:?}",
                retry_after
            );
            return Err(EdgarApiError::rate_limit(retry_after));
        }

        // Get response body
        let body = response.bytes().await.map_err(|e| {
            error!("Failed to read response body from {}: {}", url, e);
            EdgarApiError::network(e)
        })?;

        trace!("Successfully received response from {}", url);
        Ok(HttpResponse {
            status,
            headers: response_headers,
            body: body.to_vec(),
        })
    }
}

impl Default for ReqwestClient {
    fn default() -> Self {
        Self::new().expect("Failed to create default ReqwestClient")
    }
}

