//! Common type definitions used throughout the library.
//!
//! This module contains common types such as `ApiResponse`, `Taxonomy`, `Period`, and `Unit`
//! that are used by various components of the library.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Common response type for API calls
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    /// The HTTP status code of the response
    pub status: u16,
    /// The data returned by the API
    pub data: T,
}

/// XBRL taxonomy types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Taxonomy {
    /// US GAAP taxonomy
    UsGaap,
    /// IFRS taxonomy
    IfrsFull,
    /// Document and Entity Information taxonomy
    Dei,
    /// SEC Reporting Taxonomy
    Srt,
}

impl Taxonomy {
    /// Converts the taxonomy to its string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            Taxonomy::UsGaap => "us-gaap",
            Taxonomy::IfrsFull => "ifrs-full",
            Taxonomy::Dei => "dei",
            Taxonomy::Srt => "srt",
        }
    }

    /// Attempts to parse a string into a Taxonomy
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "us-gaap" => Some(Taxonomy::UsGaap),
            "ifrs-full" => Some(Taxonomy::IfrsFull),
            "dei" => Some(Taxonomy::Dei),
            "srt" => Some(Taxonomy::Srt),
            _ => None,
        }
    }
}

/// Reporting period types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Period {
    /// Annual reporting period (CY####)
    Annual(u16),
    /// Quarterly reporting period (CY####Q#)
    Quarterly(u16, u8),
    /// Instantaneous reporting period (CY####Q#I)
    Instantaneous(u16, u8),
}

impl fmt::Display for Period {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Period::Annual(year) => write!(f, "CY{}", year),
            Period::Quarterly(year, quarter) => write!(f, "CY{}Q{}", year, quarter),
            Period::Instantaneous(year, quarter) => write!(f, "CY{}Q{}I", year, quarter),
        }
    }
}

impl Period {
    /// Converts the period to its string representation
    pub fn as_str(&self) -> String {
        self.to_string()
    }

    /// Attempts to parse a string into a Period
    pub fn from_str(s: &str) -> Option<Self> {
        if s.starts_with("CY") {
            let s = &s[2..]; // Remove "CY"

            if let Some(i) = s.find('Q') {
                let year = s[..i].parse::<u16>().ok()?;
                let remainder = &s[i + 1..];

                if remainder.ends_with('I') {
                    let quarter = remainder[..remainder.len() - 1].parse::<u8>().ok()?;
                    if quarter >= 1 && quarter <= 4 {
                        return Some(Period::Instantaneous(year, quarter));
                    }
                } else {
                    let quarter = remainder.parse::<u8>().ok()?;
                    if quarter >= 1 && quarter <= 4 {
                        return Some(Period::Quarterly(year, quarter));
                    }
                }
            } else {
                let year = s.parse::<u16>().ok()?;
                return Some(Period::Annual(year));
            }
        }

        None
    }
}

/// Unit of measure types
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Unit {
    /// Simple unit (e.g., "USD", "pure")
    Simple(String),
    /// Compound unit (e.g., "USD-per-shares")
    Compound(String, String),
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Unit::Simple(unit) => write!(f, "{}", unit),
            Unit::Compound(numerator, denominator) => write!(f, "{}-per-{}", numerator, denominator),
        }
    }
}

impl Unit {
    /// Converts the unit to its string representation
    pub fn as_str(&self) -> String {
        self.to_string()
    }

    /// Attempts to parse a string into a Unit
    pub fn from_str(s: &str) -> Self {
        if let Some(idx) = s.find("-per-") {
            let numerator = s[..idx].to_string();
            let denominator = s[idx + 5..].to_string();
            Unit::Compound(numerator, denominator)
        } else {
            Unit::Simple(s.to_string())
        }
    }
}
