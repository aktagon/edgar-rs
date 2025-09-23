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
/// ```ignore
/// use edgar_rs::utils::cik::format_cik;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_cik() {
        let test_cases = [
            ("320193", "0000320193"),
            ("0000320193", "0000320193"),
            ("320193-", "0000320193"),
            ("000320193", "0000320193"),
            ("1", "0000000001"),
            ("123456789", "0123456789"),
        ];

        for (input, expected) in test_cases {
            assert_eq!(
                format_cik(input).unwrap(),
                expected,
                "Failed for input: {}",
                input
            );
        }
    }

    #[test]
    fn test_format_cik_errors() {
        let error_cases = [
            ("", "CIK must contain at least one digit"),
            ("abcdef", "CIK must contain at least one digit"),
            ("12345678901", "CIK cannot be longer than 10 digits"),
            ("abc1234567890123def", "CIK cannot be longer than 10 digits"),
            ("   ", "CIK must contain at least one digit"),
        ];

        for (input, _expected_error) in error_cases {
            assert!(
                format_cik(input).is_err(),
                "Expected error for input: {}",
                input
            );
        }
    }
}
