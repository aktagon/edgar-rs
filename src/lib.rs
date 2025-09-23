//! # edgar-rs
//!
//! `edgar-rs` is a Rust client library for accessing the SEC's EDGAR API system,
//! providing a convenient interface to retrieve company filings and XBRL financial data.
//!
//! ## Features
//!
//! - Access company submission history
//! - Retrieve XBRL company concept data
//! - Get company facts
//! - Fetch XBRL frames data
//! - Get company tickers exchange data
//! - Get mutual fund tickers data
//! - Download bulk submissions and company facts data
//!
//! ## Example
//!
//! ```rust,no_run
//! use edgar_rs::{EdgarApi, EdgarClient, Taxonomy, Unit, Period};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Initialize with a user agent (required by SEC)
//!     let edgar_api = EdgarClient::new("Your Company Name your.email@example.com")?;
//!     
//!     // Get submissions history for Apple Inc. (CIK: 0000320193)
//!     let submissions = edgar_api.get_submissions_history("0000320193").await?;
//!     println!("Apple Inc. submissions status: {}", submissions.status);
//!     
//!     // Get XBRL frames for AccountsPayableCurrent in USD for Q1 2019
//!     let frames = edgar_api.get_xbrl_frames(
//!         Taxonomy::UsGaap,
//!         "AccountsPayableCurrent",
//!         Unit::Simple("USD".to_string()),
//!         Period::Instantaneous(2019, 1)
//!     ).await?;
//!     println!("XBRL frames status: {}", frames.status);
//!
//!     // Get company tickers exchange data
//!     let tickers = edgar_api.get_company_tickers().await?;
//!     let entries = tickers.data.entries()?;
//!     println!("Found {} companies", entries.len());
//!
//!     // Get mutual fund tickers data
//!     let mf_tickers = edgar_api.get_company_tickers_mf().await?;
//!     let mf_entries = mf_tickers.data.entries()?;
//!     println!("Found {} mutual fund entries", mf_entries.len());
//!
//!     Ok(())
//! }
//! ```

// Re-export main components
pub use api::EdgarApi;
pub use client::EdgarClient;
pub use error::{EdgarApiError, Result};

// Re-export HTTP client types
#[cfg(feature = "native")]
pub use http::ReqwestClient;
#[cfg(feature = "cloudflare-workers")]
pub use http::WorkerClient;
pub use http::HttpClient;

// Re-export types
pub use types::{ApiResponse, Period, Taxonomy, Unit};

// Export models
pub use models::{
    company_concept::CompanyConcept, company_facts::CompanyFacts,
    company_tickers::{CompanyTickers, CompanyTickerEntry},
    company_tickers_mf::{CompanyTickersMf, MutualFundTickerEntry}, frames::XbrlFrames,
    submission::FilingEntry, submission::SubmissionHistory,
};

// Modules
mod api;
mod client;
mod error;
mod http;
mod models;
mod types;
mod utils;

// Private modules (not re-exported)
