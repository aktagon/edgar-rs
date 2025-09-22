//! Models for mutual fund company tickers data.
//!
//! This module contains data models for the SEC mutual fund company tickers API response.

use serde::{Deserialize, Serialize};

/// Mutual fund company tickers data from the SEC.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanyTickersMf {
    /// Field names in order: ["cik", "seriesId", "classId", "symbol"]
    pub fields: Vec<String>,

    /// Mutual fund data as array of arrays matching the fields order
    pub data: Vec<Vec<serde_json::Value>>,
}

impl CompanyTickersMf {
    /// Parse the raw data into structured mutual fund entries
    pub fn entries(&self) -> Result<Vec<MutualFundTickerEntry>, Box<dyn std::error::Error>> {
        self.data
            .iter()
            .map(|row| {
                if row.len() != 4 {
                    return Err("Invalid row length".into());
                }

                let cik = row[0].as_u64().ok_or("Invalid CIK")?;
                let series_id = row[1].as_str().ok_or("Invalid series ID")?.to_string();
                let class_id = row[2].as_str().ok_or("Invalid class ID")?.to_string();
                let symbol = row[3].as_str().ok_or("Invalid symbol")?.to_string();

                Ok(MutualFundTickerEntry {
                    cik,
                    series_id,
                    class_id,
                    symbol,
                })
            })
            .collect()
    }
}

/// A single mutual fund ticker entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MutualFundTickerEntry {
    /// Central Index Key
    pub cik: u64,

    /// Series identifier
    pub series_id: String,

    /// Class identifier
    pub class_id: String,

    /// Fund symbol/ticker
    pub symbol: String,
}