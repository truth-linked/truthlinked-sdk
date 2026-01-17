use std::time::{Duration, Instant};
use tracing::{debug, error, info, warn};

/// Logging configuration for requests and responses
#[derive(Debug, Clone)]
pub struct LoggingConfig {
    /// Enable request logging
    pub log_requests: bool,
    /// Enable response logging
    pub log_responses: bool,
    /// Enable error logging
    pub log_errors: bool,
    /// Enable timing information
    pub log_timing: bool,
    /// Maximum body size to log (bytes)
    pub max_body_size: usize,
    /// Log level for successful requests
    pub success_level: LogLevel,
    /// Log level for failed requests
    pub error_level: LogLevel,
}

#[derive(Debug, Clone)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            log_requests: true,
            log_responses: true,
            log_errors: true,
            log_timing: true,
            max_body_size: 1024, // 1KB
            success_level: LogLevel::Debug,
            error_level: LogLevel::Error,
        }
    }
}

impl LoggingConfig {
    /// Production logging config (minimal logging)
    pub fn production() -> Self {
        Self {
            log_requests: false,
            log_responses: false,
            log_errors: true,
            log_timing: true,
            max_body_size: 0,
            success_level: LogLevel::Debug,
            error_level: LogLevel::Error,
        }
    }
    
    /// Development logging config (verbose logging)
    pub fn development() -> Self {
        Self {
            log_requests: true,
            log_responses: true,
            log_errors: true,
            log_timing: true,
            max_body_size: 4096, // 4KB
            success_level: LogLevel::Info,
            error_level: LogLevel::Error,
        }
    }
    
    /// Disable all logging
    pub fn none() -> Self {
        Self {
            log_requests: false,
            log_responses: false,
            log_errors: false,
            log_timing: false,
            max_body_size: 0,
            success_level: LogLevel::Debug,
            error_level: LogLevel::Error,
        }
    }
}

/// Request/response logger with credential redaction
pub struct RequestLogger {
    config: LoggingConfig,
}

impl RequestLogger {
    pub fn new(config: LoggingConfig) -> Self {
        Self { config }
    }
    
    /// Log outgoing request
    pub fn log_request(&self, method: &str, url: &str, headers: &[(&str, &str)], body: &[u8]) {
        if !self.config.log_requests {
            return;
        }
        
        let safe_headers = self.redact_headers(headers);
        let safe_body = self.redact_body(body);
        
        match self.config.success_level {
            LogLevel::Debug => debug!(
                method = method,
                url = url,
                headers = ?safe_headers,
                body = safe_body,
                "Sending request"
            ),
            LogLevel::Info => info!(
                method = method,
                url = url,
                "Sending request"
            ),
            LogLevel::Warn => warn!(
                method = method,
                url = url,
                "Sending request"
            ),
            LogLevel::Error => error!(
                method = method,
                url = url,
                "Sending request"
            ),
        }
    }
    
    /// Log incoming response
    pub fn log_response(&self, status: u16, headers: &[(&str, &str)], body: &[u8], duration: Duration) {
        if !self.config.log_responses {
            return;
        }
        
        let safe_headers = self.redact_headers(headers);
        let safe_body = self.redact_body(body);
        let duration_ms = duration.as_millis();
        
        let log_level = if status >= 400 {
            &self.config.error_level
        } else {
            &self.config.success_level
        };
        
        match log_level {
            LogLevel::Debug => debug!(
                status = status,
                duration_ms = duration_ms,
                headers = ?safe_headers,
                body = safe_body,
                "Received response"
            ),
            LogLevel::Info => info!(
                status = status,
                duration_ms = duration_ms,
                "Received response"
            ),
            LogLevel::Warn => warn!(
                status = status,
                duration_ms = duration_ms,
                "Received response"
            ),
            LogLevel::Error => error!(
                status = status,
                duration_ms = duration_ms,
                "Received response"
            ),
        }
    }
    
    /// Log request error with timing information
    pub fn log_error(&self, method: &str, url: &str, error: &str, duration: Duration) {
        if !self.config.log_errors {
            return;
        }
        
        let duration_ms = duration.as_millis();
        
        match self.config.error_level {
            LogLevel::Debug => debug!(
                method = method,
                url = url,
                error = error,
                duration_ms = duration_ms,
                "Request failed"
            ),
            LogLevel::Info => info!(
                method = method,
                url = url,
                error = error,
                duration_ms = duration_ms,
                "Request failed"
            ),
            LogLevel::Warn => warn!(
                method = method,
                url = url,
                error = error,
                duration_ms = duration_ms,
                "Request failed"
            ),
            LogLevel::Error => error!(
                method = method,
                url = url,
                error = error,
                duration_ms = duration_ms,
                "Request failed"
            ),
        }
    }
    
    /// Redact sensitive headers
    pub fn redact_headers(&self, headers: &[(&str, &str)]) -> Vec<(String, String)> {
        headers.iter().map(|(name, value)| {
            let safe_value = if name.to_lowercase().contains("authorization") 
                || name.to_lowercase().contains("cookie")
                || name.to_lowercase().contains("token") {
                self.redact_credential(value)
            } else {
                value.to_string()
            };
            (name.to_string(), safe_value)
        }).collect()
    }
    
    /// Redact request/response body
    pub fn redact_body(&self, body: &[u8]) -> String {
        if body.is_empty() {
            return "".to_string();
        }
        
        if body.len() > self.config.max_body_size {
            return format!("<body too large: {} bytes>", body.len());
        }
        
        match std::str::from_utf8(body) {
            Ok(text) => {
                // Simple credential redaction for common patterns
                let mut result = text.to_string();
                
                // Redact sso_token values
                if let Some(start) = result.find(r#""sso_token":""#) {
                    let value_start = start + r#""sso_token":""#.len();
                    if let Some(end) = result[value_start..].find('"') {
                        let value_end = value_start + end;
                        result.replace_range(value_start..value_end, "***");
                    }
                }
                
                // Redact af_token values
                if let Some(start) = result.find(r#""af_token":""#) {
                    let value_start = start + r#""af_token":""#.len();
                    if let Some(end) = result[value_start..].find('"') {
                        let value_end = value_start + end;
                        result.replace_range(value_start..value_end, "***");
                    }
                }
                
                // Redact license_key values
                if let Some(start) = result.find(r#""license_key":""#) {
                    let value_start = start + r#""license_key":""#.len();
                    if let Some(end) = result[value_start..].find('"') {
                        let value_end = value_start + end;
                        result.replace_range(value_start..value_end, "***");
                    }
                }
                
                result
            }
            Err(_) => format!("<binary data: {} bytes>", body.len()),
        }
    }
    
    /// Redact credential values
    pub fn redact_credential(&self, value: &str) -> String {
        if value.len() <= 8 {
            "***".to_string()
        } else {
            // Special case for Bearer tokens - use 4 chars at end
            if value.starts_with("Bearer ") {
                format!("{}...{}", &value[..3], &value[value.len()-4..])
            } else {
                format!("{}...{}", &value[..3], &value[value.len()-3..])
            }
        }
    }
}

/// Request timing tracker
pub struct RequestTimer {
    start: Instant,
}

impl RequestTimer {
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
        }
    }
    
    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_header_redaction() {
        let logger = RequestLogger::new(LoggingConfig::development());
        
        let headers = vec![
            ("Content-Type", "application/json"),
            ("Authorization", "Bearer tl_free_secret123456789"),
            ("X-Custom", "safe-value"),
        ];
        
        let redacted = logger.redact_headers(&headers);
        
        assert_eq!(redacted[0].1, "application/json");
        assert_eq!(redacted[1].1, "Bea...6789");
        assert_eq!(redacted[2].1, "safe-value");
    }
    
    #[test]
    fn test_body_redaction() {
        let logger = RequestLogger::new(LoggingConfig::development());
        
        let body = r#"{"sso_token":"secret123","other":"safe"}"#.as_bytes();
        let redacted = logger.redact_body(body);
        
        assert!(redacted.contains(r#""sso_token":"***""#));
        assert!(redacted.contains(r#""other":"safe""#));
    }
}
