//! Example of using edgar-rs in a Cloudflare Worker
//!
//! This example demonstrates how to use the edgar-rs library in a Cloudflare Worker
//! to access SEC EDGAR API endpoints. This example requires the `cloudflare-workers`
//! feature to be enabled.

#[cfg(feature = "cloudflare-workers")]
mod worker_example {
    use edgar_rs::{EdgarApi, EdgarClient};
    use worker::*;

    #[event(fetch)]
    pub async fn main(req: Request, _env: Env, _ctx: Context) -> Result<Response> {
        // Parse URL to get CIK from query parameters
        let url = req.url()?;
        let cik = url
            .query_pairs()
            .find(|(key, _)| key == "cik")
            .map(|(_, value)| value.to_string())
            .unwrap_or_else(|| "0000320193".to_string()); // Default to Apple Inc.

        // Create Edgar client for Cloudflare Workers
        let edgar_client = EdgarClient::new_worker("YourCompany contact@yourcompany.com");

        // Get company submissions
        match edgar_client.get_submissions_history(&cik).await {
            Ok(submissions) => {
                let response_data = serde_json::json!({
                    "company": submissions.data.name,
                    "cik": submissions.data.cik,
                    "entityType": submissions.data.entityType,
                    "recent_filings_count": submissions.data.filings.recent.accessionNumber.len(),
                    "status": submissions.status
                });

                Response::from_json(&response_data)
            }
            Err(e) => {
                let error_response = serde_json::json!({
                    "error": e.to_string(),
                    "cik": cik
                });

                Response::from_json(&error_response)?.with_status(500)
            }
        }
    }

    #[event(fetch)]
    pub async fn company_facts(req: Request, _env: Env, _ctx: Context) -> Result<Response> {
        let url = req.url()?;
        let cik = url
            .query_pairs()
            .find(|(key, _)| key == "cik")
            .map(|(_, value)| value.to_string())
            .unwrap_or_else(|| "0000320193".to_string());

        let edgar_client = EdgarClient::new_worker("YourCompany contact@yourcompany.com");

        match edgar_client.get_company_facts(&cik).await {
            Ok(facts) => {
                let response_data = serde_json::json!({
                    "company": facts.data.entityName,
                    "cik": facts.data.cik,
                    "facts_count": facts.data.facts.len(),
                    "status": facts.status
                });

                Response::from_json(&response_data)
            }
            Err(e) => {
                let error_response = serde_json::json!({
                    "error": e.to_string(),
                    "cik": cik
                });

                Response::from_json(&error_response)?.with_status(500)
            }
        }
    }
}

// For non-worker builds, provide a placeholder
#[cfg(not(feature = "cloudflare-workers"))]
fn main() {
    println!("This example requires the 'cloudflare-workers' feature to be enabled.");
    println!("Build with: cargo build --example cloudflare_worker --features cloudflare-workers");
}