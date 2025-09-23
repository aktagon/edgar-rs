use edgar_rs::EdgarApi;
use crate::common::{test_client::create_test_client, constants::APPLE_CIK};

#[tokio::test]
async fn test_get_submissions_history() {
    let client = create_test_client().expect("Failed to create test client");

    let response = client.get_submissions_history(APPLE_CIK).await.unwrap();

    assert_eq!(response.status, 200);
    assert_eq!(response.data.cik, "0000320193");
    assert_eq!(response.data.name, "Apple Inc.");
}

#[tokio::test]
async fn test_get_company_facts() {
    let client = create_test_client().expect("Failed to create test client");

    let response = client.get_company_facts(APPLE_CIK).await.unwrap();

    assert_eq!(response.status, 200);
    assert_eq!(response.data.cik, 320193);
}