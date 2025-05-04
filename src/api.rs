//! Defines the `EdgarApi` trait which specifies all available SEC EDGAR API endpoints.

use async_trait::async_trait;
use std::path::Path;

use crate::error::Result;
use crate::models::{
    company_concept::CompanyConcept,
    company_facts::CompanyFacts,
    frames::XbrlFrames,
    submission::SubmissionHistory,
};
use crate::types::{ApiResponse, Period, Taxonomy, Unit};

/// The `EdgarApi` trait defines methods for accessing the SEC EDGAR API endpoints.
///
/// This trait provides a common interface for accessing the various SEC EDGAR API
/// endpoints, including submission history, company concepts, company facts, and XBRL frames.
#[async_trait]
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
    /// # use edgar_rs::{EdgarApi, DefaultEdgarApi};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let edgar_api = DefaultEdgarApi::new("Your Company Name your.email@example.com");
    /// let submissions = edgar_api.get_submissions_history("0000320193").await?;
    /// println!("Company name: {}", submissions.data.name);
    /// # Ok(())
    /// # }
    /// ```
    async fn get_submissions_history(&self, cik: &str) -> Result<ApiResponse<SubmissionHistory>>;

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
    /// # use edgar_rs::{EdgarApi, DefaultEdgarApi, Taxonomy};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let edgar_api = DefaultEdgarApi::new("Your Company Name your.email@example.com");
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
    /// # use edgar_rs::{EdgarApi, DefaultEdgarApi};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let edgar_api = DefaultEdgarApi::new("Your Company Name your.email@example.com");
    /// let facts = edgar_api.get_company_facts("0000320193").await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn get_company_facts(&self, cik: &str) -> Result<ApiResponse<CompanyFacts>>;

    /// Get XBRL frames data for a specific concept, taxonomy, unit, and period
    ///
    /// Endpoint: https://data.sec.gov/api/xbrl/frames/taxonomy/concept/unit/period.json
    ///
    /// # Parameters
    /// * `taxonomy` - Taxonomy to use (us-gaap, ifrs-full, dei, or srt)
    /// * `concept` - The concept tag to retrieve
    /// * `unit` - Unit of measure
    /// * `period` - Reporting period (annual, quarterly, or instantaneous)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use edgar_rs::{EdgarApi, DefaultEdgarApi, Taxonomy, Unit, Period};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let edgar_api = DefaultEdgarApi::new("Your Company Name your.email@example.com");
    /// let frames = edgar_api.get_xbrl_frames(
    ///     Taxonomy::UsGaap,
    ///     "AccountsPayableCurrent",
    ///     Unit::Simple("USD".to_string()),
    ///     Period::Instantaneous(2019, 1)
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn get_xbrl_frames(
        &self,
        taxonomy: Taxonomy,
        concept: &str,
        unit: Unit,
        period: Period,
    ) -> Result<ApiResponse<XbrlFrames>>;

    /// Download and extract bulk submissions data
    ///
    /// Endpoint: https://www.sec.gov/Archives/edgar/daily-index/bulkdata/submissions.zip
    ///
    /// # Parameters
    /// * `output_path` - Path to store the extracted data
    async fn download_bulk_submissions(&self, output_path: &Path) -> Result<()>;

    /// Download and extract bulk company facts data
    ///
    /// Endpoint: https://www.sec.gov/Archives/edgar/daily-index/xbrl/companyfacts.zip
    ///
    /// # Parameters
    /// * `output_path` - Path to store the extracted data
    async fn download_bulk_company_facts(&self, output_path: &Path) -> Result<()>;
}
