use edgar_rs::EdgarClient;
use super::constants::TEST_USER_AGENT;

/// Creates an EdgarClient for testing
pub fn create_test_client() -> Result<EdgarClient<edgar_rs::ReqwestClient>, Box<dyn std::error::Error>> {
    // Simple: just create a normal client
    // WireMock will run as a proxy, so the client doesn't need to know about it
    EdgarClient::new(TEST_USER_AGENT).map_err(Into::into)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_client() {
        let _client = create_test_client().unwrap();
        // Basic test that client can be created
    }
}