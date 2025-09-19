//! Revenue taxonomies example for the edgar-rs library.
//!
//! This example demonstrates how to retrieve revenue-related taxonomy tags
//! for a company using the SEC EDGAR API.

use edgar_rs::{EdgarClient, EdgarApi};
use env_logger;
use log::info;
use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the logger
    env_logger::init();

    info!("Revenue taxonomies example started");

    // Initialize the API client with a user agent
    let edgar_api = EdgarClient::new("Your Company Name your.email@example.com")?;

    // Get CIK from command line arguments
    let args: Vec<String> = env::args().collect();
    let cik: &str = if args.len() > 1 {
        &args[1]
    } else {
        // Fallback to Apple Inc. if no argument provided
        println!("No CIK provided. Using Apple Inc. (0000320193) as default.");
        "0000320193"
    };

    println!("Fetching revenue taxonomies for company with CIK: {}", cik);

    // Get all company facts
    let facts = edgar_api.get_company_facts(cik).await?;

    println!("\n--- Available Taxonomies ---");
    let taxonomies = facts.data.get_taxonomies();
    for taxonomy in &taxonomies {
        let tags = facts.data.get_tags_for_taxonomy(taxonomy);
        println!("{}: {} concepts available", taxonomy, tags.len());
    }

    // Look for revenue-related tags in us-gaap taxonomy
    println!("\n--- Revenue-Related Tags in US-GAAP ---");
    let us_gaap_tags = facts.data.get_tags_for_taxonomy("us-gaap");
    let revenue_tags: Vec<&String> = us_gaap_tags
        .into_iter()
        .filter(|tag| {
            let tag_lower = tag.to_lowercase();
            tag_lower.contains("revenue") ||
            tag_lower.contains("sales") ||
            tag_lower.contains("income") && !tag_lower.contains("expense")
        })
        .collect();

    println!("Found {} revenue-related tags:", revenue_tags.len());
    for (i, tag) in revenue_tags.iter().enumerate() {
        if i < 20 { // Show first 20 tags
            // Get the fact to show label if available
            if let Some(fact) = facts.data.get_fact("us-gaap", tag) {
                let label = fact.label.as_deref().unwrap_or("No label");
                println!("  {}: {}", tag, label);
            } else {
                println!("  {}", tag);
            }
        }
    }

    if revenue_tags.len() > 20 {
        println!("  ... and {} more", revenue_tags.len() - 20);
    }

    // Show specific revenue tags and their recent values
    println!("\n--- Recent Revenue Values ---");
    let specific_revenue_tags = [
        "RevenueFromContractWithCustomerExcludingAssessedTax",
        "Revenues",
        "Revenue",
        "SalesRevenueNet",
        "RevenueFromContractWithCustomerIncludingAssessedTax",
    ];

    for tag in specific_revenue_tags {
        if let Some(fact) = facts.data.get_fact("us-gaap", tag) {
            println!("\n{}:", tag);
            if let Some(label) = &fact.label {
                println!("  Label: {}", label);
            }

            // Show recent USD values
            if let Some(usd_values) = fact.units.get("USD") {
                let mut recent_values: Vec<_> = usd_values.iter().collect();
                recent_values.sort_by(|a, b| b.end.cmp(&a.end));

                println!("  Recent USD values:");
                for (i, value) in recent_values.iter().take(3).enumerate() {
                    if let Some(val) = value.as_f64() {
                        println!("    {}: ${:.2}M (period ending {})",
                               i + 1, val / 1_000_000.0, value.end);
                    }
                }
            }
        }
    }

    Ok(())
}