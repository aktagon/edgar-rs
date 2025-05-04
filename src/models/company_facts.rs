//! Models for company facts data.
//!
//! This module contains data models for the SEC EDGAR API company facts responses.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A company facts response from the SEC EDGAR API.
///
/// This struct represents the response from the company facts endpoint, which
/// provides all XBRL facts for a specific company.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanyFacts {
    /// The CIK number of the company.
    pub cik: u64, // TODO: Changed, Review

    /// The entity name.
    pub entityName: String,

    /// The facts by taxonomy and tag.
    pub facts: HashMap<String, HashMap<String, Fact>>,
}

/// An XBRL fact for a company.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fact {
    /// The label for the fact.
    pub label: Option<String>, // TODO: Review, Changed

    /// The description of the fact.
    pub description: Option<String>, // TODO: Review, Changed

    /// The units of measure and their associated values.
    pub units: HashMap<String, Vec<FactValue>>,
}

/// A single value for a fact.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactValue {
    /// The end date of the reporting period.
    pub end: String,

    /// The value of the fact.
    #[serde(default)]
    pub val: Option<serde_json::Value>,

    /// The accession number of the filing.
    pub accn: String,

    /// The fiscal year.
    pub fy: Option<i32>, // TODO: Changed, Review

    /// The fiscal period.
    pub fp: Option<String>, // TODO: Changed, Review

    /// The form type.
    pub form: String,

    /// The filed date of the report.
    pub filed: String,

    /// The start date of the reporting period (optional).
    #[serde(default)]
    pub start: Option<String>,

    /// The frame used for the value (optional).
    #[serde(default)]
    pub frame: Option<String>,
}

impl CompanyFacts {
    /// Returns all available taxonomies.
    ///
    /// # Returns
    ///
    /// A vector of available taxonomies.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use edgar_rs::{EdgarApi, EdgarClient};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let edgar_api = EdgarClient::new("Your Company Name your.email@example.com");
    /// let facts = edgar_api.get_company_facts("0000320193").await?;
    /// let taxonomies = facts.data.get_taxonomies();
    /// println!("Available taxonomies: {:?}", taxonomies);
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_taxonomies(&self) -> Vec<&String> {
        self.facts.keys().collect()
    }

    /// Returns all available tags for a specific taxonomy.
    ///
    /// # Parameters
    ///
    /// * `taxonomy` - The taxonomy to get tags for.
    ///
    /// # Returns
    ///
    /// A vector of available tags, or an empty vector if the taxonomy doesn't exist.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use edgar_rs::{EdgarApi, EdgarClient};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let edgar_api = EdgarClient::new("Your Company Name your.email@example.com");
    /// let facts = edgar_api.get_company_facts("0000320193").await?;
    /// let tags = facts.data.get_tags_for_taxonomy("us-gaap");
    /// println!("Available us-gaap tags: {:?}", tags);
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_tags_for_taxonomy(&self, taxonomy: &str) -> Vec<&String> {
        match self.facts.get(taxonomy) {
            Some(tags) => tags.keys().collect(),
            None => Vec::new(),
        }
    }

    /// Returns a specific fact by taxonomy and tag.
    ///
    /// # Parameters
    ///
    /// * `taxonomy` - The taxonomy of the fact.
    /// * `tag` - The tag of the fact.
    ///
    /// # Returns
    ///
    /// The fact, or `None` if it doesn't exist.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use edgar_rs::{EdgarApi, EdgarClient};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let edgar_api = EdgarClient::new("Your Company Name your.email@example.com");
    /// let facts = edgar_api.get_company_facts("0000320193").await?;
    /// if let Some(fact) = facts.data.get_fact("us-gaap", "AccountsPayableCurrent") {
    ///     println!("Accounts Payable Current: {:?}", fact);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_fact(&self, taxonomy: &str, tag: &str) -> Option<&Fact> {
        self.facts.get(taxonomy)?.get(tag)
    }

    /// Returns all facts for a specific fiscal year and period.
    ///
    /// # Parameters
    ///
    /// * `fiscal_year` - The fiscal year.
    /// * `fiscal_period` - The fiscal period (e.g., "Q1", "Q2", "Q3", "FY").
    ///
    /// # Returns
    ///
    /// A vector of (taxonomy, tag, fact value) tuples.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use edgar_rs::{EdgarApi, EdgarClient};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let edgar_api = EdgarClient::new("Your Company Name your.email@example.com");
    /// let facts = edgar_api.get_company_facts("0000320193").await?;
    /// let q1_2023_facts = facts.data.get_facts_for_fiscal_period(2023, "Q1");
    /// for (taxonomy, tag, unit, value) in q1_2023_facts {
    ///     println!("{}.{} ({}) = {:?}", taxonomy, tag, unit, value.val);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_facts_for_fiscal_period(
        &self,
        fiscal_year: i32,
        fiscal_period: &str,
    ) -> Vec<(&str, &str, &str, &FactValue)> {
        let mut results = Vec::new();

        for (taxonomy, tags) in &self.facts {
            for (tag, fact) in tags {
                for (unit, values) in &fact.units {
                    for value in values {
                        if value.fy == Some(fiscal_year)
                            && value.fp.as_deref() == Some(fiscal_period)
                        {
                            results.push((taxonomy.as_str(), tag.as_str(), unit.as_str(), value));
                        }
                    }
                }
            }
        }

        results
    }

    /// Returns all facts from a specific form.
    ///
    /// # Parameters
    ///
    /// * `form` - The form type (e.g., "10-K", "10-Q").
    ///
    /// # Returns
    ///
    /// A vector of (taxonomy, tag, unit, fact value) tuples.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use edgar_rs::{EdgarApi, EdgarClient};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let edgar_api = EdgarClient::new("Your Company Name your.email@example.com");
    /// let facts = edgar_api.get_company_facts("0000320193").await?;
    /// let form_10k_facts = facts.data.get_facts_for_form("10-K");
    /// for (taxonomy, tag, unit, value) in form_10k_facts {
    ///     println!("{}.{} ({}) = {:?}", taxonomy, tag, unit, value.val);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_facts_for_form(&self, form: &str) -> Vec<(&str, &str, &str, &FactValue)> {
        let mut results = Vec::new();

        for (taxonomy, tags) in &self.facts {
            for (tag, fact) in tags {
                for (unit, values) in &fact.units {
                    for value in values {
                        if value.form == form {
                            results.push((taxonomy.as_str(), tag.as_str(), unit.as_str(), value));
                        }
                    }
                }
            }
        }

        results
    }

    /// Returns the most recent value for a specific fact.
    ///
    /// # Parameters
    ///
    /// * `taxonomy` - The taxonomy of the fact.
    /// * `tag` - The tag of the fact.
    /// * `unit` - The unit of measure.
    ///
    /// # Returns
    ///
    /// The most recent value, or `None` if it doesn't exist.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use edgar_rs::{EdgarApi, EdgarClient};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let edgar_api = EdgarClient::new("Your Company Name your.email@example.com");
    /// let facts = edgar_api.get_company_facts("0000320193").await?;
    /// if let Some(value) = facts.data.get_most_recent_value("us-gaap", "AccountsPayableCurrent", "USD") {
    ///     println!("Most recent accounts payable: {:?} USD (as of {})", value.val, value.end);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_most_recent_value(
        &self,
        taxonomy: &str,
        tag: &str,
        unit: &str,
    ) -> Option<&FactValue> {
        let fact = self.get_fact(taxonomy, tag)?;
        let values = fact.units.get(unit)?;
        values.iter().max_by_key(|v| &v.end)
    }
}

/// Helper methods for extracting typed values from FactValue
impl FactValue {
    /// Returns the value as a float, if possible.
    ///
    /// # Returns
    ///
    /// The value as a float, or `None` if it's not a number.
    pub fn as_f64(&self) -> Option<f64> {
        match &self.val {
            Some(serde_json::Value::Number(n)) => n.as_f64(),
            _ => None,
        }
    }

    /// Returns the value as an integer, if possible.
    ///
    /// # Returns
    ///
    /// The value as an integer, or `None` if it's not an integer.
    pub fn as_i64(&self) -> Option<i64> {
        match &self.val {
            Some(serde_json::Value::Number(n)) => n.as_i64(),
            _ => None,
        }
    }

    /// Returns the value as a string, if possible.
    ///
    /// # Returns
    ///
    /// The value as a string, or `None` if it's not a string.
    pub fn as_str(&self) -> Option<&str> {
        match &self.val {
            Some(serde_json::Value::String(s)) => Some(s),
            _ => None,
        }
    }

    /// Returns the value as a boolean, if possible.
    ///
    /// # Returns
    ///
    /// The value as a boolean, or `None` if it's not a boolean.
    pub fn as_bool(&self) -> Option<bool> {
        match &self.val {
            Some(serde_json::Value::Bool(b)) => Some(*b),
            _ => None,
        }
    }
}
