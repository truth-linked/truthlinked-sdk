use truthlinked_sdk::*;
use std::time::Duration;

#[cfg(test)]
mod logging_tests {
    use super::*;
    
    #[test]
    fn test_logging_config_presets() {
        let prod = LoggingConfig::production();
        assert!(!prod.log_requests);
        assert!(!prod.log_responses);
        assert!(prod.log_errors);
        assert_eq!(prod.max_body_size, 0);
        
        let dev = LoggingConfig::development();
        assert!(dev.log_requests);
        assert!(dev.log_responses);
        assert!(dev.log_errors);
        assert_eq!(dev.max_body_size, 4096);
        
        let none = LoggingConfig::none();
        assert!(!none.log_requests);
        assert!(!none.log_responses);
        assert!(!none.log_errors);
    }
    
    #[test]
    fn test_header_redaction() {
        let logger = RequestLogger::new(LoggingConfig::development());
        
        let headers = vec![
            ("Content-Type", "application/json"),
            ("Authorization", "Bearer tl_free_secret123456789"),
            ("X-Custom", "safe-value"),
            ("Cookie", "session=secret123456789"),
        ];
        
        let redacted = logger.redact_headers(&headers);
        
        assert_eq!(redacted[0].1, "application/json");
        assert_eq!(redacted[1].1, "Bea...6789");
        assert_eq!(redacted[2].1, "safe-value");
        assert_eq!(redacted[3].1, "ses...789");
    }
    
    #[test]
    fn test_body_redaction_json() {
        let logger = RequestLogger::new(LoggingConfig::development());
        
        let body = r#"{"sso_token":"secret123","af_token":"token456","other":"safe"}"#.as_bytes();
        let redacted = logger.redact_body(body);
        
        assert!(redacted.contains(r#""sso_token":"***""#));
        assert!(redacted.contains(r#""af_token":"***""#));
        assert!(redacted.contains(r#""other":"safe""#));
    }
    
    #[test]
    fn test_body_redaction_large() {
        let config = LoggingConfig {
            max_body_size: 10,
            ..LoggingConfig::development()
        };
        let logger = RequestLogger::new(config);
        
        let large_body = "This is a very long body that exceeds the limit".as_bytes();
        let redacted = logger.redact_body(large_body);
        
        assert!(redacted.contains("body too large"));
        assert!(redacted.contains("47 bytes"));
    }
    
    #[test]
    fn test_body_redaction_binary() {
        let logger = RequestLogger::new(LoggingConfig::development());
        
        let binary_body = vec![0u8, 1u8, 2u8, 255u8];
        let redacted = logger.redact_body(&binary_body);
        
        assert!(redacted.contains("binary data"));
        assert!(redacted.contains("4 bytes"));
    }
    
    #[test]
    fn test_body_redaction_empty() {
        let logger = RequestLogger::new(LoggingConfig::development());
        
        let empty_body = b"";
        let redacted = logger.redact_body(empty_body);
        
        assert_eq!(redacted, "");
    }
    
    #[test]
    fn test_credential_redaction() {
        let logger = RequestLogger::new(LoggingConfig::development());
        
        // Short credential
        let short = logger.redact_credential("abc");
        assert_eq!(short, "***");
        
        // Long credential
        let long = logger.redact_credential("tl_free_verylongkey123456789");
        assert_eq!(long, "tl_...789");
        
        // Medium credential
        let medium = logger.redact_credential("medium123");
        assert_eq!(medium, "med...123");
    }
    
    #[test]
    fn test_request_logging_disabled() {
        let config = LoggingConfig {
            log_requests: false,
            ..LoggingConfig::development()
        };
        let logger = RequestLogger::new(config);
        
        // This should not panic or cause issues when logging is disabled
        logger.log_request("GET", "https://example.com", &[], b"test");
    }
    
    #[test]
    fn test_response_logging_disabled() {
        let config = LoggingConfig {
            log_responses: false,
            ..LoggingConfig::development()
        };
        let logger = RequestLogger::new(config);
        
        // This should not panic or cause issues when logging is disabled
        logger.log_response(200, &[], b"response", Duration::from_millis(100));
    }
    
    #[test]
    fn test_error_logging_disabled() {
        let config = LoggingConfig {
            log_errors: false,
            ..LoggingConfig::development()
        };
        let logger = RequestLogger::new(config);
        
        // This should not panic or cause issues when logging is disabled
        logger.log_error("GET", "https://example.com", "error", Duration::from_millis(100));
    }
    
    #[test]
    fn test_log_level_selection() {
        let config = LoggingConfig {
            success_level: LogLevel::Info,
            error_level: LogLevel::Error,
            ..LoggingConfig::development()
        };
        let logger = RequestLogger::new(config);
        
        // Test that different status codes use different log levels
        logger.log_response(200, &[], b"ok", Duration::from_millis(100));
        logger.log_response(404, &[], b"not found", Duration::from_millis(100));
        logger.log_response(500, &[], b"error", Duration::from_millis(100));
    }
}
