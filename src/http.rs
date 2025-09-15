//! HTTP client abstraction for cross-platform support.
//!
//! This module provides an HTTP client trait that can be implemented for different
//! runtimes, including native (reqwest + tokio) and Cloudflare Workers.

use async_trait::async_trait;
use serde::de::DeserializeOwned;

use crate::error::{EdgarApiError, Result};

/// HTTP response wrapper
#[derive(Debug)]
pub struct HttpResponse {
    pub status: u16,
    pub headers: std::collections::HashMap<String, String>,
    pub body: Vec<u8>,
}

impl HttpResponse {
    /// Parse the response body as JSON
    pub fn json<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        serde_json::from_slice(&self.body).map_err(|e| EdgarApiError::parse(e))
    }

    /// Get response body as bytes
    pub fn bytes(&self) -> &[u8] {
        &self.body
    }

    /// Check if the response status indicates success
    pub fn is_success(&self) -> bool {
        self.status >= 200 && self.status < 300
    }
}

/// Extension trait for making JSON requests
pub trait HttpClientExt: HttpClient {
    /// Make a GET request and parse the response as JSON
    async fn get_json<T>(&self, url: &str, headers: &[(&str, &str)]) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let response = self.get(url, headers).await?;
        if !response.is_success() {
            return Err(EdgarApiError::api(
                response.status,
                format!("Request to {} failed with status {}", url, response.status),
            ));
        }
        response.json()
    }
}

// Blanket implementation for all HttpClient implementations
impl<T: HttpClient> HttpClientExt for T {
}

/// HTTP client trait for making requests
#[async_trait]
#[cfg(feature = "native")]
pub trait HttpClient: Send + Sync {
    /// Make a GET request to the specified URL
    async fn get(&self, url: &str, headers: &[(&str, &str)]) -> Result<HttpResponse>;

    /// Make a GET request and return the response body as bytes
    async fn get_bytes(&self, url: &str, headers: &[(&str, &str)]) -> Result<Vec<u8>> {
        let response = self.get(url, headers).await?;
        if !response.is_success() {
            return Err(EdgarApiError::api(
                response.status,
                format!("Request to {} failed with status {}", url, response.status),
            ));
        }
        Ok(response.body)
    }

}

/// HTTP client trait for making requests (Cloudflare Workers)
#[async_trait(?Send)]
#[cfg(feature = "cloudflare-workers")]
pub trait HttpClient {
    /// Make a GET request to the specified URL
    async fn get(&self, url: &str, headers: &[(&str, &str)]) -> Result<HttpResponse>;

    /// Make a GET request and return the response body as bytes
    async fn get_bytes(&self, url: &str, headers: &[(&str, &str)]) -> Result<Vec<u8>> {
        let response = self.get(url, headers).await?;
        if !response.is_success() {
            return Err(EdgarApiError::api(
                response.status,
                format!("Request to {} failed with status {}", url, response.status),
            ));
        }
        Ok(response.body)
    }

}

#[cfg(feature = "native")]
mod native;

#[cfg(feature = "cloudflare-workers")]
mod workers;

#[cfg(feature = "native")]
pub use native::ReqwestClient;

#[cfg(feature = "cloudflare-workers")]
pub use workers::WorkerClient;