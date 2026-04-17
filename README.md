# edgar-rs

Async Rust client for the SEC EDGAR API. Company profiles, filings, XBRL facts, and cross-company frames. Runs on native tokio or Cloudflare Workers.

EDGAR requires every request to identify itself with a [descriptive User-Agent](https://www.sec.gov/os/accessing-edgar-data). The client takes one as a constructor argument and rejects empty values.

## Install

```toml
[dependencies]
edgar-rs = { git = "https://github.com/aktagon/edgar-rs.git", branch = "master" }
tokio = { version = "1", features = ["full"] }
```

## Minimal

```rust
use edgar_rs::{EdgarClient, EdgarApi};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = EdgarClient::new("Aktagon christian@aktagon.com");
    let subs = api.get_submissions_history("0000320193").await?; // Apple
    println!("{} — {} recent filings", subs.data.name, subs.data.filings.recent.accession_number.len());
    Ok(())
}
```

## Full walk-through

```rust
use edgar_rs::{EdgarClient, EdgarApi, Period, Taxonomy, Unit};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let api = EdgarClient::new("MyCompany my.email@example.com");
    let cik = "0000320193";

    // Company profile and recent filings
    let subs = api.get_submissions_history(cik).await?;
    let filings = subs.data.get_recent_filings();

    // Revenue time series from XBRL
    let rev = api.get_company_concept(
        cik, Taxonomy::UsGaap, "RevenueFromContractWithCustomerExcludingAssessedTax",
    ).await?;

    // Cross-company snapshot for a single concept
    let cash = api.get_xbrl_frames(
        Taxonomy::UsGaap,
        "CashAndCashEquivalentsAtCarryingValue",
        Unit::Simple("USD".into()),
        Period::Instantaneous(2024, 1),
    ).await?;

    // All XBRL facts for a company
    let facts = api.get_company_facts(cik).await?;

    Ok(())
}
```

## Run the bundled example

```bash
RUST_LOG=info cargo run --example basic_usage --features="examples" 1067983
```

CIK 1067983 is Berkshire Hathaway. Expected output includes company metadata, recent 8-K filings, multi-year revenue, and a cross-industry cash snapshot.

## API reference

- **`EdgarClient::new(user_agent: &str)`** — construct a native client. Pass `"Company name@domain.com"` (EDGAR-required format).
- **`get_submissions_history(cik) -> SubmissionResponse`** — company name, CIK, ticker-to-exchange map, SIC code and description, submission history.
- **`get_submissions_file(filename) -> Recent`** — pull paginated filing files when a company has more than 1000 filings. Filenames come from the `files` field of the main submissions response.
- **`SubmissionData::get_ticker_map() -> HashMap<String,String>`** — ticker symbols to exchange names.
- **`SubmissionData::get_recent_filings() -> Vec<Filing>`** — most recent 1000 filings, with `form`, `filing_date`, `report_date`.
- **`SubmissionData::get_all_filings(api) -> Result<Vec<Filing>>`** — complete history including paginated files.
- **`get_company_concept(cik, taxonomy, concept) -> ConceptResponse`** — time-series values for a single GAAP or IFRS concept. Access `.data.units["USD"]` for `Vec<ConceptValue>` with `form`, `fp`, `val`, `end`.
- **`get_xbrl_frames(taxonomy, concept, unit, period) -> XbrlFramesResponse`** — snapshot of a concept across all filers. Use `.data.get_top_companies(n, ascending)` and `.data.get_statistics()`.
- **`get_company_facts(cik) -> CompanyFactsResponse`** — every XBRL tag for a company. Iterate `.data.get_taxonomies()` and `.data.get_tags_for_taxonomy(t)`, or filter with `.data.get_facts_for_form("10-K")`.

## Runtimes

### Native (default)

```toml
edgar-rs = { git = "https://github.com/aktagon/edgar-rs.git", features = ["native"] }
```

Uses `reqwest` and `tokio`. Full functionality, including bulk downloads.

### Cloudflare Workers

```toml
edgar-rs = { git = "https://github.com/aktagon/edgar-rs.git", features = ["cloudflare-workers"] }
```

```rust
use edgar_rs::{EdgarApi, EdgarClient};
use worker::*;

#[event(fetch)]
pub async fn main(req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    let client = EdgarClient::new_worker("YourCompany contact@yourcompany.com");
    let submissions = client.get_submissions_history("0000320193").await?;

    Response::from_json(&serde_json::json!({
        "company": submissions.data.name,
        "cik": submissions.data.cik,
        "recent_filings_count": submissions.data.filings.recent.accession_number.len(),
    }))
}
```

Bulk download functions (`download_bulk_submissions`, `download_bulk_company_facts`) are unavailable in Workers — no filesystem.

## Feature flags

- `native` — default. reqwest + tokio.
- `cloudflare-workers` — Workers runtime.
- `examples` — build the `basic_usage` example.

## Rate limits

SEC EDGAR enforces 10 requests per second per IP. The client does not throttle automatically. Build a limiter around it if you query in bulk.

## License

MIT.

---

Built by [Aktagon](https://aktagon.com). Applied AI for regulated markets. Commercial support: [christian@aktagon.com](mailto:christian@aktagon.com).
