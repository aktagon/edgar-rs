use edgar_rs::EdgarApi;
use crate::common::test_client::create_test_client;

#[tokio::test]
async fn test_get_company_tickers() {
    let client = create_test_client().expect("Failed to create test client");

    let response = client.get_company_tickers().await.unwrap();

    assert_eq!(response.status, 200);
    let entries = response.data.entries().unwrap();
    assert!(!entries.is_empty());
}

#[tokio::test]
async fn test_get_company_tickers_mf() {
    let client = create_test_client().expect("Failed to create test client");

    let response = client.get_company_tickers_mf().await.unwrap();

    assert_eq!(response.status, 200);
    let entries = response.data.entries().unwrap();
    assert!(!entries.is_empty());
}