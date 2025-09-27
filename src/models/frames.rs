//! Models for XBRL frames data.
//!
//! This module contains data models for the SEC EDGAR API XBRL frames responses.

use log::{error, trace};
use serde::{Deserialize, Serialize};

/// An XBRL frames response from the SEC EDGAR API.
///
/// This struct represents the response from the XBRL frames endpoint, which
/// provides aggregated facts across reporting entities for a specific concept,
/// taxonomy, unit, and period.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XbrlFrames {
    /// The taxonomy used (e.g., "us-gaap").
    pub taxonomy: String,

    /// The tag identifier within the taxonomy.
    pub tag: String,

    /// The CIK numbers included in the frame.
    pub ciks: Option<Vec<String>>, // TODO: Changed, Review

    /// The unit of measure.
    pub unit: Option<String>, // TODO: Changed, Review

    /// Information about what the data represents.
    #[serde(default)]
    pub uom: String, // Unit of measure description

    /// The label for the tag.
    pub label: String,

    /// The description of the tag.
    pub description: String,

    /// The values for the frame.
    pub data: Vec<FrameValue>,
}

/// A single value in an XBRL frame.
/// TODO: loc missing from schema??
/// {"accn":"0001104659-24-037408","cik":1750,"entityName":"AAR CORP","loc":"US-IL","end":"2024-02-29","val":69200000}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameValue {
    /// The CIK number of the entity.
    pub cik: u64,

    /// The name of the entity.
    #[serde(rename = "entityName")]
    pub entity_name: String,

    /// The value of the concept.
    pub val: f64,

    /// The accession number of the filing.
    pub accn: String,

    /// The fiscal year.
    pub fy: Option<i32>, // TODO: Changed, Review

    /// The fiscal period.
    pub fp: Option<String>, // TODO: Changed, Review

    /// The form type.
    pub form: Option<String>, // TODO: Changed, Review

    /// The filed date of the report.
    pub filed: Option<String>, // TODO: Changed, Review

    /// The end date of the reporting period.
    pub end: String,

    /// The start date of the reporting period (optional).
    #[serde(default)]
    pub start: Option<String>,
}

impl XbrlFrames {
    /// Returns the values for a specific company.
    ///
    /// # Parameters
    ///
    /// * `cik` - The CIK number of the company.
    ///
    /// # Returns
    ///
    /// A vector of frame values for the specified company.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use edgar_rs::{EdgarApi, EdgarClient, Config, Taxonomy, Unit, Period};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Config::new("Your Company Name your.email@example.com");
    /// let edgar_api = EdgarClient::new(config)?;
    /// let frames = edgar_api.get_xbrl_frames(
    ///     Taxonomy::UsGaap,
    ///     "AccountsPayableCurrent",
    ///     Unit::Simple("USD".to_string()),
    ///     Period::Instantaneous(2019, 1)
    /// ).await?;
    /// let apple_values = frames.data.get_values_for_company("0000320193");
    /// for value in apple_values {
    ///     println!("Apple Accounts Payable: {} USD (as of {})", value.val, value.end);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_values_for_company(&self, cik: &str) -> Vec<&FrameValue> {
        // Try to parse the CIK
        let cik_number = match cik.parse::<u64>() {
            Ok(num) => num,
            Err(e) => {
                error!("Failed to parse CIK '{}': {}", cik, e);
                return Vec::new(); // Early return for invalid CIKs
            }
        };

        trace!("Successfully parsed CIK '{}' to {}", cik, cik_number);

        self.data
            .iter()
            .filter(|value| value.cik == cik_number)
            .collect()
    }

    /// Returns the top N companies by value.
    ///
    /// # Parameters
    ///
    /// * `n` - The number of companies to return.
    /// * `ascending` - Whether to sort in ascending order.
    ///
    /// # Returns
    ///
    /// A vector of frame values sorted by value.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use edgar_rs::{EdgarApi, EdgarClient, Config, Taxonomy, Unit, Period};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Config::new("Your Company Name your.email@example.com");
    /// let edgar_api = EdgarClient::new(config)?;
    /// let frames = edgar_api.get_xbrl_frames(
    ///     Taxonomy::UsGaap,
    ///     "AccountsPayableCurrent",
    ///     Unit::Simple("USD".to_string()),
    ///     Period::Instantaneous(2019, 1)
    /// ).await?;
    /// let top_10 = frames.data.get_top_companies(10, false);
    /// for value in top_10 {
    ///     println!("{}: {} USD", value.entity_name, value.val);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_top_companies(&self, n: usize, ascending: bool) -> Vec<&FrameValue> {
        let mut values: Vec<&FrameValue> = self.data.iter().collect();

        if ascending {
            values.sort_by(|a, b| {
                a.val
                    .partial_cmp(&b.val)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
        } else {
            values.sort_by(|a, b| {
                b.val
                    .partial_cmp(&a.val)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
        }

        values.into_iter().take(n).collect()
    }

    /// Returns basic statistics about the values in the frame.
    ///
    /// # Returns
    ///
    /// A struct containing statistics about the values in the frame.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use edgar_rs::{EdgarApi, EdgarClient, Config, Taxonomy, Unit, Period};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Config::new("Your Company Name your.email@example.com");
    /// let edgar_api = EdgarClient::new(config)?;
    /// let frames = edgar_api.get_xbrl_frames(
    ///     Taxonomy::UsGaap,
    ///     "AccountsPayableCurrent",
    ///     Unit::Simple("USD".to_string()),
    ///     Period::Instantaneous(2019, 1)
    /// ).await?;
    /// let stats = frames.data.get_statistics();
    /// println!("Mean: {}, Median: {}, Min: {}, Max: {}",
    ///     stats.mean, stats.median, stats.min, stats.max);
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_statistics(&self) -> FrameStatistics {
        let mut values: Vec<f64> = self.data.iter().map(|v| v.val).collect();
        values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        let count = values.len();
        let sum: f64 = values.iter().sum();
        let mean = if count > 0 { sum / count as f64 } else { 0.0 };

        let median = if count > 0 {
            if count % 2 == 0 {
                (values[count / 2 - 1] + values[count / 2]) / 2.0
            } else {
                values[count / 2]
            }
        } else {
            0.0
        };

        let min = values.first().copied().unwrap_or(0.0);
        let max = values.last().copied().unwrap_or(0.0);

        // Calculate standard deviation
        let variance = if count > 1 {
            let sum_of_squares: f64 = values.iter().map(|v| (v - mean).powi(2)).sum();
            sum_of_squares / (count - 1) as f64
        } else {
            0.0
        };
        let std_dev = variance.sqrt();

        FrameStatistics {
            count,
            mean,
            median,
            min,
            max,
            std_dev,
        }
    }
}

/// Statistics about the values in a frame.
#[derive(Debug, Clone)]
pub struct FrameStatistics {
    /// The number of values.
    pub count: usize,

    /// The mean value.
    pub mean: f64,

    /// The median value.
    pub median: f64,

    /// The minimum value.
    pub min: f64,

    /// The maximum value.
    pub max: f64,

    /// The standard deviation of the values.
    pub std_dev: f64,
}
