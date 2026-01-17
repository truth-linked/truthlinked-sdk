use crate::error::{Result, TruthlinkedError};
use crate::logging::LoggingConfig;
use crate::retry::RetryConfig;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::time::Duration;

/// Builder for configuring Truthlinked API client
/// 
/// Provides a fluent interface for configuring all client options including
/// timeouts, retries, logging, custom headers, and security settings.
/// 
/// # Example
/// ```rust,no_run
/// use truthlinked_sdk::ClientBuilder;
/// use std::time::Duration;
/// 
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = ClientBuilder::new("https://api.truthlinked.org", "tl_free_...")
///     .timeout(Duration::from_secs(60))
///     .retries(5)
///     .user_agent("MyApp/1.0")
///     .header("X-Request-ID", "12345")?
///     .enable_logging()
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct ClientBuilder {
    base_url: String,
    license_key: String,
    timeout: Duration,
    connect_timeout: Duration,
    retry_config: RetryConfig,
    logging_config: LoggingConfig,
    custom_headers: HeaderMap,
    user_agent: Option<String>,
    proxy_url: Option<String>,
    pool_max_idle_per_host: usize,
    pool_idle_timeout: Duration,
    enable_gzip: bool,
    enable_brotli: bool,
    certificate_pins: Vec<String>,
    allow_http: bool,  // For testing only
}

impl ClientBuilder {
    /// Create a new client builder
    /// 
    /// # Arguments
    /// * `base_url` - API base URL (must be HTTPS)
    /// * `license_key` - Your Truthlinked license key
    pub fn new(base_url: impl Into<String>, license_key: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            license_key: license_key.into(),
            timeout: Duration::from_secs(30),
            connect_timeout: Duration::from_secs(10),
            retry_config: RetryConfig::production(),
            logging_config: LoggingConfig::production(),
            custom_headers: HeaderMap::new(),
            user_agent: None,
            proxy_url: None,
            pool_max_idle_per_host: 10,
            pool_idle_timeout: Duration::from_secs(90),
            enable_gzip: true,
            enable_brotli: true,
            certificate_pins: Vec::new(),
            allow_http: false,
        }
    }
    
    /// Set request timeout
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
    
    /// Set connection timeout
    pub fn connect_timeout(mut self, timeout: Duration) -> Self {
        self.connect_timeout = timeout;
        self
    }
    
    /// Set retry configuration
    pub fn retry_config(mut self, config: RetryConfig) -> Self {
        self.retry_config = config;
        self
    }
    
    /// Set number of retry attempts (convenience method)
    pub fn retries(mut self, max_attempts: u32) -> Self {
        self.retry_config.max_attempts = max_attempts;
        self
    }
    
    /// Set logging configuration
    pub fn logging_config(mut self, config: LoggingConfig) -> Self {
        self.logging_config = config;
        self
    }
    
    /// Enable development logging (convenience method)
    pub fn enable_logging(mut self) -> Self {
        self.logging_config = LoggingConfig::development();
        self
    }
    
    /// Disable all logging (convenience method)
    pub fn disable_logging(mut self) -> Self {
        self.logging_config = LoggingConfig::none();
        self
    }
    
    /// Add custom header
    pub fn header(mut self, name: impl AsRef<str>, value: impl AsRef<str>) -> Result<Self> {
        let header_name = HeaderName::from_bytes(name.as_ref().as_bytes())
            .map_err(|_| TruthlinkedError::InvalidRequest("Invalid header name".to_string()))?;
        let header_value = HeaderValue::from_str(value.as_ref())
            .map_err(|_| TruthlinkedError::InvalidRequest("Invalid header value".to_string()))?;
        
        self.custom_headers.insert(header_name, header_value);
        Ok(self)
    }
    
    /// Set User-Agent header
    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = Some(user_agent.into());
        self
    }
    
    /// Set HTTP proxy URL
    pub fn proxy(mut self, proxy_url: impl Into<String>) -> Self {
        self.proxy_url = Some(proxy_url.into());
        self
    }
    
    /// Set connection pool configuration
    pub fn pool_config(mut self, max_idle_per_host: usize, idle_timeout: Duration) -> Self {
        self.pool_max_idle_per_host = max_idle_per_host;
        self.pool_idle_timeout = idle_timeout;
        self
    }
    
    /// Enable/disable gzip compression
    pub fn gzip(mut self, enable: bool) -> Self {
        self.enable_gzip = enable;
        self
    }
    
    /// Enable/disable brotli compression
    pub fn brotli(mut self, enable: bool) -> Self {
        self.enable_brotli = enable;
        self
    }
    
    /// Add certificate pin for enhanced security
    /// 
    /// # Arguments
    /// * `pin` - SHA256 hash of the certificate's public key (base64 encoded)
    pub fn certificate_pin(mut self, pin: impl Into<String>) -> Self {
        self.certificate_pins.push(pin.into());
        self
    }
    
    /// Build the configured client
    pub fn build(self) -> Result<crate::client::Client> {
        // Validate base URL (allow HTTP only in testing mode)
        if !self.allow_http && !self.base_url.starts_with("https://") {
            return Err(TruthlinkedError::InvalidRequest(
                "Base URL must use HTTPS".to_string()
            ));
        }
        
        // Build HTTP client
        let mut client_builder = reqwest::Client::builder()
            .timeout(self.timeout)
            .connect_timeout(self.connect_timeout)
            .pool_idle_timeout(self.pool_idle_timeout)
            .pool_max_idle_per_host(self.pool_max_idle_per_host)
            .https_only(!self.allow_http)  // Allow HTTP only in testing mode
            .default_headers(self.custom_headers);
        
        // Set user agent
        if let Some(user_agent) = self.user_agent {
            client_builder = client_builder.user_agent(user_agent);
        } else {
            client_builder = client_builder.user_agent(format!(
                "truthlinked-sdk/{}", 
                env!("CARGO_PKG_VERSION")
            ));
        }
        
        // Set proxy if configured
        if let Some(proxy_url) = self.proxy_url {
            let proxy = reqwest::Proxy::all(&proxy_url)
                .map_err(|_| TruthlinkedError::InvalidRequest("Invalid proxy URL".to_string()))?;
            client_builder = client_builder.proxy(proxy);
        }
        
        // TODO: Implement certificate pinning when reqwest supports it
        if !self.certificate_pins.is_empty() {
            tracing::warn!("Certificate pinning not yet implemented in reqwest");
        }
        
        let http_client = client_builder.build()
            .map_err(|_| TruthlinkedError::InvalidRequest("Failed to build HTTP client".to_string()))?;
        
        crate::client::Client::with_config(
            http_client,
            self.base_url,
            self.license_key,
            self.retry_config,
            self.logging_config,
        )
    }
}

/// Convenience methods for common configurations
impl ClientBuilder {
    /// Production configuration with minimal logging and conservative timeouts
    pub fn production(base_url: impl Into<String>, license_key: impl Into<String>) -> Self {
        Self::new(base_url, license_key)
            .timeout(Duration::from_secs(30))
            .connect_timeout(Duration::from_secs(10))
            .retry_config(RetryConfig::production())
            .logging_config(LoggingConfig::production())
    }
    
    /// Development configuration with verbose logging and shorter timeouts
    pub fn development(base_url: impl Into<String>, license_key: impl Into<String>) -> Self {
        Self::new(base_url, license_key)
            .timeout(Duration::from_secs(60))
            .connect_timeout(Duration::from_secs(5))
            .retry_config(RetryConfig::aggressive())
            .logging_config(LoggingConfig::development())
    }
    
    /// Testing configuration with no retries and no logging
    pub fn testing(base_url: impl Into<String>, license_key: impl Into<String>) -> Self {
        let mut builder = Self::new(base_url, license_key)
            .timeout(Duration::from_secs(5))
            .connect_timeout(Duration::from_secs(2))
            .retry_config(RetryConfig::none())
            .logging_config(LoggingConfig::none());
        builder.allow_http = true;  // Allow HTTP for testing
        builder
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_builder_basic() {
        let builder = ClientBuilder::new("https://api.example.com", "test_key");
        assert_eq!(builder.base_url, "https://api.example.com");
        assert_eq!(builder.license_key, "test_key");
    }
    
    #[test]
    fn test_builder_fluent_interface() {
        let builder = ClientBuilder::new("https://api.example.com", "test_key")
            .timeout(Duration::from_secs(60))
            .retries(5)
            .user_agent("TestApp/1.0")
            .enable_logging();
        
        assert_eq!(builder.timeout, Duration::from_secs(60));
        assert_eq!(builder.retry_config.max_attempts, 5);
        assert_eq!(builder.user_agent, Some("TestApp/1.0".to_string()));
    }
    
    #[test]
    fn test_builder_presets() {
        let prod_builder = ClientBuilder::production("https://api.example.com", "key");
        assert_eq!(prod_builder.timeout, Duration::from_secs(30));
        
        let dev_builder = ClientBuilder::development("https://api.example.com", "key");
        assert_eq!(dev_builder.timeout, Duration::from_secs(60));
        
        let test_builder = ClientBuilder::testing("https://api.example.com", "key");
        assert_eq!(test_builder.timeout, Duration::from_secs(5));
    }
}
