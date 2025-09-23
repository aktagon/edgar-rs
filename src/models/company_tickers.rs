//! Models for company tickers exchange data.
//!
//! This module contains data models for the SEC company tickers exchange API response.

use serde::{Deserialize, Serialize};

/// Company tickers exchange data from the SEC.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanyTickers {
    /// Field names in order: ["cik", "name", "ticker", "exchange"]
    pub fields: Vec<String>,

    /// Company data as array of arrays matching the fields order
    pub data: Vec<Vec<serde_json::Value>>,
}

impl CompanyTickers {
    /// Parse the raw data into structured company entries
    pub fn entries(&self) -> Result<Vec<CompanyTickerEntry>, Box<dyn std::error::Error>> {
        self.data
            .iter()
            .map(|row| {
                if row.len() != 4 {
                    return Err("Invalid row length".into());
                }

                let cik = row[0].as_u64().ok_or("Invalid CIK")?;
                let name = row[1].as_str().ok_or("Invalid name")?.to_string();
                let ticker = row[2].as_str().ok_or("Invalid ticker")?.to_string();
                let exchange = row[3].as_str().unwrap_or("").to_string();

                Ok(CompanyTickerEntry {
                    cik,
                    name,
                    ticker,
                    exchange,
                })
            })
            .collect()
    }
}

/// A single company ticker entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanyTickerEntry {
    /// Central Index Key
    pub cik: u64,

    /// Company name
    pub name: String,

    /// Stock ticker symbol
    pub ticker: String,

    /// Exchange name
    pub exchange: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_company_tickers_entries_success() {
        let tickers = CompanyTickers {
            fields: vec!["cik".to_string(), "name".to_string(), "ticker".to_string(), "exchange".to_string()],
            data: vec![
                vec![
                    json!(320193),
                    json!("Apple Inc."),
                    json!("AAPL"),
                    json!("Nasdaq"),
                ],
                vec![
                    json!(789019),
                    json!("Microsoft Corporation"),
                    json!("MSFT"),
                    json!("Nasdaq"),
                ],
            ],
        };

        let entries = tickers.entries().unwrap();
        assert_eq!(entries.len(), 2);

        assert_eq!(entries[0].cik, 320193);
        assert_eq!(entries[0].name, "Apple Inc.");
        assert_eq!(entries[0].ticker, "AAPL");
        assert_eq!(entries[0].exchange, "Nasdaq");

        assert_eq!(entries[1].cik, 789019);
        assert_eq!(entries[1].name, "Microsoft Corporation");
        assert_eq!(entries[1].ticker, "MSFT");
        assert_eq!(entries[1].exchange, "Nasdaq");
    }

    #[test]
    fn test_company_tickers_entries_invalid_row_length() {
        let tickers = CompanyTickers {
            fields: vec!["cik".to_string(), "name".to_string(), "ticker".to_string(), "exchange".to_string()],
            data: vec![
                vec![json!(320193), json!("Apple Inc."), json!("AAPL")], // Missing exchange
            ],
        };

        let result = tickers.entries();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid row length"));
    }

    #[test]
    fn test_company_tickers_entries_invalid_cik() {
        let tickers = CompanyTickers {
            fields: vec!["cik".to_string(), "name".to_string(), "ticker".to_string(), "exchange".to_string()],
            data: vec![
                vec![json!("invalid"), json!("Apple Inc."), json!("AAPL"), json!("Nasdaq")],
            ],
        };

        let result = tickers.entries();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid CIK"));
    }

    #[test]
    fn test_company_tickers_entries_missing_exchange() {
        let tickers = CompanyTickers {
            fields: vec!["cik".to_string(), "name".to_string(), "ticker".to_string(), "exchange".to_string()],
            data: vec![
                vec![json!(320193), json!("Apple Inc."), json!("AAPL"), json!(null)],
            ],
        };

        let entries = tickers.entries().unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].exchange, ""); // Should default to empty string
    }
}