//! Configuration module for EDGAR API client

/// Default base URL for EDGAR API endpoints
pub const DEFAULT_BASE_URL: &str = "https://";

/// Configuration for EDGAR API client
#[derive(Debug, Clone)]
pub struct Config {
    /// Base URL for API requests (default: "https://")
    /// Can be overridden to use proxy services like "https://proxy.example.com/"
    pub base_url: String,
    /// User agent string for requests (required by SEC)
    pub user_agent: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            base_url: DEFAULT_BASE_URL.to_string(),
            user_agent: "edgar-rs/0.1.0".to_string(),
        }
    }
}

impl Config {
    /// Create a new configuration with just a user agent, using the default base URL
    ///
    /// # Parameters
    /// * `user_agent` - User agent string (should include company name and contact)
    ///
    /// # Example
    /// ```
    /// use edgar_rs::Config;
    ///
    /// let config = Config::new("Your Company contact@example.com");
    /// ```
    pub fn new(user_agent: &str) -> Self {
        Self {
            base_url: DEFAULT_BASE_URL.to_string(),
            user_agent: user_agent.to_string(),
        }
    }

    /// Build a complete URL by replacing "https://" with the configured base URL
    ///
    /// # Parameters
    /// * `url` - Original URL starting with "https://"
    ///
    /// # Returns
    /// Complete URL with base URL applied
    ///
    /// # Example
    /// ```
    /// use edgar_rs::Config;
    ///
    /// let mut config = Config::new("Company contact@example.com");
    /// config.base_url = "https://proxy.example.com/".to_string();
    /// let url = config.build_url("https://www.sec.gov/files/data.json");
    /// assert_eq!(url, "https://proxy.example.com/www.sec.gov/files/data.json");
    /// ```
    pub fn build_url(&self, url: &str) -> String {
        if url.starts_with("https://") {
            format!("{}{}", self.base_url, &url[8..])
        } else {
            url.to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.base_url, "https://");
        assert_eq!(config.user_agent, "edgar-rs/0.1.0");
    }

    #[test]
    fn test_new_config() {
        let config = Config::new("Company contact@example.com");
        assert_eq!(config.base_url, "https://");
        assert_eq!(config.user_agent, "Company contact@example.com");
    }

    #[test]
    fn test_build_url_default() {
        let config = Config::new("Company contact@example.com");
        let url = config.build_url("https://www.sec.gov/files/data.json");
        assert_eq!(url, "https://www.sec.gov/files/data.json");
    }

    #[test]
    fn test_build_url_proxy() {
        let mut config = Config::new("Company contact@example.com");
        config.base_url = "https://proxy.example.com/".to_string();
        let url = config.build_url("https://www.sec.gov/files/data.json");
        assert_eq!(url, "https://proxy.example.com/www.sec.gov/files/data.json");
    }

    #[test]
    fn test_build_url_data_sec_gov() {
        let mut config = Config::new("Company contact@example.com");
        config.base_url = "https://proxy.example.com/".to_string();
        let url = config.build_url("https://data.sec.gov/api/xbrl/companyfacts/CIK0000320193.json");
        assert_eq!(url, "https://proxy.example.com/data.sec.gov/api/xbrl/companyfacts/CIK0000320193.json");
    }

    #[test]
    fn test_build_url_non_https() {
        let mut config = Config::new("Company contact@example.com");
        config.base_url = "https://proxy.example.com/".to_string();
        let url = config.build_url("http://example.com/test");
        assert_eq!(url, "http://example.com/test");
    }
}