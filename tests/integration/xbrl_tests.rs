use edgar_rs::{EdgarApi, Taxonomy, Unit, Period};
use crate::common::{test_client::create_test_client, constants::{APPLE_CIK, REVENUE_CONCEPT, CASH_CONCEPT}};

#[tokio::test]
async fn test_get_company_concept() {
    let client = create_test_client().expect("Failed to create test client");

    let response = client
        .get_company_concept(APPLE_CIK, Taxonomy::UsGaap, REVENUE_CONCEPT)
        .await
        .unwrap();

    assert_eq!(response.status, 200);
    assert_eq!(response.data.cik, 320193);
    assert_eq!(response.data.entity_name, "Apple Inc.");
    assert_eq!(response.data.taxonomy, "us-gaap");
    assert_eq!(response.data.tag, REVENUE_CONCEPT);
}

#[tokio::test]
async fn test_get_xbrl_frames() {
    let client = create_test_client().expect("Failed to create test client");

    let response = client
        .get_xbrl_frames(
            Taxonomy::UsGaap,
            CASH_CONCEPT,
            Unit::Simple("USD".to_string()),
            Period::Instantaneous(2024, 1),
        )
        .await
        .unwrap();

    assert_eq!(response.status, 200);
    assert_eq!(response.data.taxonomy, "us-gaap");
    assert_eq!(response.data.tag, CASH_CONCEPT);
    assert_eq!(response.data.uom, "USD");
    assert!(!response.data.data.is_empty());
}