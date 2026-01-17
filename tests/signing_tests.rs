use truthlinked_sdk::*;

#[cfg(test)]
mod signing_tests {
    use super::*;
    
    #[test]
    fn test_request_signing_deterministic() {
        let signer = RequestSigner::new("test_key");
        let signature1 = signer.sign_request("GET", "/health", b"", 1234567890);
        let signature2 = signer.sign_request("GET", "/health", b"", 1234567890);
        
        assert_eq!(signature1, signature2, "Signatures should be deterministic");
    }
    
    #[test]
    fn test_request_signing_different_methods() {
        let signer = RequestSigner::new("test_key");
        let get_sig = signer.sign_request("GET", "/health", b"", 1234567890);
        let post_sig = signer.sign_request("POST", "/health", b"", 1234567890);
        
        assert_ne!(get_sig, post_sig, "Different methods should produce different signatures");
    }
    
    #[test]
    fn test_request_signing_different_paths() {
        let signer = RequestSigner::new("test_key");
        let health_sig = signer.sign_request("GET", "/health", b"", 1234567890);
        let tokens_sig = signer.sign_request("GET", "/tokens", b"", 1234567890);
        
        assert_ne!(health_sig, tokens_sig, "Different paths should produce different signatures");
    }
    
    #[test]
    fn test_request_signing_different_timestamps() {
        let signer = RequestSigner::new("test_key");
        let sig1 = signer.sign_request("GET", "/health", b"", 1234567890);
        let sig2 = signer.sign_request("GET", "/health", b"", 1234567891);
        
        assert_ne!(sig1, sig2, "Different timestamps should produce different signatures");
    }
    
    #[test]
    fn test_request_signing_different_bodies() {
        let signer = RequestSigner::new("test_key");
        let empty_sig = signer.sign_request("POST", "/tokens", b"", 1234567890);
        let body_sig = signer.sign_request("POST", "/tokens", b"{\"test\":\"data\"}", 1234567890);
        
        assert_ne!(empty_sig, body_sig, "Different bodies should produce different signatures");
    }
    
    #[test]
    fn test_request_signing_different_keys() {
        let signer1 = RequestSigner::new("key1");
        let signer2 = RequestSigner::new("key2");
        
        let sig1 = signer1.sign_request("GET", "/health", b"", 1234567890);
        let sig2 = signer2.sign_request("GET", "/health", b"", 1234567890);
        
        assert_ne!(sig1, sig2, "Different keys should produce different signatures");
    }
    
    #[test]
    fn test_current_timestamp() {
        let ts1 = RequestSigner::current_timestamp();
        std::thread::sleep(std::time::Duration::from_millis(1));
        let ts2 = RequestSigner::current_timestamp();
        
        assert!(ts2 >= ts1, "Timestamps should be monotonic");
    }
    
    #[test]
    fn test_signature_format() {
        let signer = RequestSigner::new("test_key");
        let signature = signer.sign_request("GET", "/health", b"", 1234567890);
        
        // Base64 encoded signatures should be valid base64
        assert!(base64::Engine::decode(&base64::engine::general_purpose::STANDARD, &signature).is_ok());
        
        // Should be 44 characters (32 bytes base64 encoded)
        assert_eq!(signature.len(), 44);
    }
}
