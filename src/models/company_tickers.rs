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