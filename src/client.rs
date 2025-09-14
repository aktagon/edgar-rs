//! Implementation of the `EdgarApi` trait using reqwest.
//!
//! This module provides the `EdgarClient` implementation of the `EdgarApi` trait,
//! which uses reqwest to make HTTP requests to the SEC EDGAR API.

use async_trait::async_trait;
use log::{error, trace, warn};
use reqwest::Client;
use std::path::Path;

use crate::api::EdgarApi;
use crate::error::{EdgarApiError, Result};
use crate::models::{
    company_concept::CompanyConcept, company_facts::CompanyFacts, frames::XbrlFrames,
    submission::{Recent, SubmissionHistory},
};
use crate::rate_limit::RateLimiter;
use crate::types::{ApiResponse, Period, Taxonomy, Unit};
use crate::utils::cik::format_cik;
use crate::utils::download::{extract_zip, write_temp_file};

/// Default implementation of the `EdgarApi` trait using reqwest.
///
/// This struct provides a concrete implementation of the `EdgarApi` trait
/// using reqwest to make HTTP requests to the SEC EDGAR API.
///
/// # Example
///
/// ```rust,no_run
/// use edgar_rs::{EdgarApi, EdgarClient};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let edgar_api = EdgarClient::new("Your Company Name your.email@example.com");
///     let submissions = edgar_api.get_submissions_history("0000320193").await?;
///     println!("Company name: {}", submissions.data.name);
///     Ok(())
/// }
/// ```
pub struct EdgarClient {
    client: Client,
    rate_limiter: RateLimiter,
    user_agent: String,
}

impl EdgarClient {
    /// Creates a new `EdgarClient` instance with the specified user agent.
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
    /// let edgar_api = EdgarClient::new("Your Company Name your.email@example.com");
    /// ```
    pub fn new(user_agent: &str) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Failed to build reqwest client");

        Self {
            client,
            rate_limiter: RateLimiter::new(10, 1), // 10 requests per second
            user_agent: user_agent.to_string(),
        }
    }

    /// Creates a new `EdgarClient` instance with custom settings.
    ///
    /// # Parameters
    ///
    /// * `client` - A custom reqwest Client.
    /// * `user_agent` - The user agent string to use for requests.
    /// * `rate_limit` - Requests per second rate limit.
    pub fn with_client(client: Client, user_agent: &str, rate_limit: u32) -> Self {
        Self {
            client,
            rate_limiter: RateLimiter::new(rate_limit, 1),
            user_agent: user_agent.to_string(),
        }
    }

    /// Makes a GET request to the specified URL.
    async fn get<T>(&self, url: &str) -> Result<ApiResponse<T>>
    where
        T: serde::de::DeserializeOwned,
    {
        trace!("Starting API request to {}", url);

        // Wait for rate limiter
        trace!("Waiting for rate limiter");
        self.rate_limiter.acquire().await;
        trace!("Rate limiter token acquired");

        trace!("Sending GET request to {}", url);
        let response = self
            .client
            .get(url)
            .header("User-Agent", &self.user_agent)
            .send()
            .await
            .map_err(|e| {
                error!("Network error while requesting {}: {}", url, e);
                EdgarApiError::network(e)
            })?;

        let status = response.status().as_u16();
        trace!("Received response from {} with status code {}", url, status);

        // Handle rate limiting
        if status == 429 {
            let retry_after = response
                .headers()
                .get("Retry-After")
                .and_then(|h| h.to_str().ok())
                .and_then(|s| s.parse::<u64>().ok());

            warn!(
                "Rate limited by API (status 429). Retry-After: {:?}",
                retry_after
            );
            return Err(EdgarApiError::rate_limit(retry_after));
        }

        // Handle other errors
        if !response.status().is_success() {
            error!("Request to {} failed with status {}", url, status);
            return Err(EdgarApiError::api(
                status,
                format!("Request to {} failed with status {}", url, status),
            ));
        }

        // Parse response
        trace!("Parsing JSON response from {}", url);
        let data = response.json::<T>().await.map_err(|e| {
            error!("Failed to parse JSON response from {}: {}", url, e);
            EdgarApiError::parse(e)
        })?;

        trace!("Successfully parsed response from {}", url);
        Ok(ApiResponse { status, data })
    }
}

#[async_trait]
impl EdgarApi for EdgarClient {
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

    async fn download_bulk_submissions(&self, output_path: &Path) -> Result<()> {
        let url = "https://www.sec.gov/Archives/edgar/daily-index/bulkdata/submissions.zip";

        // Wait for rate limiter
        self.rate_limiter.acquire().await;

        // Download the zip file
        let response = self
            .client
            .get(url)
            .header("User-Agent", &self.user_agent)
            .send()
            .await
            .map_err(|e| EdgarApiError::network(e))?;

        if !response.status().is_success() {
            return Err(EdgarApiError::api(
                response.status().as_u16(),
                "Failed to download bulk submissions data",
            ));
        }

        // Get the bytes from the response
        let bytes = response
            .bytes()
            .await
            .map_err(|e| EdgarApiError::network(e))?;

        // Write to temp file and extract
        let temp_path = write_temp_file(&bytes)?;
        extract_zip(&temp_path, output_path)?;

        Ok(())
    }

    async fn download_bulk_company_facts(&self, output_path: &Path) -> Result<()> {
        let url = "https://www.sec.gov/Archives/edgar/daily-index/xbrl/companyfacts.zip";

        // Wait for rate limiter
        self.rate_limiter.acquire().await;

        // Download the zip file
        let response = self
            .client
            .get(url)
            .header("User-Agent", &self.user_agent)
            .send()
            .await
            .map_err(|e| EdgarApiError::network(e))?;

        if !response.status().is_success() {
            return Err(EdgarApiError::api(
                response.status().as_u16(),
                "Failed to download bulk company facts data",
            ));
        }

        // Get the bytes from the response
        let bytes = response
            .bytes()
            .await
            .map_err(|e| EdgarApiError::network(e))?;

        // Write to temp file and extract
        let temp_path = write_temp_file(&bytes)?;
        extract_zip(&temp_path, output_path)?;

        Ok(())
    }
}
