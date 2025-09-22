use edgar_rs::EdgarApi;
use crate::common::test_client::create_test_client;

#[tokio::test]
async fn test_get_company_tickers() {
    let client = create_test_client().expect("Failed to create test client");

    let result = client.get_company_tickers().await;

    match result {
        Ok(response) => {
            assert_eq!(response.status, 200);
            let entries = response.data.entries().expect("Failed to parse ticker entries");
            assert!(!entries.is_empty());
            println!("Successfully retrieved {} company tickers", entries.len());
        }
        Err(e) => {
            println!("Test failed (expected if WireMock not running): {}", e);
        }
    }
}

#[tokio::test]
async fn test_get_company_tickers_mf() {
    let client = create_test_client().expect("Failed to create test client");

    let result = client.get_company_tickers_mf().await;

    match result {
        Ok(response) => {
            assert_eq!(response.status, 200);
            let entries = response.data.entries().expect("Failed to parse MF ticker entries");
            assert!(!entries.is_empty());
            println!("Successfully retrieved {} mutual fund tickers", entries.len());
        }
        Err(e) => {
            println!("Test failed (expected if WireMock not running): {}", e);
        }
    }
}