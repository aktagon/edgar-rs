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
    pub entityType: String,

    /// The SIC code of the company.
    #[serde(default)]
    pub sic: String,

    /// The SIC description of the company.
    #[serde(default)]
    pub sicDescription: String,

    /// Whether the company is a insider transaction issuer.
    #[serde(default)]
    pub insiderTransactionForIssuerExists: u8,

    /// Whether the company is a insider transaction reporter.
    #[serde(default)]
    pub insiderTransactionForOwnerExists: u8,

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
    pub formerNames: Vec<FormerName>,

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
    pub accessionNumber: Vec<String>,

    /// The filing dates of the filings.
    #[serde(default)]
    pub filingDate: Vec<String>,

    /// The report dates of the filings.
    #[serde(default)]
    pub reportDate: Vec<String>,

    /// The acceptance dates and times of the filings.
    #[serde(default)]
    pub acceptanceDateTime: Vec<String>,

    /// The form types of the filings.
    #[serde(default)]
    pub form: Vec<String>,

    /// The primary document for each filing.
    #[serde(default)]
    pub primaryDocument: Vec<String>,

    /// The primary document description for each filing.
    #[serde(default)]
    pub primaryDocDescription: Vec<String>,

    /// The file numbers of the filings.
    #[serde(default)]
    pub fileNumber: Vec<String>,

    /// The film numbers of the filings.
    #[serde(default)]
    pub filmNumber: Vec<String>,

    /// The items referenced in the filings.
    #[serde(default)]
    pub items: Vec<String>,

    /// The size of the complete submission file in bytes.
    #[serde(default)]
    pub size: Vec<i64>,

    /// Indicates whether the filing was submitted via paper.
    #[serde(default)]
    pub isXBRL: Vec<i64>,

    /// Indicates whether the filing was submitted via paper.
    #[serde(default)]
    pub isInlineXBRL: Vec<i64>,

    /// Indicates whether the filing was submitted via paper.
    #[serde(default)]
    pub isPaper: Vec<i64>,

    /// Instance document URLs.
    #[serde(default)]
    pub instanceUrl: Vec<Option<String>>,
}

/// Information about a filing history file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    /// The name of the file.
    pub name: String,

    /// The filings contained in the file.
    pub filingCount: i64,

    /// The start date of the filings in the file.
    pub filingFrom: String,

    /// The end date of the filings in the file.
    pub filingTo: String,
}

/// Helper methods for the SubmissionHistory struct.
impl SubmissionHistory {
    /// Returns a list of all filings in the submission history.
    ///
    /// This method returns a list of all filings in the submission history,
    /// including those in the `recent` field and those in any referenced files.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use edgar_rs::{EdgarApi, DefaultEdgarApi};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let edgar_api = DefaultEdgarApi::new("Your Company Name your.email@example.com");
    /// let submissions = edgar_api.get_submissions_history("0000320193").await?;
    /// let filings = submissions.data.get_all_filings();
    /// for filing in filings {
    ///     println!("Form: {}, Filing Date: {}", filing.form, filing.filing_date);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_all_filings(&self) -> Vec<FilingEntry> {
        let recent = &self.filings.recent;
        let mut entries = Vec::new();

        // Process the recent filings
        for i in 0..recent.accessionNumber.len() {
            if i < recent.form.len() && i < recent.filingDate.len() {
                entries.push(FilingEntry {
                    accession_number: recent.accessionNumber.get(i).cloned().unwrap_or_default(),
                    filing_date: recent.filingDate.get(i).cloned().unwrap_or_default(),
                    report_date: recent.reportDate.get(i).cloned().unwrap_or_default(),
                    form: recent.form.get(i).cloned().unwrap_or_default(),
                    primary_document: recent.primaryDocument.get(i).cloned().unwrap_or_default(),
                    is_xbrl: recent.isXBRL.get(i).cloned().unwrap_or(0) == 1,
                    is_inline_xbrl: recent.isInlineXBRL.get(i).cloned().unwrap_or(0) == 1,
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
    /// # use edgar_rs::{EdgarApi, DefaultEdgarApi};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let edgar_api = DefaultEdgarApi::new("Your Company Name your.email@example.com");
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

/// A filing entry in a company's submission history.
#[derive(Debug, Clone)]
pub struct FilingEntry {
    /// The accession number of the filing.
    pub accession_number: String,

    /// The filing date of the filing.
    pub filing_date: String,

    /// The report date of the filing.
    pub report_date: String,

    /// The form type of the filing.
    pub form: String,

    /// The primary document for the filing.
    pub primary_document: String,

    /// Indicates whether the filing is in XBRL format.
    pub is_xbrl: bool,

    /// Indicates whether the filing is in inline XBRL format.
    pub is_inline_xbrl: bool,
}
