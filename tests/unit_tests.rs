use truthlinked_sdk::*;

#[cfg(test)]
mod error_tests {
    use super::*;
    
    #[test]
    fn test_error_display() {
        let error = TruthlinkedError::Unauthorized;
        assert_eq!(error.to_string(), "Authentication failed");
        
        let error = TruthlinkedError::Forbidden;
        assert_eq!(error.to_string(), "Access denied: insufficient tier permissions");
        
        let error = TruthlinkedError::RateLimitExceeded("Too many requests".to_string());
        assert_eq!(error.to_string(), "Rate limit exceeded: Too many requests");
    }
    
    #[test]
    fn test_error_from_serde() {
        let json_error = serde_json::from_str::<serde_json::Value>("invalid json");
        assert!(json_error.is_err());
        
        let sdk_error: TruthlinkedError = json_error.unwrap_err().into();
        match sdk_error {
            TruthlinkedError::SerializationError => {},
            _ => panic!("Expected SerializationError"),
        }
    }
}

#[cfg(test)]
mod license_tests {
    use super::*;
    
    #[test]
    fn test_license_key_creation() {
        let key = LicenseKey::new("tl_free_test123456789".to_string());
        assert_eq!(key.redacted(), "tl_...789");
    }
    
    #[test]
    fn test_license_key_redaction() {
        let key = LicenseKey::new("short".to_string());
        assert_eq!(key.redacted(), "***");
        
        let key = LicenseKey::new("tl_free_verylongkey123456789".to_string());
        assert_eq!(key.redacted(), "tl_...789");
    }
    
    #[test]
    fn test_license_key_debug() {
        let key = LicenseKey::new("tl_free_secret123456789".to_string());
        let debug_str = format!("{:?}", key);
        assert!(debug_str.contains("tl_...789"));
        assert!(!debug_str.contains("secret"));
    }
    
    #[test]
    fn test_license_key_display() {
        let key = LicenseKey::new("tl_free_secret123456789".to_string());
        let display_str = format!("{}", key);
        assert_eq!(display_str, "tl_...789");
    }
    
    #[test]
    fn test_license_key_serialization() {
        let key = LicenseKey::new("tl_free_secret123456789".to_string());
        let json = serde_json::to_string(&key).unwrap();
        assert!(json.contains("tl_...789"));
        assert!(!json.contains("secret"));
    }
}

#[cfg(test)]
mod types_tests {
    use super::*;
    
    #[test]
    fn test_tier_serialization() {
        let tier = Tier::Free;
        let json = serde_json::to_string(&tier).unwrap();
        assert_eq!(json, "\"free\"");
        
        let tier = Tier::Professional;
        let json = serde_json::to_string(&tier).unwrap();
        assert_eq!(json, "\"professional\"");
    }
    
    #[test]
    fn test_tier_deserialization() {
        let tier: Tier = serde_json::from_str("\"free\"").unwrap();
        assert_eq!(tier, Tier::Free);
        
        let tier: Tier = serde_json::from_str("\"enterprise\"").unwrap();
        assert_eq!(tier, Tier::Enterprise);
    }
    
    #[test]
    fn test_health_response() {
        let health = HealthResponse {
            status: "healthy".to_string(),
            version: "1.0.0".to_string(),
        };
        
        let json = serde_json::to_string(&health).unwrap();
        let parsed: HealthResponse = serde_json::from_str(&json).unwrap();
        
        assert_eq!(parsed.status, "healthy");
        assert_eq!(parsed.version, "1.0.0");
    }
    
    #[test]
    fn test_token_request() {
        let request = TokenRequest {
            sso_token: "test_token".to_string(),
            requested_scope: vec!["read".to_string(), "write".to_string()],
            nonce: "deadbeef".to_string(),
            channel_binding: "cafebabe".to_string(),
        };
        
        let json = serde_json::to_string(&request).unwrap();
        let parsed: TokenRequest = serde_json::from_str(&json).unwrap();
        
        assert_eq!(parsed.sso_token, "test_token");
        assert_eq!(parsed.requested_scope.len(), 2);
        assert_eq!(parsed.nonce, "deadbeef");
    }
}
