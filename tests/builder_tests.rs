use truthlinked_sdk::*;
use std::time::Duration;

#[cfg(test)]
mod builder_tests {
    use super::*;
    
    #[test]
    fn test_builder_basic() {
        let _builder = ClientBuilder::new("https://api.example.com", "test_key");
        // Basic construction should work
    }
    
    #[test]
    fn test_builder_fluent_interface() {
        let _builder = ClientBuilder::new("https://api.example.com", "test_key")
            .timeout(Duration::from_secs(60))
            .retries(5)
            .user_agent("TestApp/1.0")
            .enable_logging();
        
        // Fluent interface should work without issues
    }
    
    #[test]
    fn test_builder_header_addition() {
        let _builder = ClientBuilder::new("https://api.example.com", "test_key")
            .header("X-Custom-Header", "custom-value")
            .unwrap()
            .header("X-Another-Header", "another-value")
            .unwrap();
        
        // Multiple headers should be addable
    }
    
    #[test]
    fn test_builder_invalid_header_name() {
        let result = ClientBuilder::new("https://api.example.com", "test_key")
            .header("Invalid Header Name", "value");
        
        assert!(result.is_err());
        match result.unwrap_err() {
            TruthlinkedError::InvalidRequest(msg) => {
                assert!(msg.contains("Invalid header name"));
            }
            _ => panic!("Expected InvalidRequest error"),
        }
    }
    
    #[test]
    fn test_builder_invalid_header_value() {
        let result = ClientBuilder::new("https://api.example.com", "test_key")
            .header("X-Custom", "invalid\nvalue");
        
        assert!(result.is_err());
        match result.unwrap_err() {
            TruthlinkedError::InvalidRequest(msg) => {
                assert!(msg.contains("Invalid header value"));
            }
            _ => panic!("Expected InvalidRequest error"),
        }
    }
    
    #[test]
    fn test_builder_presets() {
        let _prod_builder = ClientBuilder::production("https://api.example.com", "key");
        let _dev_builder = ClientBuilder::development("https://api.example.com", "key");
        let _test_builder = ClientBuilder::testing("https://api.example.com", "key");
        
        // All presets should construct without issues
    }
    
    #[test]
    fn test_builder_build_https_enforcement() {
        let result = ClientBuilder::new("http://insecure.example.com", "key").build();
        
        assert!(result.is_err());
        match result.unwrap_err() {
            TruthlinkedError::InvalidRequest(msg) => {
                assert!(msg.contains("HTTPS"));
            }
            _ => panic!("Expected InvalidRequest error for HTTP URL"),
        }
    }
    
    #[test]
    fn test_builder_build_success() {
        let result = ClientBuilder::new("https://api.example.com", "test_key").build();
        
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_builder_with_proxy() {
        let builder = ClientBuilder::new("https://api.example.com", "test_key")
            .proxy("http://proxy.example.com:8080");
        
        // Should accept proxy configuration
        let result = builder.build();
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_builder_pool_config() {
        let builder = ClientBuilder::new("https://api.example.com", "test_key")
            .pool_config(20, Duration::from_secs(120));
        
        let result = builder.build();
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_builder_compression_settings() {
        let builder = ClientBuilder::new("https://api.example.com", "test_key")
            .gzip(true)
            .brotli(false);
        
        let result = builder.build();
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_builder_certificate_pinning() {
        let builder = ClientBuilder::new("https://api.example.com", "test_key")
            .certificate_pin("sha256/AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=");
        
        let result = builder.build();
        assert!(result.is_ok());
        // Note: Certificate pinning is not yet implemented in reqwest,
        // so this just tests that the API accepts the configuration
    }
    
    #[test]
    fn test_builder_retry_config() {
        let retry_config = RetryConfig::aggressive();
        let builder = ClientBuilder::new("https://api.example.com", "test_key")
            .retry_config(retry_config);
        
        let result = builder.build();
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_builder_logging_config() {
        let logging_config = LoggingConfig::development();
        let builder = ClientBuilder::new("https://api.example.com", "test_key")
            .logging_config(logging_config);
        
        let result = builder.build();
        assert!(result.is_ok());
    }
}
