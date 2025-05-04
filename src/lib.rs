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
//!     let edgar_api = EdgarClient::new("Your Company Name your.email@example.com");
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
//!     Ok(())
//! }
//! ```

// Re-export main components
pub use api::EdgarApi;
pub use client::EdgarClient;
pub use error::{EdgarApiError, Result};

// Re-export types
pub use types::{ApiResponse, Period, Taxonomy, Unit};

// Export models
pub use models::{
    company_concept::CompanyConcept, company_facts::CompanyFacts, frames::XbrlFrames,
    submission::SubmissionHistory,
};

// Modules
mod api;
mod client;
mod error;
mod models;
mod types;
mod utils;

// Private modules (not re-exported)
mod rate_limit;
