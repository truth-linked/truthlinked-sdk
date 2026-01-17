use thiserror::Error;

/// Errors that can occur when using the Truthlinked SDK
/// 
/// All error messages are designed to be safe for logging and do not leak
/// sensitive information such as license keys or internal system details.
/// 
/// # Security Considerations
/// - No credential information is included in error messages
/// - Internal system details are not exposed
/// - Error messages are safe to log and display to users
/// - Stack traces do not contain sensitive information
#[derive(Error, Debug)]
pub enum TruthlinkedError {
    /// Network-related errors (connection failures, timeouts, DNS resolution)
    /// 
    /// This error indicates a problem with network connectivity between the
    /// client and the Truthlinked API. Common causes include:
    /// - Internet connectivity issues
    /// - DNS resolution failures
    /// - Firewall blocking HTTPS traffic
    /// - API server temporarily unavailable
    #[error("Network error: {0}")]
    Network(String),
    
    /// Authentication failed due to invalid or expired license key
    /// 
    /// This error occurs when:
    /// - License key is malformed or invalid
    /// - License key has expired
    /// - License key signature verification fails
    /// 
    /// Resolution: Verify your license key and ensure it hasn't expired
    #[error("Authentication failed")]
    Unauthorized,
    
    /// Access denied due to insufficient license tier permissions
    /// 
    /// This error occurs when your license tier doesn't include access to
    /// the requested operation. For example:
    /// - Free tier attempting token exchange (requires Professional+)
    /// - Professional tier attempting enforcement (requires Enterprise+)
    /// 
    /// Resolution: Upgrade your license tier or use a different endpoint
    #[error("Access denied: insufficient tier permissions")]
    Forbidden,
    
    /// Rate limit exceeded for your license tier
    /// 
    /// Each license tier has monthly request limits:
    /// - Free: 1,000 requests/month
    /// - Professional: 500,000 requests/month
    /// - Enterprise: Unlimited
    /// 
    /// Resolution: Wait for the limit to reset or upgrade your tier
    #[error("Rate limit exceeded: {0}")]
    RateLimitExceeded(String),
    
    /// Request validation failed due to invalid parameters
    /// 
    /// This error indicates that the request parameters are malformed,
    /// missing required fields, or contain invalid values.
    /// 
    /// Resolution: Check the API documentation and verify request parameters
    #[error("Invalid request: {0}")]
    InvalidRequest(String),
    
    /// Internal server error occurred
    /// 
    /// This indicates a problem on the Truthlinked API server side.
    /// The issue has been logged and will be investigated.
    /// 
    /// Resolution: Retry the request after a brief delay
    #[error("Server error")]
    ServerError,
    
    /// Failed to serialize or deserialize request/response data
    /// 
    /// This error indicates a problem with JSON serialization/deserialization,
    /// typically due to unexpected response format from the server.
    #[error("Serialization error")]
    SerializationError,
    
    /// Received an invalid or unexpected response from the server
    /// 
    /// This error occurs when the server returns a response that doesn't
    /// match the expected format or contains invalid data.
    #[error("Invalid response from server")]
    InvalidResponse,
    
    /// License has expired and needs to be renewed
    /// 
    /// Your license key has passed its expiration date and is no longer valid.
    /// 
    /// Resolution: Contact support to renew your license
    #[error("License expired")]
    LicenseExpired,
}

impl From<reqwest::Error> for TruthlinkedError {
    fn from(err: reqwest::Error) -> Self {
        // Don't leak internal details
        if err.is_timeout() {
            TruthlinkedError::Network("Request timeout".to_string())
        } else if err.is_connect() {
            TruthlinkedError::Network("Connection failed".to_string())
        } else {
            TruthlinkedError::Network("Network error".to_string())
        }
    }
}

impl From<serde_json::Error> for TruthlinkedError {
    fn from(_: serde_json::Error) -> Self {
        // Don't leak JSON structure
        TruthlinkedError::SerializationError
    }
}

pub type Result<T> = std::result::Result<T, TruthlinkedError>;
