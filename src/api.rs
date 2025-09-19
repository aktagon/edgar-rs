//! Defines the `EdgarApi` trait which specifies all available SEC EDGAR API endpoints.

use async_trait::async_trait;
#[cfg(feature = "native")]
use std::path::Path;

use crate::error::Result;
use crate::models::{
    company_concept::CompanyConcept, company_facts::CompanyFacts, frames::XbrlFrames,
    submission::{Recent, SubmissionHistory},
};
use crate::types::{ApiResponse, Period, Taxonomy, Unit};

/// The `EdgarApi` trait defines methods for accessing the SEC EDGAR API endpoints.
///
/// This trait provides a common interface for accessing the various SEC EDGAR API
/// endpoints, including submission history, company concepts, company facts, and XBRL frames.
#[async_trait]
#[cfg(feature = "native")]
pub trait EdgarApi {
    /// Get company's submissions history
    ///
    /// Endpoint: https://data.sec.gov/submissions/CIK##########.json
    ///
    /// # Parameters
    /// * `cik` - 10-digit Central Index Key, including leading zeros
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use edgar_rs::{EdgarApi, EdgarClient};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let edgar_api = EdgarClient::new("Your Company Name your.email@example.com");
    /// let submissions = edgar_api.get_submissions_history("0000320193").await?;
    /// println!("Company name: {}", submissions.data.name);
    /// # Ok(())
    /// # }
    /// ```
    async fn get_submissions_history(&self, cik: &str) -> Result<ApiResponse<SubmissionHistory>>;

    /// Get additional submissions history file
    ///
    /// Endpoint: https://data.sec.gov/submissions/{filename}
    ///
    /// This method fetches additional filing history files when there are more than 1000 filings.
    /// The filenames are provided in the `files` field of the main submissions response.
    ///
    /// # Parameters
    /// * `filename` - Name of the additional submissions file (e.g., "CIK0001067983-submissions-001.json")
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use edgar_rs::{EdgarApi, EdgarClient};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let edgar_api = EdgarClient::new("Your Company Name your.email@example.com");
    /// let submissions = edgar_api.get_submissions_history("0000320193").await?;
    /// if let Some(files) = submissions.data.filings.files {
    ///     if !files.is_empty() {
    ///         let additional_submissions = edgar_api.get_submissions_file(&files[0].name).await?;
    ///         println!("Got additional filings: {}", additional_submissions.data.filings.recent.accession_number.len());
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    async fn get_submissions_file(&self, filename: &str) -> Result<ApiResponse<Recent>>;

    /// Get company concept data for a specific taxonomy and tag
    ///
    /// Endpoint: https://data.sec.gov/api/xbrl/companyconcept/CIK##########/{taxonomy}/{tag}.json
    ///
    /// # Parameters
    /// * `cik` - 10-digit Central Index Key, including leading zeros
    /// * `taxonomy` - XBRL taxonomy (e.g. "us-gaap", "dei")
    /// * `tag` - XBRL tag identifier (e.g. "AccountsPayableCurrent")
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use edgar_rs::{EdgarApi, EdgarClient, Taxonomy};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let edgar_api = EdgarClient::new("Your Company Name your.email@example.com");
    /// let concept = edgar_api.get_company_concept(
    ///     "0000320193",
    ///     Taxonomy::UsGaap,
    ///     "AccountsPayableCurrent"
    /// ).await?;
    /// println!("Concept label: {}", concept.data.label);
    /// # Ok(())
    /// # }
    /// ```
    async fn get_company_concept(
        &self,
        cik: &str,
        taxonomy: Taxonomy,
        tag: &str,
    ) -> Result<ApiResponse<CompanyConcept>>;

    /// Get all company facts for a company
    ///
    /// Endpoint: https://data.sec.gov/api/xbrl/companyfacts/CIK##########.json
    ///
    /// # Parameters
    /// * `cik` - 10-digit Central Index Key, including leading zeros
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use edgar_rs::{EdgarApi, EdgarClient};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let edgar_api = EdgarClient::new("Your Company Name your.email@example.com");
    /// let facts = edgar_api.get_company_facts("0000320193").await?;
    /// println!("Company CIK: {}", facts.data.cik);
    /// # Ok(())
    /// # }
    /// ```
    async fn get_company_facts(&self, cik: &str) -> Result<ApiResponse<CompanyFacts>>;

    /// Get XBRL frames data for a specific taxonomy, tag, unit and period
    ///
    /// Endpoint: https://data.sec.gov/api/xbrl/frames/{taxonomy}/{tag}/{unit}/{period}.json
    ///
    /// # Parameters
    /// * `taxonomy` - XBRL taxonomy (e.g. "us-gaap", "dei")
    /// * `tag` - XBRL tag identifier (e.g. "AccountsPayableCurrent")
    /// * `unit` - Unit of measure (e.g. Unit::Simple("USD".to_string()))
    /// * `period` - Time period (e.g. Period::Instantaneous(2019, 1) for Q1 2019)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use edgar_rs::{EdgarApi, EdgarClient, Taxonomy, Unit, Period};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let edgar_api = EdgarClient::new("Your Company Name your.email@example.com");
    /// let frames = edgar_api.get_xbrl_frames(
    ///     Taxonomy::UsGaap,
    ///     "AccountsPayableCurrent",
    ///     Unit::Simple("USD".to_string()),
    ///     Period::Instantaneous(2019, 1)
    /// ).await?;
    /// println!("Data count: {}", frames.data.data.len());
    /// # Ok(())
    /// # }
    /// ```
    async fn get_xbrl_frames(
        &self,
        taxonomy: Taxonomy,
        tag: &str,
        unit: Unit,
        period: Period,
    ) -> Result<ApiResponse<XbrlFrames>>;

    /// Download bulk submissions data
    ///
    /// Note: This functionality is not available in Cloudflare Workers
    /// as it requires file system access.
    #[cfg(feature = "native")]
    async fn download_bulk_submissions(&self, output_path: &str) -> Result<()>;

    /// Download bulk company facts data
    ///
    /// Note: This functionality is not available in Cloudflare Workers
    /// as it requires file system access.
    #[cfg(feature = "native")]
    async fn download_bulk_company_facts(&self, output_path: &str) -> Result<()>;

    /// Extract ZIP files
    ///
    /// Note: This functionality is not available in Cloudflare Workers
    /// as it requires file system access.
    #[cfg(feature = "native")]
    async fn extract_zip_files(&self, zip_path: &Path, output_dir: &Path) -> Result<()>;
}

/// The `EdgarApi` trait defines methods for accessing the SEC EDGAR API endpoints (Cloudflare Workers).
///
/// This trait provides a common interface for accessing the various SEC EDGAR API
/// endpoints, including submission history, company concepts, company facts, and XBRL frames.
#[async_trait(?Send)]
#[cfg(feature = "cloudflare-workers")]
pub trait EdgarApi {
    /// Get company's submissions history
    ///
    /// Endpoint: https://data.sec.gov/submissions/CIK##########.json
    ///
    /// # Parameters
    /// * `cik` - 10-digit Central Index Key, including leading zeros
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use edgar_rs::{EdgarApi, EdgarClient};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let edgar_api = EdgarClient::new("Your Company Name your.email@example.com");
    /// let submissions = edgar_api.get_submissions_history("0000320193").await?;
    /// println!("Company name: {}", submissions.data.name);
    /// # Ok(())
    /// # }
    /// ```
    async fn get_submissions_history(&self, cik: &str) -> Result<ApiResponse<SubmissionHistory>>;

    /// Get additional submissions history file
    ///
    /// Endpoint: https://data.sec.gov/submissions/{filename}
    ///
    /// This method fetches additional filing history files when there are more than 1000 filings.
    /// The filenames are provided in the `files` field of the main submissions response.
    ///
    /// # Parameters
    /// * `filename` - Name of the additional submissions file (e.g., "CIK0001067983-submissions-001.json")
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use edgar_rs::{EdgarApi, EdgarClient};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let edgar_api = EdgarClient::new("Your Company Name your.email@example.com");
    /// let submissions = edgar_api.get_submissions_history("0001067983").await?;
    /// if let Some(files) = &submissions.data.filings.files {
    ///     for file in files {
    ///         let additional = edgar_api.get_submissions_file(&file.name).await?;
    ///         println!("Additional filings: {}", additional.data.accession_number.len());
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    async fn get_submissions_file(&self, filename: &str) -> Result<ApiResponse<Recent>>;

    /// Get company data for a specific concept and taxonomy
    ///
    /// Endpoint: https://data.sec.gov/api/xbrl/companyconcept/CIK##########/taxonomy/tag.json
    ///
    /// # Parameters
    /// * `cik` - 10-digit Central Index Key, including leading zeros
    /// * `taxonomy` - Taxonomy to use (us-gaap, ifrs-full, dei, or srt)
    /// * `tag` - The concept tag to retrieve
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use edgar_rs::{EdgarApi, EdgarClient, Taxonomy};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let edgar_api = EdgarClient::new("Your Company Name your.email@example.com");
    /// let concept = edgar_api.get_company_concept(
    ///     "0000320193",
    ///     Taxonomy::UsGaap,
    ///     "AccountsPayableCurrent"
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn get_company_concept(
        &self,
        cik: &str,
        taxonomy: Taxonomy,
        tag: &str,
    ) -> Result<ApiResponse<CompanyConcept>>;

    /// Get all company facts for a specific company
    ///
    /// Endpoint: https://data.sec.gov/api/xbrl/companyfacts/CIK##########.json
    ///
    /// # Parameters
    /// * `cik` - 10-digit Central Index Key, including leading zeros
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use edgar_rs::{EdgarApi, EdgarClient};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let edgar_api = EdgarClient::new("Your Company Name your.email@example.com");
    /// let facts = edgar_api.get_company_facts("0000320193").await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn get_company_facts(&self, cik: &str) -> Result<ApiResponse<CompanyFacts>>;

    /// Get XBRL frames data for a specific taxonomy, tag, unit and period
    ///
    /// Endpoint: https://data.sec.gov/api/xbrl/frames/{taxonomy}/{tag}/{unit}/{period}.json
    ///
    /// # Parameters
    /// * `taxonomy` - XBRL taxonomy (e.g. "us-gaap", "dei")
    /// * `tag` - XBRL tag identifier (e.g. "AccountsPayableCurrent")
    /// * `unit` - Unit of measure (e.g. Unit::Simple("USD".to_string()))
    /// * `period` - Time period (e.g. Period::Instantaneous(2019, 1) for Q1 2019)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use edgar_rs::{EdgarApi, EdgarClient, Taxonomy, Unit, Period};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let edgar_api = EdgarClient::new("Your Company Name your.email@example.com");
    /// let frames = edgar_api.get_xbrl_frames(
    ///     Taxonomy::UsGaap,
    ///     "AccountsPayableCurrent",
    ///     Unit::Simple("USD".to_string()),
    ///     Period::Instantaneous(2019, 1)
    /// ).await?;
    /// println!("Data count: {}", frames.data.data.len());
    /// # Ok(())
    /// # }
    /// ```
    async fn get_xbrl_frames(
        &self,
        taxonomy: Taxonomy,
        tag: &str,
        unit: Unit,
        period: Period,
    ) -> Result<ApiResponse<XbrlFrames>>;
}
