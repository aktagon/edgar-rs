//! Implementation of the `EdgarApi` trait using HTTP client abstraction.
//!
//! This module provides the `EdgarClient` implementation of the `EdgarApi` trait,
//! which uses the HTTP client abstraction to support multiple runtimes.

use async_trait::async_trait;
use log::{error, trace};
#[cfg(feature = "native")]
use std::path::Path;
use crate::api::EdgarApi;
use crate::error::{EdgarApiError, Result};
#[cfg(feature = "native")]
use crate::http::HttpClient;
#[cfg(feature = "cloudflare-workers")]
use crate::http::HttpClient;
use crate::models::{
    company_concept::CompanyConcept, company_facts::CompanyFacts, company_tickers::CompanyTickers,
    company_tickers_mf::CompanyTickersMf, frames::XbrlFrames, submission::{Recent, SubmissionHistory},
};
use crate::types::{ApiResponse, Period, Taxonomy, Unit};
use crate::utils::cik::format_cik;
#[cfg(feature = "native")]
use crate::utils::download::{extract_zip, write_temp_file};

/// Implementation of the `EdgarApi` trait using HTTP client abstraction.
///
/// This struct provides a concrete implementation of the `EdgarApi` trait
/// that can work with different HTTP clients for different runtimes.
///
/// # Example
///
/// ```rust,no_run
/// use edgar_rs::{EdgarApi, EdgarClient};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let edgar_api = EdgarClient::new("Your Company Name your.email@example.com")?;
///     let submissions = edgar_api.get_submissions_history("0000320193").await?;
///     println!("Company name: {}", submissions.data.name);
///     Ok(())
/// }
/// ```
pub struct EdgarClient<H: HttpClient> {
    http_client: H,
    user_agent: String,
}

impl<H: HttpClient> EdgarClient<H> {
    /// Creates a new `EdgarClient` instance with a custom HTTP client.
    ///
    /// # Parameters
    ///
    /// * `http_client` - A custom HTTP client implementation.
    /// * `user_agent` - The user agent string to use for requests.
    pub fn with_client(http_client: H, user_agent: &str) -> Self {
        Self {
            http_client,
            user_agent: user_agent.to_string(),
        }
    }

    /// Makes a GET request to the specified URL.
    async fn get<T>(&self, url: &str) -> Result<ApiResponse<T>>
    where
        T: serde::de::DeserializeOwned,
    {
        trace!("Starting API request to {}", url);

        let headers = [("User-Agent", self.user_agent.as_str())];

        let response = self.http_client.get(url, &headers).await?;
        let status = response.status;

        // Handle rate limiting
        if status == 429 {
            let retry_after = response.headers
                .get("retry-after")
                .and_then(|s| s.parse::<u64>().ok());

            error!(
                "Rate limited by API (status 429). Retry-After: {:?}",
                retry_after
            );
            return Err(EdgarApiError::rate_limit(retry_after));
        }

        // Handle other errors
        if !response.is_success() {
            error!("Request to {} failed with status {}", url, status);
            return Err(EdgarApiError::api(
                status,
                format!("Request to {} failed with status {}", url, status),
            ));
        }

        // Parse response
        trace!("Parsing JSON response from {}", url);
        let data = response.json::<T>()?;

        trace!("Successfully parsed response from {}", url);
        Ok(ApiResponse { status, data })
    }
}

// Native specific implementations
#[cfg(feature = "native")]
impl EdgarClient<crate::http::ReqwestClient> {
    /// Creates a new `EdgarClient` instance with the default native HTTP client and specified user agent.
    ///
    /// # Parameters
    ///
    /// * `user_agent` - The user agent string to use for requests. As per SEC guidelines,
    ///   this should include your company name and contact email.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use edgar_rs::EdgarClient;
    ///
    /// fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let edgar_api = EdgarClient::new("Your Company Name your.email@example.com")?;
    ///     Ok(())
    /// }
    /// ```
    pub fn new(user_agent: &str) -> Result<Self> {
        use crate::http::ReqwestClient;

        let http_client = ReqwestClient::new()?;
        Ok(Self {
            http_client,
            user_agent: user_agent.to_string(),
        })
    }
}

// Cloudflare Workers specific implementations
#[cfg(feature = "cloudflare-workers")]
impl EdgarClient<crate::http::WorkerClient> {
    /// Creates a new `EdgarClient` instance for Cloudflare Workers.
    ///
    /// # Parameters
    ///
    /// * `user_agent` - The user agent string to use for requests.
    pub fn new_worker(user_agent: &str) -> Self {
        use crate::http::WorkerClient;

        let http_client = WorkerClient::new();
        Self {
            http_client,
            user_agent: user_agent.to_string(),
        }
    }
}

#[async_trait]
#[cfg(feature = "native")]
impl<H: HttpClient> EdgarApi for EdgarClient<H> {
    async fn get_submissions_history(&self, cik: &str) -> Result<ApiResponse<SubmissionHistory>> {
        let formatted_cik = format_cik(cik)?;
        let url = format!("https://data.sec.gov/submissions/CIK{}.json", formatted_cik);
        trace!("Fetching submissions history for CIK: {}", formatted_cik);

        self.get(&url).await
    }

    async fn get_submissions_file(&self, filename: &str) -> Result<ApiResponse<Recent>> {
        let url = format!("https://data.sec.gov/submissions/{}", filename);
        trace!("Fetching submissions file: {}", filename);

        self.get(&url).await
    }

    async fn get_company_concept(
        &self,
        cik: &str,
        taxonomy: Taxonomy,
        tag: &str,
    ) -> Result<ApiResponse<CompanyConcept>> {
        let formatted_cik = format_cik(cik)?;
        let url = format!(
            "https://data.sec.gov/api/xbrl/companyconcept/CIK{}/{}/{}.json",
            formatted_cik,
            taxonomy.as_str(),
            tag
        );
        trace!("Fetching company concept for CIK: {}, taxonomy: {}, tag: {}", formatted_cik, taxonomy.as_str(), tag);

        self.get(&url).await
    }

    async fn get_company_facts(&self, cik: &str) -> Result<ApiResponse<CompanyFacts>> {
        let formatted_cik = format_cik(cik)?;
        let url = format!("https://data.sec.gov/api/xbrl/companyfacts/CIK{}.json", formatted_cik);
        trace!("Fetching company facts for CIK: {}", formatted_cik);

        self.get(&url).await
    }

    async fn get_xbrl_frames(
        &self,
        taxonomy: Taxonomy,
        tag: &str,
        unit: Unit,
        period: Period,
    ) -> Result<ApiResponse<XbrlFrames>> {
        let url = format!(
            "https://data.sec.gov/api/xbrl/frames/{}/{}/{}/{}.json",
            taxonomy.as_str(),
            tag,
            unit.as_str(),
            period.as_str()
        );
        trace!("Fetching XBRL frames for taxonomy: {}, tag: {}, unit: {}, period: {}", taxonomy.as_str(), tag, unit, period);

        self.get(&url).await
    }

    async fn get_company_tickers(&self) -> Result<ApiResponse<CompanyTickers>> {
        let url = "https://www.sec.gov/files/company_tickers_exchange.json";
        trace!("Fetching company tickers exchange data");

        self.get(url).await
    }

    async fn get_company_tickers_mf(&self) -> Result<ApiResponse<CompanyTickersMf>> {
        let url = "https://www.sec.gov/files/company_tickers_mf.json";
        trace!("Fetching mutual fund tickers data");

        self.get(url).await
    }

    async fn download_bulk_submissions(&self, output_path: &str) -> Result<()> {
        let url = "https://www.sec.gov/Archives/edgar/daily-index/bulkdata/submissions.zip";

        trace!("Downloading bulk submissions from: {}", url);

        let headers = [
            ("User-Agent", self.user_agent.as_str()),
            ("Accept", "application/zip"),
        ];

        // Download the ZIP file
        let data = self.http_client.get_bytes(url, &headers).await?;
        trace!("Downloaded bulk submissions: {} bytes", data.len());

        // Write to temporary file
        let temp_file = write_temp_file(&data)?;
        trace!("Wrote bulk submissions to temp file: {}", temp_file.display());

        // Extract the ZIP file
        extract_zip(&temp_file, Path::new(output_path))?;
        trace!("Extracted bulk submissions to: {}", output_path);

        Ok(())
    }

    async fn download_bulk_company_facts(&self, output_path: &str) -> Result<()> {
        let url = "https://www.sec.gov/Archives/edgar/daily-index/bulkdata/companyfacts.zip";

        trace!("Downloading bulk company facts from: {}", url);

        let headers = [
            ("User-Agent", self.user_agent.as_str()),
            ("Accept", "application/zip"),
        ];

        // Download the ZIP file
        let data = self.http_client.get_bytes(url, &headers).await?;
        trace!("Downloaded bulk company facts: {} bytes", data.len());

        // Write to temporary file
        let temp_file = write_temp_file(&data)?;
        trace!("Wrote bulk company facts to temp file: {}", temp_file.display());

        // Extract the ZIP file
        extract_zip(&temp_file, Path::new(output_path))?;
        trace!("Extracted bulk company facts to: {}", output_path);

        Ok(())
    }

    async fn extract_zip_files(&self, zip_path: &Path, output_dir: &Path) -> Result<()> {
        extract_zip(zip_path, output_dir)
    }
}

#[async_trait(?Send)]
#[cfg(feature = "cloudflare-workers")]
impl<H: HttpClient> EdgarApi for EdgarClient<H> {
    async fn get_submissions_history(&self, cik: &str) -> Result<ApiResponse<SubmissionHistory>> {
        let formatted_cik = format_cik(cik).map_err(|_| EdgarApiError::invalid_cik(cik))?;
        let url = format!("https://data.sec.gov/submissions/CIK{}.json", formatted_cik);
        self.get(&url).await
    }

    async fn get_submissions_file(&self, filename: &str) -> Result<ApiResponse<Recent>> {
        let url = format!("https://data.sec.gov/submissions/{}", filename);
        self.get(&url).await
    }

    async fn get_company_concept(
        &self,
        cik: &str,
        taxonomy: Taxonomy,
        tag: &str,
    ) -> Result<ApiResponse<CompanyConcept>> {
        let formatted_cik = format_cik(cik).map_err(|_| EdgarApiError::invalid_cik(cik))?;
        let url = format!(
            "https://data.sec.gov/api/xbrl/companyconcept/CIK{}/{}/{}.json",
            formatted_cik,
            taxonomy.as_str(),
            tag
        );
        self.get(&url).await
    }

    async fn get_company_facts(&self, cik: &str) -> Result<ApiResponse<CompanyFacts>> {
        let formatted_cik = format_cik(cik).map_err(|_| EdgarApiError::invalid_cik(cik))?;
        let url = format!(
            "https://data.sec.gov/api/xbrl/companyfacts/CIK{}.json",
            formatted_cik
        );
        self.get(&url).await
    }

    async fn get_xbrl_frames(
        &self,
        taxonomy: Taxonomy,
        concept: &str,
        unit: Unit,
        period: Period,
    ) -> Result<ApiResponse<XbrlFrames>> {
        let url = format!(
            "https://data.sec.gov/api/xbrl/frames/{}/{}/{}/{}.json",
            taxonomy.as_str(),
            concept,
            unit.as_str(),
            period.as_str()
        );
        self.get(&url).await
    }

    async fn get_company_tickers(&self) -> Result<ApiResponse<CompanyTickers>> {
        let url = "https://www.sec.gov/files/company_tickers_exchange.json";
        self.get(url).await
    }

    async fn get_company_tickers_mf(&self) -> Result<ApiResponse<CompanyTickersMf>> {
        let url = "https://www.sec.gov/files/company_tickers_mf.json";
        self.get(url).await
    }

    #[cfg(feature = "native")]
    async fn download_bulk_submissions(&self, output_path: &Path) -> Result<()> {
        let url = "https://www.sec.gov/Archives/edgar/daily-index/bulkdata/submissions.zip";
        let headers = [("User-Agent", self.user_agent.as_str())];

        // Download the zip file
        let bytes = self.http_client.get_bytes(url, &headers).await?;

        // Write to temp file and extract
        let temp_path = write_temp_file(&bytes)?;
        extract_zip(&temp_path, output_path)?;

        Ok(())
    }


    #[cfg(feature = "native")]
    async fn download_bulk_company_facts(&self, output_path: &Path) -> Result<()> {
        let url = "https://www.sec.gov/Archives/edgar/daily-index/xbrl/companyfacts.zip";
        let headers = [("User-Agent", self.user_agent.as_str())];

        // Download the zip file
        let bytes = self.http_client.get_bytes(url, &headers).await?;

        // Write to temp file and extract
        let temp_path = write_temp_file(&bytes)?;
        extract_zip(&temp_path, output_path)?;

        Ok(())
    }

}
