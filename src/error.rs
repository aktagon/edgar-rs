//! Error types for the edgar-rs library.
//!
//! This module defines the error types used throughout the library, including
//! `EdgarApiError` for handling various error conditions.

use std::fmt;
use thiserror::Error;

/// A specialized Result type for edgar-rs operations.
pub type Result<T> = std::result::Result<T, EdgarApiError>;

/// Errors that can occur when interacting with the SEC EDGAR API.
#[derive(Error, Debug)]
pub enum EdgarApiError {
    /// Network-related errors.
    #[error("Network error: {0}")]
    NetworkError(String),

    /// Errors parsing API responses.
    #[error("Parse error: {0}")]
    ParseError(String),

    /// Errors forming or sending requests.
    #[error("Request error: {0}")]
    RequestError(String),

    /// Errors returned by the API itself.
    #[error("API error {status}: {message}")]
    ApiError {
        /// HTTP status code
        status: u16,
        /// Error message
        message: String,
    },

    /// Rate limit exceeded.
    #[error("Rate limit exceeded. Please wait {retry_after:?} seconds before retrying.")]
    RateLimitExceeded {
        /// Suggested retry after (seconds)
        retry_after: Option<u64>,
    },

    /// Invalid CIK number format.
    #[error("Invalid CIK format: {0}")]
    InvalidCik(String),

    /// I/O errors when writing files.
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    /// Zip extraction errors.
    #[error("Zip extraction error: {0}")]
    ZipError(String),

    /// HTTP client errors from reqwest.
    #[error("HTTP client error: {0}")]
    HttpError(#[from] reqwest::Error),
}

impl EdgarApiError {
    /// Returns true if the error is a transient error that may succeed if retried.
    pub fn is_transient(&self) -> bool {
        match self {
            EdgarApiError::NetworkError(_) => true,
            EdgarApiError::ApiError { status, .. } => {
                *status == 429 || *status == 503 || *status >= 500
            }
            EdgarApiError::RateLimitExceeded { .. } => true,
            _ => false,
        }
    }

    /// Returns true if the error is due to rate limiting.
    pub fn is_rate_limited(&self) -> bool {
        matches!(self, EdgarApiError::RateLimitExceeded { .. })
            || matches!(self, EdgarApiError::ApiError { status: 429, .. })
    }

    /// Creates a new network error.
    pub fn network(msg: impl fmt::Display) -> Self {
        EdgarApiError::NetworkError(msg.to_string())
    }

    /// Creates a new parse error.
    pub fn parse(msg: impl fmt::Display) -> Self {
        EdgarApiError::ParseError(msg.to_string())
    }

    /// Creates a new request error.
    pub fn request(msg: impl fmt::Display) -> Self {
        EdgarApiError::RequestError(msg.to_string())
    }

    /// Creates a new API error.
    pub fn api(status: u16, message: impl fmt::Display) -> Self {
        EdgarApiError::ApiError {
            status,
            message: message.to_string(),
        }
    }

    /// Creates a new rate limit exceeded error.
    pub fn rate_limit(retry_after: Option<u64>) -> Self {
        EdgarApiError::RateLimitExceeded { retry_after }
    }

    /// Creates a new invalid CIK error.
    pub fn invalid_cik(cik: impl fmt::Display) -> Self {
        EdgarApiError::InvalidCik(cik.to_string())
    }

    /// Creates a new zip error.
    pub fn zip(msg: impl fmt::Display) -> Self {
        EdgarApiError::ZipError(msg.to_string())
    }
}
