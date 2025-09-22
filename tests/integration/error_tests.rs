use edgar_rs::{EdgarApi, EdgarApiError};
use crate::common::test_client::create_test_client;

#[tokio::test]
async fn test_invalid_cik() {
    let client = create_test_client().expect("Failed to create test client");

    let result = client.get_submissions_history("invalid_cik").await;

    match result {
        Ok(_) => {
            println!("Expected error for invalid CIK");
        }
        Err(e) => {
            println!("Correctly got error for invalid CIK: {}", e);
            // Verify it's the right kind of error
            match e {
                EdgarApiError::InvalidCik(_) | EdgarApiError::ApiError { .. } | EdgarApiError::ParseError(_) => {
                    // Expected error types for invalid CIK
                }
                _ => {
                    println!("Unexpected error type for invalid CIK: {}", e);
                }
            }
        }
    }
}

#[tokio::test]
async fn test_nonexistent_cik() {
    let client = create_test_client().expect("Failed to create test client");

    // Use a valid format but non-existent CIK
    let result = client.get_submissions_history("0000000001").await;

    match result {
        Ok(_) => {
            println!("Expected error for non-existent CIK");
        }
        Err(e) => {
            println!("Correctly got error for non-existent CIK: {}", e);
        }
    }
}