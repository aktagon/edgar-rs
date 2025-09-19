//! Rate limiting functionality for API requests.
//!
//! This module provides rate limiting functionality to ensure that requests to the
//! SEC EDGAR API don't exceed the allowed rate limits.

use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::time::{sleep, Duration};

/// A rate limiter for API requests.
///
/// This struct provides rate limiting functionality to ensure that requests to the
/// SEC EDGAR API don't exceed the allowed rate limits. It uses a token bucket
/// algorithm to limit the rate of requests.
#[derive(Debug, Clone)]
pub struct RateLimiter {
    semaphore: Arc<Semaphore>,
    rate: u32,
    per_seconds: u32,
}

impl RateLimiter {
    /// Creates a new `RateLimiter` with the specified rate limit.
    ///
    /// # Parameters
    ///
    /// * `rate` - The maximum number of requests allowed.
    /// * `per_seconds` - The time period in seconds for the rate limit.
    pub fn new(rate: u32, per_seconds: u32) -> Self {
        let semaphore = Arc::new(Semaphore::new(rate as usize));
        let limiter = Self {
            semaphore,
            rate,
            per_seconds,
        };

        // Start a background task to replenish the tokens
        limiter.start_replenisher();

        limiter
    }

    /// Acquires a token from the rate limiter, waiting if necessary.
    ///
    /// This method waits until a token is available and then acquires it.
    pub async fn acquire(&self) {
        let _permit = self.semaphore.acquire().await.unwrap();
        // Permit is dropped at the end of the scope, automatically releasing the token
    }

    /// Starts a background task to replenish tokens at the specified rate.
    fn start_replenisher(&self) {
        let semaphore = self.semaphore.clone();
        let rate = self.rate;
        let per_seconds = self.per_seconds;

        tokio::spawn(async move {
            let sleep_duration = Duration::from_millis((per_seconds as u64 * 1000) / rate as u64);

            loop {
                sleep(sleep_duration).await;
                semaphore.add_permits(1);
            }
        });
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[tokio::test]
    async fn test_rate_limiter() {
        let rate_limiter = RateLimiter::new(5, 1); // 5 requests per second
        let start = Instant::now();

        // First 5 requests should proceed immediately
        for _ in 0..5 {
            rate_limiter.acquire().await;
        }

        let elapsed = start.elapsed();
        assert!(
            elapsed.as_millis() < 100,
            "First 5 requests should be immediate"
        );

        // 6th request should be delayed
        let start = Instant::now();
        rate_limiter.acquire().await;
        let elapsed = start.elapsed();
        assert!(
            elapsed.as_millis() >= 200,
            "6th request should be delayed by at least 200ms"
        );
    }
}
