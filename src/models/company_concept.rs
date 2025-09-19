//! Models for company concept data.
//!
//! This module contains data models for the SEC EDGAR API company concept responses.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A company concept response from the SEC EDGAR API.
///
/// This struct represents the response from the company concept endpoint, which
/// provides all disclosures of a specific XBRL concept for a specific company.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanyConcept {
    /// The CIK number of the company.
    //#[serde(deserialize_with = "deserialize_cik")]
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
    /// # use edgar_rs::{EdgarApi, EdgarClient, Taxonomy};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let edgar_api = EdgarClient::new("Your Company Name your.email@example.com");
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
    /// # use edgar_rs::{EdgarApi, EdgarClient, Taxonomy};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let edgar_api = EdgarClient::new("Your Company Name your.email@example.com");
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
    /// # use edgar_rs::{EdgarApi, EdgarClient, Taxonomy};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let edgar_api = EdgarClient::new("Your Company Name your.email@example.com");
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
    /// # use edgar_rs::{EdgarApi, EdgarClient, Taxonomy};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let edgar_api = EdgarClient::new("Your Company Name your.email@example.com");
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
}
