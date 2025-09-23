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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_company_tickers_mf_entries_success() {
        let mf_tickers = CompanyTickersMf {
            fields: vec!["cik".to_string(), "seriesId".to_string(), "classId".to_string(), "symbol".to_string()],
            data: vec![
                vec![
                    json!(1234567),
                    json!("S000012345"),
                    json!("C000012345"),
                    json!("FUNDX"),
                ],
                vec![
                    json!(7654321),
                    json!("S000067890"),
                    json!("C000067890"),
                    json!("FUNDY"),
                ],
            ],
        };

        let entries = mf_tickers.entries().unwrap();
        assert_eq!(entries.len(), 2);

        assert_eq!(entries[0].cik, 1234567);
        assert_eq!(entries[0].series_id, "S000012345");
        assert_eq!(entries[0].class_id, "C000012345");
        assert_eq!(entries[0].symbol, "FUNDX");

        assert_eq!(entries[1].cik, 7654321);
        assert_eq!(entries[1].series_id, "S000067890");
        assert_eq!(entries[1].class_id, "C000067890");
        assert_eq!(entries[1].symbol, "FUNDY");
    }

    #[test]
    fn test_company_tickers_mf_entries_invalid_row_length() {
        let mf_tickers = CompanyTickersMf {
            fields: vec!["cik".to_string(), "seriesId".to_string(), "classId".to_string(), "symbol".to_string()],
            data: vec![
                vec![json!(1234567), json!("S000012345"), json!("C000012345")], // Missing symbol
            ],
        };

        let result = mf_tickers.entries();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid row length"));
    }

    #[test]
    fn test_company_tickers_mf_entries_invalid_cik() {
        let mf_tickers = CompanyTickersMf {
            fields: vec!["cik".to_string(), "seriesId".to_string(), "classId".to_string(), "symbol".to_string()],
            data: vec![
                vec![json!("invalid"), json!("S000012345"), json!("C000012345"), json!("FUNDX")],
            ],
        };

        let result = mf_tickers.entries();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid CIK"));
    }
}