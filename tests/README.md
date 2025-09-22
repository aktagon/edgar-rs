# Integration Tests for edgar-rs

This directory contains integration tests that verify all API endpoints work correctly with real SEC data.

## Test Modes

### **Direct Mode (Default - Recommended)**

Tests run directly against the SEC API. Simple and reliable.

### **WireMock Recording Mode (Optional)**

For faster CI or offline testing - records real responses for replay.

---

## Running Tests

### Option 1: Direct SEC API (Recommended)

**Simplest approach - just run the tests:**

```bash
# Run all integration tests
cargo test --test lib

# Run specific test suite
cargo test --test lib integration::client_tests

# Run with output
cargo test --test lib -- --nocapture
```

**What happens:**

- Tests hit real SEC API directly
- Uses authentic data (Apple Inc. CIK: 0000320193)
- Validates real JSON parsing
- Tests rate limiting and error handling

### Option 2: WireMock Recording & Replay

**For CI pipelines or offline development:**

#### Quick Start (Automated Script)

```bash
# Record real responses (one-time setup)
./tests/run-wiremock.sh record

# Use recorded responses (fast!)
./tests/run-wiremock.sh replay

# Clean up
./tests/run-wiremock.sh clean
```

#### Manual Setup

#### Step 1: Record Real Responses

```bash
# 1. Start WireMock in recording mode
docker run -it --rm \
  -p 8080:8080 \
  -v $(pwd)/tests/wiremock:/home/wiremock \
  wiremock/wiremock:latest \
  --proxy-all="https://data.sec.gov" \
  --proxy-all="https://www.sec.gov" \
  --record-mappings \
  --verbose

# 2. Run tests through proxy (records responses)
HTTP_PROXY=http://localhost:8080 HTTPS_PROXY=http://localhost:8080 \
  cargo test --test lib -- --nocapture
```

#### Step 2: Replay Recorded Responses

```bash
# 1. Stop recording container (Ctrl+C)
# 2. Start WireMock in replay mode
docker run -it --rm \
  -p 8080:8080 \
  -v $(pwd)/tests/wiremock:/home/wiremock \
  wiremock/wiremock:latest

# 3. Run tests using recorded responses (fast!)
HTTP_PROXY=http://localhost:8080 HTTPS_PROXY=http://localhost:8080 \
  cargo test --test lib
```

---

## Test Suites

### Available Test Files

```bash
# Basic client functionality
cargo test --test lib integration::client_tests

# XBRL endpoints (concepts, frames)
cargo test --test lib integration::xbrl_tests

# Company/MF tickers
cargo test --test lib integration::tickers_tests

# Error handling
cargo test --test lib integration::error_tests

# Test utilities
cargo test --test lib common::test_client
```

### Individual Tests

```bash
# Test Apple submissions
cargo test --test lib test_get_submissions_history

# Test company facts
cargo test --test lib test_get_company_facts

# Test invalid CIK handling
cargo test --test lib test_invalid_cik

# Test XBRL frames
cargo test --test lib test_get_xbrl_frames
```

---

## Troubleshooting

### Test Failures

**"Rate limited" errors:**

```bash
# Wait a few seconds and retry
sleep 5 && cargo test --test lib
```

**Network timeout:**

```bash
# Increase timeout and retry
RUST_TEST_TIMEOUT=60 cargo test --test lib
```

### WireMock Issues

**Container won't start:**

```bash
# Check if port 8080 is in use
lsof -i :8080

# Use different port
docker run -p 9090:8080 ... wiremock/wiremock:latest
HTTP_PROXY=http://localhost:9090 cargo test --test lib
```

**No responses recorded:**

```bash
# Verify proxy settings
echo $HTTP_PROXY
echo $HTTPS_PROXY

# Check WireMock logs for incoming requests
```

**Recorded responses not found:**

```bash
# Verify volume mount
ls -la tests/wiremock/mappings/
ls -la tests/wiremock/__files/
```

---

## CI/CD Integration

### GitHub Actions Example

```yaml
name: Integration Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest

    services:
      wiremock:
        image: wiremock/wiremock:latest
        ports:
          - 8080:8080
        volumes:
          - ./tests/wiremock:/home/wiremock

    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      # Option 1: Direct tests (simple)
      - name: Run integration tests (direct)
        run: cargo test --test lib

      # Option 2: With WireMock (if recordings exist)
      - name: Run integration tests (recorded)
        run: |
          HTTP_PROXY=http://localhost:8080 \
          HTTPS_PROXY=http://localhost:8080 \
          cargo test --test lib
        if: hashFiles('tests/wiremock/mappings/*.json') != ''
```

---

## Quick Reference

### TL;DR - Just Run Tests

```bash
# Simple: Test against real SEC API
cargo test --test lib

# Fast: Record once, replay many times
./tests/run-wiremock.sh record   # One-time setup
./tests/run-wiremock.sh replay   # Fast repeated testing
```

### Environment Variables

```bash
HTTP_PROXY=http://localhost:8080     # Route through WireMock
HTTPS_PROXY=http://localhost:8080    # Route HTTPS through WireMock
WIREMOCK_PORT=9090                   # Use different port
RUST_TEST_TIMEOUT=60                 # Increase test timeout
```

### Docker Commands

```bash
# Recording mode
docker run --rm -p 8080:8080 -v $(pwd)/tests/wiremock:/home/wiremock \
  wiremock/wiremock:latest --proxy-all="https://data.sec.gov" --record-mappings

# Replay mode
docker run --rm -p 8080:8080 -v $(pwd)/tests/wiremock:/home/wiremock \
  wiremock/wiremock:latest

# Stop all WireMock containers
docker stop $(docker ps -q --filter ancestor=wiremock/wiremock)
```

## Test Structure

- `integration/client_tests.rs` - Basic client functionality
- `integration/xbrl_tests.rs` - XBRL-related endpoints
- `integration/tickers_tests.rs` - Ticker endpoints
- `integration/error_tests.rs` - Error handling
- `common/` - Shared test utilities

## Test Data

All tests use Apple Inc. (CIK: 0000320193) for consistency. This provides:

- Comprehensive API coverage with single company
- Predictable, stable test data
- Real SEC response structures

## Benefits

- ✅ **Simple**: No complex HTTP client modifications
- ✅ **Authentic**: Real SEC API responses recorded
- ✅ **Fast**: Replay mode has no network calls
- ✅ **Reliable**: Same responses every test run
- ✅ **KISS**: Uses standard HTTP proxy pattern

