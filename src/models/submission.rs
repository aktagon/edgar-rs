//! Models for submission history data.
//!
//! This module contains data models for the SEC EDGAR API submission history responses.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A company's submission history from the SEC EDGAR API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmissionHistory {
    /// The CIK number of the company.
    pub cik: String,

    /// The entity type.
    #[serde(default)]
    #[serde(rename = "entityType")]
    pub entity_type: String,

    /// The SIC code of the company.
    #[serde(default)]
    pub sic: String,

    /// The SIC description of the company.
    #[serde(default)]
    #[serde(rename = "sicDescription")]
    pub sic_description: String,

    /// Whether the company is a insider transaction issuer.
    #[serde(default)]
    #[serde(rename = "insiderTransactionForIssuerExists")]
    pub insider_transaction_for_issuer_exists: u8,

    /// Whether the company is a insider transaction reporter.
    #[serde(default)]
    #[serde(rename = "insiderTransactionForOwnerExists")]
    pub insider_transaction_for_owner_exists: u8,

    /// The name of the company.
    pub name: String,

    /// Alternative names of the company.
    #[serde(default)]
    pub tickers: Vec<String>,

    /// The exchanges the company is listed on.
    #[serde(default)]
    pub exchanges: Vec<String>,

    /// The company's former names.
    #[serde(default)]
    #[serde(rename = "formerNames")]
    pub former_names: Vec<FormerName>,

    /// The company's filing history.
    pub filings: Filings,

    /// Additional JSON files containing filing history.
    #[serde(default)]
    pub files: Option<Vec<FileInfo>>,
}

/// Information about a company's former name.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormerName {
    /// The former name.
    pub name: String,

    /// The date the name was changed from.
    pub from: String,

    /// The date the name was changed to.
    pub to: String,
}

/// A company's filing history.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Filings {
    /// Filing history metadata.
    pub recent: Recent,

    /// Filing history files.
    #[serde(default)]
    pub files: Option<Vec<FileInfo>>,
}

/// Filing history metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recent {
    /// The accession numbers of the filings.
    #[serde(default)]
    #[serde(rename = "accessionNumber")]
    pub accession_number: Vec<String>,

    /// The filing dates of the filings.
    #[serde(default)]
    #[serde(rename = "filingDate")]
    pub filing_date: Vec<String>,

    /// The report dates of the filings.
    #[serde(default)]
    #[serde(rename = "reportDate")]
    pub report_date: Vec<String>,

    /// The acceptance dates and times of the filings.
    #[serde(default)]
    #[serde(rename = "acceptanceDateTime")]
    pub acceptance_date_time: Vec<String>,

    /// The form types of the filings.
    #[serde(default)]
    pub form: Vec<String>,

    /// The primary document for each filing.
    #[serde(default)]
    #[serde(rename = "primaryDocument")]
    pub primary_document: Vec<String>,

    /// The primary document description for each filing.
    #[serde(default)]
    #[serde(rename = "primaryDocDescription")]
    pub primary_doc_description: Vec<String>,

    /// The file numbers of the filings.
    #[serde(default)]
    #[serde(rename = "fileNumber")]
    pub file_number: Vec<String>,

    /// The film numbers of the filings.
    #[serde(default)]
    #[serde(rename = "filmNumber")]
    pub film_number: Vec<String>,

    /// The items referenced in the filings.
    #[serde(default)]
    pub items: Vec<String>,

    /// The size of the complete submission file in bytes.
    #[serde(default)]
    pub size: Vec<i64>,

    /// Indicates whether the filing was submitted via paper.
    #[serde(default)]
    #[serde(rename = "isXBRL")]
    pub is_xbrl: Vec<i64>,

    /// Indicates whether the filing was submitted via paper.
    #[serde(default)]
    #[serde(rename = "isInlineXBRL")]
    pub is_inline_xbrl: Vec<i64>,

    /// Indicates whether the filing was submitted via paper.
    #[serde(default)]
    #[serde(rename = "isPaper")]
    pub is_paper: Vec<i64>,

    /// Instance document URLs.
    #[serde(default)]
    #[serde(rename = "instanceUrl")]
    pub instance_url: Vec<Option<String>>,
}

/// Information about a filing history file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    /// The name of the file.
    pub name: String,

    /// The filings contained in the file.
    #[serde(rename = "filingCount")]
    pub filing_count: i64,

    /// The start date of the filings in the file.
    #[serde(rename = "filingFrom")]
    pub filing_from: String,

    /// The end date of the filings in the file.
    #[serde(rename = "filingTo")]
    pub filing_to: String,
}

/// Helper methods for the SubmissionHistory struct.
impl SubmissionHistory {
    /// Returns a list of recent filings from the submission history.
    ///
    /// This method returns recent filings from the `recent` field, limited to
    /// the most recent 1000 filings. For access to all filings including older
    /// ones, use `get_all_filings()` instead.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use edgar_rs::{EdgarApi, EdgarClient};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let edgar_api = EdgarClient::new("Your Company Name your.email@example.com")?;
    /// let submissions = edgar_api.get_submissions_history("0000320193").await?;
    /// let filings = submissions.data.get_recent_filings();
    /// for filing in filings {
    ///     println!("Form: {}, Filing Date: {}", filing.form, filing.filing_date);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_recent_filings(&self) -> Vec<FilingEntry> {
        Self::convert_recent_to_filing_entries(&self.filings.recent)
    }

    /// Returns a comprehensive list of all filings including those from paginated files.
    ///
    /// This method fetches additional filing files if they exist and returns
    /// a complete list of all filings for the company. This includes recent filings
    /// plus any older filings stored in separate paginated files.
    ///
    /// # Parameters
    /// * `api_client` - An instance of EdgarApi to fetch additional files
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use edgar_rs::{EdgarApi, EdgarClient};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let edgar_api = EdgarClient::new("Your Company Name your.email@example.com")?;
    /// let submissions = edgar_api.get_submissions_history("0001067983").await?;
    /// let all_filings = submissions.data.get_all_filings(&edgar_api).await?;
    /// println!("Total filings: {}", all_filings.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_all_filings<T: crate::api::EdgarApi>(
        &self,
        api_client: &T,
    ) -> crate::error::Result<Vec<FilingEntry>> {
        let mut all_filings = self.get_recent_filings();

        if let Some(files) = &self.filings.files {
            for file_info in files {
                let additional_filings = api_client.get_submissions_file(&file_info.name).await?;
                let additional_entries = Self::convert_recent_to_filing_entries(&additional_filings.data);
                all_filings.extend(additional_entries);
            }
        }

        Ok(all_filings)
    }

    /// Convert a Recent struct to a vector of FilingEntry
    fn convert_recent_to_filing_entries(recent: &Recent) -> Vec<FilingEntry> {
        let mut entries = Vec::new();

        for i in 0..recent.accession_number.len() {
            if i < recent.form.len() && i < recent.filing_date.len() {
                entries.push(FilingEntry {
                    accession_number: recent.accession_number.get(i).cloned().unwrap_or_default(),
                    filing_date: recent.filing_date.get(i).cloned().unwrap_or_default(),
                    report_date: recent.report_date.get(i).cloned().unwrap_or_default(),
                    acceptance_date_time: recent
                        .acceptance_date_time
                        .get(i)
                        .cloned()
                        .unwrap_or_default(),
                    form: recent.form.get(i).cloned().unwrap_or_default(),
                    primary_document: recent.primary_document.get(i).cloned().unwrap_or_default(),
                    primary_doc_description: recent
                        .primary_doc_description
                        .get(i)
                        .cloned()
                        .unwrap_or_default(),
                    file_number: recent.file_number.get(i).cloned().unwrap_or_default(),
                    film_number: recent.film_number.get(i).cloned().unwrap_or_default(),
                    items: recent.items.get(i).cloned().unwrap_or_default(),
                    size: recent.size.get(i).cloned().unwrap_or(0),
                    is_xbrl: recent.is_xbrl.get(i).cloned().unwrap_or(0) == 1,
                    is_inline_xbrl: recent.is_inline_xbrl.get(i).cloned().unwrap_or(0) == 1,
                    is_paper: recent.is_paper.get(i).cloned().unwrap_or(0) == 1,
                    instance_url: recent.instance_url.get(i).cloned().unwrap_or(None),
                });
            }
        }

        entries
    }

    /// Returns a map of ticker symbols to exchange names.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use edgar_rs::{EdgarApi, EdgarClient};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let edgar_api = EdgarClient::new("Your Company Name your.email@example.com")?;
    /// let submissions = edgar_api.get_submissions_history("0000320193").await?;
    /// let ticker_map = submissions.data.get_ticker_map();
    /// for (ticker, exchange) in ticker_map {
    ///     println!("Ticker: {} on Exchange: {}", ticker, exchange);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_ticker_map(&self) -> HashMap<String, String> {
        let mut ticker_map = HashMap::new();

        if self.tickers.len() == self.exchanges.len() {
            for (i, ticker) in self.tickers.iter().enumerate() {
                if let Some(exchange) = self.exchanges.get(i) {
                    ticker_map.insert(ticker.clone(), exchange.clone());
                }
            }
        }

        ticker_map
    }
}

/// A filing entry in a company's submission history. NOTE: The data in the Recent struct is copied
/// to Vec<FilingEntry>. This could be optimized later.
#[derive(Debug, Clone)]
pub struct FilingEntry {
    /// The accession number of the filing.
    pub accession_number: String,

    /// The filing date of the filing.
    pub filing_date: String,

    /// The report date of the filing.
    pub report_date: String,

    /// The acceptance date and time of the filing.
    pub acceptance_date_time: String,

    /// The form type of the filing.
    pub form: String,

    /// The primary document for the filing.
    pub primary_document: String,

    /// The primary document description for the filing.
    pub primary_doc_description: String,

    /// The file number of the filing.
    pub file_number: String,

    /// The film number of the filing.
    pub film_number: String,

    /// The items referenced in the filing.
    pub items: String,

    /// The size of the complete submission file in bytes.
    pub size: i64,

    /// Indicates whether the filing is in XBRL format.
    pub is_xbrl: bool,

    /// Indicates whether the filing is in inline XBRL format.
    pub is_inline_xbrl: bool,

    /// Indicates whether the filing was submitted via paper.
    pub is_paper: bool,

    /// Instance document URL.
    pub instance_url: Option<String>,
}
