//! Basic usage example for the edgar-rs library.
//!
//! This example demonstrates basic usage of the edgar-rs library to retrieve
//! company information and financial data from the SEC EDGAR API.

use edgar_rs::{EdgarClient, EdgarApi, Period, Taxonomy, Unit};
use env_logger;
use log::info;
use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the logger
    env_logger::init();

    info!("Basic usage example started");
    // Initialize the API client with a user agent
    // Replace with your company name and contact email!
    let edgar_api = EdgarClient::new("Your Company Name your.email@example.com");

    // Company CIK to retrieve data for (Apple Inc.)

    // Get CIK from command line arguments
    let args: Vec<String> = env::args().collect();
    let cik: &str = if args.len() > 1 {
        &args[1]
    } else {
        // Fallback to Apple Inc. if no argument provided
        println!("No CIK provided. Using Apple Inc. (0000320193) as default.");
        "0000320193"
    };

    println!("Fetching information for company with CIK: {}", cik);

    // Step 1: Get company submissions history (basic company info)
    println!("\n--- Company Information ---");
    let submissions = edgar_api.get_submissions_history(cik).await?;
    println!("Company: {}", submissions.data.name);

    // Print ticker symbols and exchanges
    let ticker_map = submissions.data.get_ticker_map();
    for (ticker, exchange) in ticker_map {
        println!("Listed on {} as {}", exchange, ticker);
    }

    // Print SIC code and description
    if !submissions.data.sic.is_empty() {
        println!(
            "SIC: {} - {}",
            submissions.data.sic, submissions.data.sicDescription
        );
    }

    // Step 2: Get recent filings
    println!("\n--- Recent Filings ---");
    let filings = submissions.data.get_all_filings();
    for (i, filing) in filings.iter().take(5).enumerate() {
        println!(
            "{}. {} filed on {} (for period ending {})",
            i + 1,
            filing.form,
            filing.filing_date,
            filing.report_date
        );
    }

    // Step 3: Get company concept data for Revenue
    println!("\n--- Revenue Data ---");
    let revenues = edgar_api
        .get_company_concept(
            cik,
            Taxonomy::UsGaap,
            "RevenueFromContractWithCustomerExcludingAssessedTax",
        )
        .await?;

    // Print the most recent revenue figures
    if let Some(usd_values) = revenues.data.units.get("USD") {
        // Sort by end date, most recent first
        let mut values = usd_values.clone();
        values.sort_by(|a, b| b.end.cmp(&a.end));

        for (i, value) in values.iter().take(5).enumerate() {
            println!(
                "{}. {} - Period: {} - Revenue: ${:.2} million",
                i + 1,
                value.form,
                value.fp,
                value.val / 1_000_000.0
            );
        }
    }

    // Step 4: Get XBRL frames data to compare with other companies
    println!("\n--- Industry Comparison (Cash and Cash Equivalents) ---");
    let cash_frames = edgar_api
        .get_xbrl_frames(
            Taxonomy::UsGaap,
            "CashAndCashEquivalentsAtCarryingValue",
            Unit::Simple("USD".to_string()),
            Period::Instantaneous(2024, 1), // Q1 2024
        )
        .await?;

    // Print top 5 companies by cash holdings
    let top_cash = cash_frames.data.get_top_companies(5, false);
    for (i, value) in top_cash.iter().enumerate() {
        println!(
            "{}. {} - ${:.2} billion",
            i + 1,
            value.entityName,
            value.val / 1_000_000_000.0
        );
    }

    // Calculate statistics
    let stats = cash_frames.data.get_statistics();
    println!(
        "\nStatistics: Mean: ${:.2}B, Median: ${:.2}B, Max: ${:.2}B",
        stats.mean / 1_000_000_000.0,
        stats.median / 1_000_000_000.0,
        stats.max / 1_000_000_000.0
    );

    // Step 5: Get all company facts
    println!("\n--- Getting all company facts ---");
    let facts = edgar_api.get_company_facts(cik).await?;

    // Count available concepts by taxonomy
    for taxonomy in facts.data.get_taxonomies() {
        let tags = facts.data.get_tags_for_taxonomy(taxonomy);
        println!("{}: {} concepts available", taxonomy, tags.len());
    }

    // Print recent income statement metrics from the most recent 10-K
    println!("\n--- Recent 10-K Key Metrics (USD millions) ---");
    let form_10k_facts = facts.data.get_facts_for_form("10-K");

    // Define metrics to look for
    let key_metrics = [
        "RevenueFromContractWithCustomerExcludingAssessedTax",
        "CostOfGoodsAndServicesSold",
        "GrossProfit",
        "ResearchAndDevelopmentExpense",
        "SellingGeneralAndAdministrativeExpense",
        "OperatingIncomeLoss",
        "NetIncomeLoss",
    ];

    // Find the most recent 10-K filing date
    let mut latest_10k_date = String::new();
    for (_, _, _, value) in &form_10k_facts {
        if latest_10k_date.is_empty() || value.filed > latest_10k_date {
            latest_10k_date = value.filed.clone();
        }
    }

    println!("Most recent 10-K filed on: {}", latest_10k_date);

    // Find and print key metrics from this 10-K
    for metric in key_metrics {
        for (taxonomy, tag, unit, value) in &form_10k_facts {
            if *taxonomy == "us-gaap"
                && *tag == metric
                && *unit == "USD"
                && value.filed == latest_10k_date
            {
                if let Some(val) = value.as_f64() {
                    println!("{}: ${:.2}M", tag, val / 1_000_000.0);
                }
            }
        }
    }

    Ok(())
}
