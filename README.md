# edgar-rs

A simple, async Rust client for the SEC EDGAR API. Fetch company profiles,
recent filings, XBRL data and key financial metrics.

## Installation

Add to your project’s `Cargo.toml`:

```toml
[dependencies]
edgar-rs = { git = "https://github.com/aktagon/edgar-rs.git", branch = "master" }

# Alternatively:
edgar-rs = "0.1.0"
```

## Example

To enable and run the bundled examples:

```bash
RUST_LOG=info \
cargo run --example basic_usage --features="examples" 1067983
```

```text
Fetching information for company with CIK: 1067983

--- Company Information ---
Company: BERKSHIRE HATHAWAY INC
Listed on NYSE as BRK-A
Listed on NYSE as BRK-B
SIC: 6331 – Fire, Marine & Casualty Insurance

--- Recent Filings ---
1. 8-K filed on 2025-04-17 (for period ending 2025-04-17)
…

--- Revenue Data ---
1. 10-K – Period: FY – Revenue: $249714.00 million
…

--- Industry Comparison (Cash and Cash Equivalents) ---
1. The Goldman Sachs Group, Inc. – $209.38 billion
…
```

## Usage

```rust
use edgar_rs::{EdgarClient, EdgarApi, Period, Taxonomy, Unit};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let api = EdgarClient::new("MyCompany my.email@example.com");
    let cik = "0000320193";
    // 1. Company profile & submissions history
    let subs = api.get_submissions_history(cik).await?;
    // 2. Recent filings
    let filings = subs.data.get_recent_filings();
    // 3. Revenue time series
    let rev = api
        .get_company_concept(cik, Taxonomy::UsGaap, "RevenueFromContractWithCustomerExcludingAssessedTax")
        .await?;
    // 4. XBRL frames for industry comparison
    let cf = api
        .get_xbrl_frames(Taxonomy::UsGaap, "CashAndCashEquivalentsAtCarryingValue", Unit::Simple("USD".into()), Period::Instantaneous(2024, 1))
        .await?;
    // 5. All company facts (XBRL tags)
    let facts = api.get_company_facts(cik).await?;
    Ok(())
}
```

## API Reference

- **`EdgarClient::new(user_agent: &str)`**
  Constructs a new client. Pass a descriptive user-agent (e.g. `"MyCompany name@domain.com"`).

- **`get_submissions_history(cik: &str) -> SubmissionResponse`**
  Fetches company name, CIK, ticker↔exchange map, SIC code & description and submission history.

- **`get_submissions_file(filename: &str) -> Recent`**
  Fetches additional filing history files when there are more than 1000 filings. Filenames are provided in the `files` field of the main submissions response.

- **`SubmissionData::get_ticker_map() -> HashMap<String,String>`**
  Returns a map of ticker symbols to exchange names.

- **`SubmissionData::get_recent_filings() -> Vec<Filing>`**
  Returns a chronological list of recent filings (`form`, `filing_date`, `report_date`). Limited to the most recent 1000 filings.

- **`SubmissionData::get_all_filings(api_client: &EdgarApi) -> Result<Vec<Filing>>`**
  Returns a comprehensive list of all filings including those from paginated files. Use this when you need access to more than 1000 filings.

- **`get_company_concept(cik: &str, taxonomy: Taxonomy, concept: &str) -> ConceptResponse`**
  Retrieves time-series values for a given GAAP/IFRS concept (e.g. revenue).

  - Access `.data.units["USD"]` to get `Vec<ConceptValue>` with `form`, `fp`, `val`, `end`.

- **`get_xbrl_frames(taxonomy: Taxonomy, concept: &str, unit: Unit, period: Period) -> XbrlFramesResponse`**
  Pulls a snapshot of that concept across all SEC filers for a given period.

  - Use `.data.get_top_companies(n, ascending)` and `.data.get_statistics()`.

- **`get_company_facts(cik: &str) -> CompanyFactsResponse`**
  Retrieves all XBRL tags for a company.

  - Iterate `.data.get_taxonomies()` and `.data.get_tags_for_taxonomy(taxonomy)` for counts.
  - Use `.data.get_facts_for_form("10-K")` to filter by filing type.


## Feature Flags

- **`examples`** – Enable to run the `basic_usage` example.

## Support

Commercial support is available. Contact christian@aktagon.com.

## License

MIT
