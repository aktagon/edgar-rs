use edgar_rs::{EdgarApi, EdgarApiError};
use crate::common::test_client::create_test_client;

#[tokio::test]
async fn test_invalid_cik() {
    let client = create_test_client().expect("Failed to create test client");

    let result = client.get_submissions_history("invalid_cik").await;

    assert!(result.is_err(), "Expected error for invalid CIK");

    let error = result.unwrap_err();
    assert!(
        matches!(error, EdgarApiError::InvalidCik(_) | EdgarApiError::ApiError { .. } | EdgarApiError::ParseError(_)),
        "Unexpected error type: {}", error
    );
}

#[tokio::test]
async fn test_nonexistent_cik() {
    let client = create_test_client().expect("Failed to create test client");

    let result = client.get_submissions_history("0000000001").await;

    assert!(result.is_err(), "Expected error for non-existent CIK");
}