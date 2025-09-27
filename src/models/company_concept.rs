//! Models for company concept data.
//!
//! This module contains data models for the SEC EDGAR API company concept responses.

use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;

/// Custom deserializer for CIK that accepts both string and integer values.
fn deserialize_cik<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;

    #[derive(Deserialize)]
    #[serde(untagged)]
    enum CikValue {
        String(String),
        Integer(u64),
    }

    match CikValue::deserialize(deserializer)? {
        CikValue::String(s) => s.parse().map_err(D::Error::custom),
        CikValue::Integer(i) => Ok(i),
    }
}

/// A company concept response from the SEC EDGAR API.
///
/// This struct represents the response from the company concept endpoint, which
/// provides all disclosures of a specific XBRL concept for a specific company.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanyConcept {
    /// The CIK number of the company.
    #[serde(deserialize_with = "deserialize_cik")]
    pub cik: u64,
    //pub cik: String,
    /// The entity name.
    #[serde(rename = "entityName")]
    pub entity_name: String,

    /// The taxonomy used (e.g., "us-gaap").
    pub taxonomy: String,

    /// The tag identifier within the taxonomy.
    pub tag: String,

    /// The label for the tag.
    pub label: String,

    /// The description of the tag.
    pub description: String,

    /// The units of measure and their associated values.
    pub units: HashMap<String, Vec<ConceptValue>>,
}

/// A single value for a company concept.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptValue {
    /// The end date of the reporting period.
    pub end: String,

    /// The value of the concept.
    pub val: f64,

    /// The accession number of the filing.
    pub accn: String,

    /// The fiscal year.
    pub fy: i32,

    /// The fiscal period.
    pub fp: String,

    /// The form type.
    pub form: String,

    /// The filed date of the report.
    pub filed: String,

    /// The frame used for the value.
    pub frame: Option<String>,

    /// The start date of the reporting period (optional).
    #[serde(default)]
    pub start: Option<String>,
}


impl CompanyConcept {
    /// Returns the values for the specified unit of measure.
    ///
    /// # Parameters
    ///
    /// * `unit` - The unit of measure.
    ///
    /// # Returns
    ///
    /// A vector of values for the specified unit of measure, or an empty vector
    /// if no values exist for the unit.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use edgar_rs::{EdgarApi, EdgarClient, Config, Taxonomy};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Config::new("Your Company Name your.email@example.com");
    /// let edgar_api = EdgarClient::new(config)?;
    /// let concept = edgar_api.get_company_concept(
    ///     "0000320193",
    ///     Taxonomy::UsGaap,
    ///     "AccountsPayableCurrent"
    /// ).await?;
    /// let usd_values = concept.data.get_values_for_unit("USD");
    /// for value in usd_values {
    ///     println!("Period ending {}: {}", value.end, value.val);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_values_for_unit(&self, unit: &str) -> Vec<&ConceptValue> {
        match self.units.get(unit) {
            Some(values) => values.iter().collect(),
            None => Vec::new(),
        }
    }

    /// Returns the most recent value for the specified unit of measure.
    ///
    /// # Parameters
    ///
    /// * `unit` - The unit of measure.
    ///
    /// # Returns
    ///
    /// The most recent value for the specified unit of measure, or `None` if no
    /// values exist for the unit.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use edgar_rs::{EdgarApi, EdgarClient, Config, Taxonomy};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Config::new("Your Company Name your.email@example.com");
    /// let edgar_api = EdgarClient::new(config)?;
    /// let concept = edgar_api.get_company_concept(
    ///     "0000320193",
    ///     Taxonomy::UsGaap,
    ///     "AccountsPayableCurrent"
    /// ).await?;
    /// if let Some(latest) = concept.data.get_most_recent_value("USD") {
    ///     println!("Most recent accounts payable: {} USD (as of {})", latest.val, latest.end);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_most_recent_value(&self, unit: &str) -> Option<&ConceptValue> {
        self.units.get(unit)?.iter().max_by_key(|v| &v.end)
    }

    /// Returns all available units of measure.
    ///
    /// # Returns
    ///
    /// A vector of available units of measure.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use edgar_rs::{EdgarApi, EdgarClient, Config, Taxonomy};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Config::new("Your Company Name your.email@example.com");
    /// let edgar_api = EdgarClient::new(config)?;
    /// let concept = edgar_api.get_company_concept(
    ///     "0000320193",
    ///     Taxonomy::UsGaap,
    ///     "AccountsPayableCurrent"
    /// ).await?;
    /// let units = concept.data.get_available_units();
    /// println!("Available units: {:?}", units);
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_available_units(&self) -> Vec<&String> {
        self.units.keys().collect()
    }

    /// Returns the values for the specified fiscal year and period.
    ///
    /// # Parameters
    ///
    /// * `fiscal_year` - The fiscal year.
    /// * `fiscal_period` - The fiscal period (e.g., "Q1", "Q2", "Q3", "FY").
    ///
    /// # Returns
    ///
    /// A vector of values for the specified fiscal year and period.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use edgar_rs::{EdgarApi, EdgarClient, Config, Taxonomy};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Config::new("Your Company Name your.email@example.com");
    /// let edgar_api = EdgarClient::new(config)?;
    /// let concept = edgar_api.get_company_concept(
    ///     "0000320193",
    ///     Taxonomy::UsGaap,
    ///     "AccountsPayableCurrent"
    /// ).await?;
    /// let q1_2023_values = concept.data.get_values_for_fiscal_period(2023, "Q1");
    /// for (unit, value) in q1_2023_values {
    ///     println!("Q1 2023 value in {}: {}", unit, value.val);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_values_for_fiscal_period(
        &self,
        fiscal_year: i32,
        fiscal_period: &str,
    ) -> Vec<(String, &ConceptValue)> {
        let mut results = Vec::new();

        for (unit, values) in &self.units {
            for value in values {
                if value.fy == fiscal_year && value.fp == fiscal_period {
                    results.push((unit.clone(), value));
                }
            }
        }

        results
    }

    /// Get the CIK formatted as a string with 10 digits (with leading zeros)
    pub fn get_cik_as_string(&self) -> String {
        format!("{:010}", self.cik)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_deserialize_cik_as_integer() {
        let json = json!({
            "cik": 320193,
            "entityName": "APPLE INC",
            "taxonomy": "us-gaap",
            "tag": "AccountsPayableCurrent",
            "label": "Accounts Payable, Current",
            "description": "Carrying value as of the balance sheet date of liabilities incurred and payable to vendors for goods and services received that are used in an entity's business.",
            "units": {}
        });

        let concept: CompanyConcept = serde_json::from_value(json).unwrap();
        assert_eq!(concept.cik, 320193);
    }

    #[test]
    fn test_deserialize_cik_as_string() {
        let json = json!({
            "cik": "320193",
            "entityName": "APPLE INC",
            "taxonomy": "us-gaap",
            "tag": "AccountsPayableCurrent",
            "label": "Accounts Payable, Current",
            "description": "Carrying value as of the balance sheet date of liabilities incurred and payable to vendors for goods and services received that are used in an entity's business.",
            "units": {}
        });

        let concept: CompanyConcept = serde_json::from_value(json).unwrap();
        assert_eq!(concept.cik, 320193);
    }

    #[test]
    fn test_get_cik_as_string() {
        let concept = CompanyConcept {
            cik: 320193,
            entity_name: "APPLE INC".to_string(),
            taxonomy: "us-gaap".to_string(),
            tag: "AccountsPayableCurrent".to_string(),
            label: "Accounts Payable, Current".to_string(),
            description: "Description".to_string(),
            units: HashMap::new(),
        };

        assert_eq!(concept.get_cik_as_string(), "0000320193");
    }

    fn create_test_concept() -> CompanyConcept {
        let mut units = HashMap::new();

        let usd_values = vec![
            ConceptValue {
                end: "2023-12-31".to_string(),
                val: 1000000.0,
                accn: "0000320193-23-000064".to_string(),
                fy: 2023,
                fp: "FY".to_string(),
                form: "10-K".to_string(),
                filed: "2023-11-03".to_string(),
                frame: Some("CY2023Q4".to_string()),
                start: Some("2023-01-01".to_string()),
            },
            ConceptValue {
                end: "2023-09-30".to_string(),
                val: 950000.0,
                accn: "0000320193-23-000106".to_string(),
                fy: 2024,
                fp: "Q1".to_string(),
                form: "10-Q".to_string(),
                filed: "2023-11-02".to_string(),
                frame: Some("CY2023Q3".to_string()),
                start: Some("2023-07-01".to_string()),
            },
        ];

        let eur_values = vec![
            ConceptValue {
                end: "2023-12-31".to_string(),
                val: 850000.0,
                accn: "0000320193-23-000064".to_string(),
                fy: 2023,
                fp: "FY".to_string(),
                form: "10-K".to_string(),
                filed: "2023-11-03".to_string(),
                frame: Some("CY2023Q4".to_string()),
                start: Some("2023-01-01".to_string()),
            },
        ];

        units.insert("USD".to_string(), usd_values);
        units.insert("EUR".to_string(), eur_values);

        CompanyConcept {
            cik: 320193,
            entity_name: "APPLE INC".to_string(),
            taxonomy: "us-gaap".to_string(),
            tag: "AccountsPayableCurrent".to_string(),
            label: "Accounts Payable, Current".to_string(),
            description: "Test description".to_string(),
            units,
        }
    }

    #[test]
    fn test_get_values_for_unit() {
        let concept = create_test_concept();

        let usd_values = concept.get_values_for_unit("USD");
        assert_eq!(usd_values.len(), 2);
        assert_eq!(usd_values[0].val, 1000000.0);
        assert_eq!(usd_values[1].val, 950000.0);

        let eur_values = concept.get_values_for_unit("EUR");
        assert_eq!(eur_values.len(), 1);
        assert_eq!(eur_values[0].val, 850000.0);

        let nonexistent_values = concept.get_values_for_unit("GBP");
        assert_eq!(nonexistent_values.len(), 0);
    }

    #[test]
    fn test_get_most_recent_value() {
        let concept = create_test_concept();

        let latest_usd = concept.get_most_recent_value("USD");
        assert!(latest_usd.is_some());
        assert_eq!(latest_usd.unwrap().end, "2023-12-31");
        assert_eq!(latest_usd.unwrap().val, 1000000.0);

        let latest_eur = concept.get_most_recent_value("EUR");
        assert!(latest_eur.is_some());
        assert_eq!(latest_eur.unwrap().val, 850000.0);

        let latest_nonexistent = concept.get_most_recent_value("GBP");
        assert!(latest_nonexistent.is_none());
    }

    #[test]
    fn test_get_available_units() {
        let concept = create_test_concept();

        let units = concept.get_available_units();
        assert_eq!(units.len(), 2);
        assert!(units.contains(&&"USD".to_string()));
        assert!(units.contains(&&"EUR".to_string()));
    }

    #[test]
    fn test_get_values_for_fiscal_period() {
        let concept = create_test_concept();

        let fy_2023_values = concept.get_values_for_fiscal_period(2023, "FY");
        assert_eq!(fy_2023_values.len(), 2);

        let q1_2024_values = concept.get_values_for_fiscal_period(2024, "Q1");
        assert_eq!(q1_2024_values.len(), 1);
        assert_eq!(q1_2024_values[0].0, "USD");
        assert_eq!(q1_2024_values[0].1.val, 950000.0);

        let nonexistent_values = concept.get_values_for_fiscal_period(2022, "Q1");
        assert_eq!(nonexistent_values.len(), 0);
    }
}
