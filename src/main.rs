// EDGAR API trait definition
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Clone)]
pub struct Config {
    pub base_url: String,
    pub user_agent: String,
}

impl Config {
    pub fn new(user_agent: &str) -> Self {
        Self {
            base_url: "https://".to_string(),
            user_agent: user_agent.to_string(),
        }
    }

    pub fn build_url(&self, url: &str) -> String {
        if url.starts_with("https://") {
            format!("{}{}", self.base_url, &url[8..])
        } else {
            url.to_string()
        }
    }
}

// Common response type for API calls
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub data: T,
    pub status: u16,
}

// Error type for API calls
#[derive(Debug)]
pub enum EdgarApiError {
    NetworkError(String),
    ParseError(String),
    RequestError(String),
    ApiError { status: u16, message: String },
}

impl std::fmt::Display for EdgarApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NetworkError(msg) => write!(f, "Network error: {}", msg),
            Self::ParseError(msg) => write!(f, "Parse error: {}", msg),
            Self::RequestError(msg) => write!(f, "Request error: {}", msg),
            Self::ApiError { status, message } => write!(f, "API error {}: {}", status, message),
        }
    }
}

impl Error for EdgarApiError {}

// Types for parameters used in API calls
#[derive(Debug, Clone, Copy)]
pub enum Taxonomy {
    UsGaap,
    IfrsFull,
    Dei,
    Srt,
}

impl Taxonomy {
    pub fn as_str(&self) -> &'static str {
        match self {
            Taxonomy::UsGaap => "us-gaap",
            Taxonomy::IfrsFull => "ifrs-full",
            Taxonomy::Dei => "dei",
            Taxonomy::Srt => "srt",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Period {
    Annual(u16),            // CY####
    Quarterly(u16, u8),     // CY####Q#
    Instantaneous(u16, u8), // CY####Q#I
}

impl Period {
    pub fn as_str(&self) -> String {
        match self {
            Period::Annual(year) => format!("CY{}", year),
            Period::Quarterly(year, quarter) => format!("CY{}Q{}", year, quarter),
            Period::Instantaneous(year, quarter) => format!("CY{}Q{}I", year, quarter),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Unit {
    Simple(String),           // e.g., "USD", "pure"
    Compound(String, String), // e.g., "USD-per-shares"
}

impl Unit {
    pub fn as_str(&self) -> String {
        match self {
            Unit::Simple(unit) => unit.clone(),
            Unit::Compound(numerator, denominator) => format!("{}-per-{}", numerator, denominator),
        }
    }
}

// Main EDGAR API trait
#[async_trait]
pub trait EdgarApi {
    /// Get company's submissions history
    ///
    /// Endpoint: https://data.sec.gov/submissions/CIK##########.json
    ///
    /// # Parameters
    /// * `cik` - 10-digit Central Index Key, including leading zeros
    async fn get_submissions_history(
        &self,
        cik: &str,
    ) -> Result<ApiResponse<serde_json::Value>, EdgarApiError>;

    /// Get company data for a specific concept and taxonomy
    ///
    /// Endpoint: https://data.sec.gov/api/xbrl/companyconcept/CIK##########/taxonomy/tag.json
    ///
    /// # Parameters
    /// * `cik` - 10-digit Central Index Key, including leading zeros
    /// * `taxonomy` - Taxonomy to use (us-gaap, ifrs-full, dei, or srt)
    /// * `tag` - The concept tag to retrieve
    async fn get_company_concept(
        &self,
        cik: &str,
        taxonomy: Taxonomy,
        tag: &str,
    ) -> Result<ApiResponse<serde_json::Value>, EdgarApiError>;

    /// Get all company facts for a specific company
    ///
    /// Endpoint: https://data.sec.gov/api/xbrl/companyfacts/CIK##########.json
    ///
    /// # Parameters
    /// * `cik` - 10-digit Central Index Key, including leading zeros
    async fn get_company_facts(
        &self,
        cik: &str,
    ) -> Result<ApiResponse<serde_json::Value>, EdgarApiError>;

    /// Get XBRL frames data for a specific concept, taxonomy, unit, and period
    ///
    /// Endpoint: https://data.sec.gov/api/xbrl/frames/taxonomy/concept/unit/period.json
    ///
    /// # Parameters
    /// * `taxonomy` - Taxonomy to use (us-gaap, ifrs-full, dei, or srt)
    /// * `concept` - The concept tag to retrieve
    /// * `unit` - Unit of measure
    /// * `period` - Reporting period (annual, quarterly, or instantaneous)
    async fn get_xbrl_frames(
        &self,
        taxonomy: Taxonomy,
        concept: &str,
        unit: Unit,
        period: Period,
    ) -> Result<ApiResponse<serde_json::Value>, EdgarApiError>;

    /// Download and extract bulk submissions data
    ///
    /// Endpoint: https://www.sec.gov/Archives/edgar/daily-index/bulkdata/submissions.zip
    ///
    /// # Parameters
    /// * `output_path` - Path to store the extracted data
    async fn download_bulk_submissions(
        &self,
        output_path: &std::path::Path,
    ) -> Result<(), EdgarApiError>;

    /// Download and extract bulk company facts data
    ///
    /// Endpoint: https://www.sec.gov/Archives/edgar/daily-index/xbrl/companyfacts.zip
    ///
    /// # Parameters
    /// * `output_path` - Path to store the extracted data
    async fn download_bulk_company_facts(
        &self,
        output_path: &std::path::Path,
    ) -> Result<(), EdgarApiError>;
}

// A default implementation of the EdgarApi trait
pub struct EdgarClient {
    client: reqwest::Client,
    config: Config,
}

impl EdgarClient {
    pub fn new(config: Config) -> Self {
        // As per SEC guidelines, a user agent with contact info is required
        // The user_agent should be in format "Company Name AdminContact@example.com"
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Failed to build reqwest client");

        Self {
            client,
            config,
        }
    }

    fn format_cik(&self, cik: &str) -> String {
        // Ensure CIK is 10 digits with leading zeros
        format!("{:010}", cik.parse::<u64>().unwrap_or(0))
    }
}

#[async_trait]
impl EdgarApi for EdgarClient {
    async fn get_submissions_history(
        &self,
        cik: &str,
    ) -> Result<ApiResponse<serde_json::Value>, EdgarApiError> {
        let formatted_cik = self.format_cik(cik);
        let url = format!("https://data.sec.gov/submissions/CIK{}.json", formatted_cik);
        let final_url = self.config.build_url(&url);

        let response = self
            .client
            .get(&final_url)
            .header("User-Agent", &self.config.user_agent)
            .send()
            .await
            .map_err(|e| EdgarApiError::NetworkError(e.to_string()))?;

        let status = response.status().as_u16();
        if !response.status().is_success() {
            return Err(EdgarApiError::ApiError {
                status,
                message: format!(
                    "Failed to get submissions history for CIK {}",
                    formatted_cik
                ),
            });
        }

        let data = response
            .json::<serde_json::Value>()
            .await
            .map_err(|e| EdgarApiError::ParseError(e.to_string()))?;

        Ok(ApiResponse { data, status })
    }

    async fn get_company_concept(
        &self,
        cik: &str,
        taxonomy: Taxonomy,
        tag: &str,
    ) -> Result<ApiResponse<serde_json::Value>, EdgarApiError> {
        let formatted_cik = self.format_cik(cik);
        let url = format!(
            "https://data.sec.gov/api/xbrl/companyconcept/CIK{}/{}/{}.json",
            formatted_cik,
            taxonomy.as_str(),
            tag
        );

        let response = self
            .client
            .get(&url)
            .header("User-Agent", &self.config.user_agent)
            .send()
            .await
            .map_err(|e| EdgarApiError::NetworkError(e.to_string()))?;

        let status = response.status().as_u16();
        if !response.status().is_success() {
            return Err(EdgarApiError::ApiError {
                status,
                message: format!(
                    "Failed to get company concept for CIK {} and tag {}",
                    formatted_cik, tag
                ),
            });
        }

        let data = response
            .json::<serde_json::Value>()
            .await
            .map_err(|e| EdgarApiError::ParseError(e.to_string()))?;

        Ok(ApiResponse { data, status })
    }

    async fn get_company_facts(
        &self,
        cik: &str,
    ) -> Result<ApiResponse<serde_json::Value>, EdgarApiError> {
        let formatted_cik = self.format_cik(cik);
        let url = format!(
            "https://data.sec.gov/api/xbrl/companyfacts/CIK{}.json",
            formatted_cik
        );

        let response = self
            .client
            .get(&url)
            .header("User-Agent", &self.config.user_agent)
            .send()
            .await
            .map_err(|e| EdgarApiError::NetworkError(e.to_string()))?;

        let status = response.status().as_u16();
        if !response.status().is_success() {
            return Err(EdgarApiError::ApiError {
                status,
                message: format!("Failed to get company facts for CIK {}", formatted_cik),
            });
        }

        let data = response
            .json::<serde_json::Value>()
            .await
            .map_err(|e| EdgarApiError::ParseError(e.to_string()))?;

        Ok(ApiResponse { data, status })
    }

    async fn get_xbrl_frames(
        &self,
        taxonomy: Taxonomy,
        concept: &str,
        unit: Unit,
        period: Period,
    ) -> Result<ApiResponse<serde_json::Value>, EdgarApiError> {
        let url = format!(
            "https://data.sec.gov/api/xbrl/frames/{}/{}/{}/{}.json",
            taxonomy.as_str(),
            concept,
            unit.as_str(),
            period.as_str()
        );

        let response = self
            .client
            .get(&url)
            .header("User-Agent", &self.config.user_agent)
            .send()
            .await
            .map_err(|e| EdgarApiError::NetworkError(e.to_string()))?;

        let status = response.status().as_u16();
        if !response.status().is_success() {
            return Err(EdgarApiError::ApiError {
                status,
                message: format!("Failed to get XBRL frames for concept {}", concept),
            });
        }

        let data = response
            .json::<serde_json::Value>()
            .await
            .map_err(|e| EdgarApiError::ParseError(e.to_string()))?;

        Ok(ApiResponse { data, status })
    }

    async fn download_bulk_submissions(
        &self,
        output_path: &std::path::Path,
    ) -> Result<(), EdgarApiError> {
        let url = "https://www.sec.gov/Archives/edgar/daily-index/bulkdata/submissions.zip";

        // Download the zip file
        let response = self
            .client
            .get(url)
            .header("User-Agent", &self.config.user_agent)
            .send()
            .await
            .map_err(|e| EdgarApiError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(EdgarApiError::ApiError {
                status: response.status().as_u16(),
                message: "Failed to download bulk submissions data".to_string(),
            });
        }

        // Get the bytes from the response
        let bytes = response
            .bytes()
            .await
            .map_err(|e| EdgarApiError::NetworkError(e.to_string()))?;

        // Create a temporary file to store the zip
        let temp_file = tempfile::NamedTempFile::new()
            .map_err(|e| EdgarApiError::RequestError(e.to_string()))?;

        // Write the zip file to the temporary file
        std::fs::write(temp_file.path(), &bytes)
            .map_err(|e| EdgarApiError::RequestError(e.to_string()))?;

        // Extract the zip file to the output path
        let file = std::fs::File::open(temp_file.path())
            .map_err(|e| EdgarApiError::RequestError(e.to_string()))?;

        let mut archive =
            zip::ZipArchive::new(file).map_err(|e| EdgarApiError::ParseError(e.to_string()))?;

        for i in 0..archive.len() {
            let mut file = archive
                .by_index(i)
                .map_err(|e| EdgarApiError::ParseError(e.to_string()))?;

            let outpath = output_path.join(file.name());

            if file.name().ends_with('/') {
                std::fs::create_dir_all(&outpath)
                    .map_err(|e| EdgarApiError::RequestError(e.to_string()))?;
            } else {
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        std::fs::create_dir_all(p)
                            .map_err(|e| EdgarApiError::RequestError(e.to_string()))?;
                    }
                }

                let mut outfile = std::fs::File::create(&outpath)
                    .map_err(|e| EdgarApiError::RequestError(e.to_string()))?;

                std::io::copy(&mut file, &mut outfile)
                    .map_err(|e| EdgarApiError::RequestError(e.to_string()))?;
            }
        }

        Ok(())
    }

    async fn download_bulk_company_facts(
        &self,
        output_path: &std::path::Path,
    ) -> Result<(), EdgarApiError> {
        let url = "https://www.sec.gov/Archives/edgar/daily-index/xbrl/companyfacts.zip";

        // Download the zip file
        let response = self
            .client
            .get(url)
            .header("User-Agent", &self.config.user_agent)
            .send()
            .await
            .map_err(|e| EdgarApiError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(EdgarApiError::ApiError {
                status: response.status().as_u16(),
                message: "Failed to download bulk company facts data".to_string(),
            });
        }

        // Get the bytes from the response
        let bytes = response
            .bytes()
            .await
            .map_err(|e| EdgarApiError::NetworkError(e.to_string()))?;

        // Create a temporary file to store the zip
        let temp_file = tempfile::NamedTempFile::new()
            .map_err(|e| EdgarApiError::RequestError(e.to_string()))?;

        // Write the zip file to the temporary file
        std::fs::write(temp_file.path(), &bytes)
            .map_err(|e| EdgarApiError::RequestError(e.to_string()))?;

        // Extract the zip file to the output path
        let file = std::fs::File::open(temp_file.path())
            .map_err(|e| EdgarApiError::RequestError(e.to_string()))?;

        let mut archive =
            zip::ZipArchive::new(file).map_err(|e| EdgarApiError::ParseError(e.to_string()))?;

        for i in 0..archive.len() {
            let mut file = archive
                .by_index(i)
                .map_err(|e| EdgarApiError::ParseError(e.to_string()))?;

            let outpath = output_path.join(file.name());

            if file.name().ends_with('/') {
                std::fs::create_dir_all(&outpath)
                    .map_err(|e| EdgarApiError::RequestError(e.to_string()))?;
            } else {
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        std::fs::create_dir_all(p)
                            .map_err(|e| EdgarApiError::RequestError(e.to_string()))?;
                    }
                }

                let mut outfile = std::fs::File::create(&outpath)
                    .map_err(|e| EdgarApiError::RequestError(e.to_string()))?;

                std::io::copy(&mut file, &mut outfile)
                    .map_err(|e| EdgarApiError::RequestError(e.to_string()))?;
            }
        }

        Ok(())
    }
}

// Example usage of the trait
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the API client with a user agent
    let config = Config::new("Example Corp example@example.com");
    let edgar_api = EdgarClient::new(config);

    // Get submissions history for Apple Inc. (CIK: 0000320193)
    let submissions = edgar_api.get_submissions_history("0000320193").await?;
    println!("Apple Inc. submissions status: {}", submissions.status);

    // Get company concept for Apple Inc. and the AccountsPayableCurrent tag
    let concept = edgar_api
        .get_company_concept("0000320193", Taxonomy::UsGaap, "AccountsPayableCurrent")
        .await?;
    println!(
        "Apple Inc. AccountsPayableCurrent status: {}",
        concept.status
    );

    // Get all company facts for Apple Inc.
    let facts = edgar_api.get_company_facts("0000320193").await?;
    println!("Apple Inc. all facts status: {}", facts.status);

    // Get XBRL frames for AccountsPayableCurrent in USD for Q1 2019
    let frames = edgar_api
        .get_xbrl_frames(
            Taxonomy::UsGaap,
            "AccountsPayableCurrent",
            Unit::Simple("USD".to_string()),
            Period::Instantaneous(2019, 1),
        )
        .await?;
    println!("XBRL frames status: {}", frames.status);

    // Create a temp directory for bulk downloads
    let temp_dir = tempfile::tempdir()?;

    // Download bulk submissions data
    edgar_api.download_bulk_submissions(temp_dir.path()).await?;
    println!("Bulk submissions downloaded to: {:?}", temp_dir.path());

    // Download bulk company facts data
    edgar_api
        .download_bulk_company_facts(temp_dir.path())
        .await?;
    println!("Bulk company facts downloaded to: {:?}", temp_dir.path());

    Ok(())
}
