//! Utilities for working with CIK (Central Index Key) numbers.
//!
//! This module contains utility functions for formatting and validating CIK numbers.

/// Formats a CIK number to ensure it's 10 digits with leading zeros.
///
/// # Parameters
///
/// * `cik` - The CIK number to format.
///
/// # Returns
///
/// The formatted CIK number as a string with 10 digits and leading zeros.
///
/// # Errors
///
/// Returns an error if the CIK is not a valid number.
///
/// # Example
///
/// ```
/// use edgar_rs::utils::cik::format_cik;
///
/// let formatted = format_cik("320193").unwrap();
/// assert_eq!(formatted, "0000320193");
/// ```
pub fn format_cik(cik: &str) -> Result<String, &'static str> {
    // Remove any non-numeric characters
    let clean_cik = cik
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>();

    // Check if the CIK is a valid number
    if clean_cik.is_empty() {
        return Err("CIK must contain at least one digit");
    }

    if clean_cik.len() > 10 {
        return Err("CIK cannot be longer than 10 digits");
    }

    // Format the CIK with leading zeros
    Ok(format!(
        "{:010}",
        clean_cik.parse::<u64>().map_err(|_| "Invalid CIK format")?
    ))
}

/// Validates a CIK number.
///
/// # Parameters
///
/// * `cik` - The CIK number to validate.
///
/// # Returns
///
/// `true` if the CIK is valid, `false` otherwise.
///
/// # Example
///
/// ```
/// use edgar_rs::utils::cik::is_valid_cik;
///
/// assert_eq!(is_valid_cik("0000320193"), true);
/// assert_eq!(is_valid_cik("not a cik"), false);
/// ```
pub fn is_valid_cik(cik: &str) -> bool {
    format_cik(cik).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_cik() {
        assert_eq!(format_cik("320193").unwrap(), "0000320193");
        assert_eq!(format_cik("0000320193").unwrap(), "0000320193");
        assert_eq!(format_cik("320193-").unwrap(), "0000320193");
        assert_eq!(format_cik("000320193").unwrap(), "0000320193");
    }

    #[test]
    fn test_format_cik_errors() {
        assert!(format_cik("").is_err());
        assert!(format_cik("abcdef").is_err());
        assert!(format_cik("12345678901").is_err());
    }

    #[test]
    fn test_is_valid_cik() {
        assert!(is_valid_cik("320193"));
        assert!(is_valid_cik("0000320193"));
        assert!(is_valid_cik("320193-"));
        assert!(!is_valid_cik(""));
        assert!(!is_valid_cik("abcdef"));
        assert!(!is_valid_cik("12345678901"));
    }
}
