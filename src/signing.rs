use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};
use base64::Engine;

type HmacSha256 = Hmac<Sha256>;

/// Request signing for replay attack prevention
pub struct RequestSigner {
    signing_key: [u8; 32],
}

impl RequestSigner {
    /// Create new request signer from license key
    pub fn new(license_key: &str) -> Self {
        // Derive signing key from license key using HMAC
        let mut mac = HmacSha256::new_from_slice(b"truthlinked-request-signing-v1")
            .expect("HMAC can take key of any size");
        mac.update(license_key.as_bytes());
        let result = mac.finalize();
        
        let mut signing_key = [0u8; 32];
        signing_key.copy_from_slice(&result.into_bytes());
        
        Self { signing_key }
    }
    
    /// Sign a request with timestamp and body
    pub fn sign_request(
        &self,
        method: &str,
        path: &str,
        body: &[u8],
        timestamp: u64,
    ) -> String {
        let mut mac = HmacSha256::new_from_slice(&self.signing_key)
            .expect("Valid key length");
        
        // Sign: METHOD\nPATH\nTIMESTAMP\nBODY
        mac.update(method.as_bytes());
        mac.update(b"\n");
        mac.update(path.as_bytes());
        mac.update(b"\n");
        mac.update(timestamp.to_string().as_bytes());
        mac.update(b"\n");
        mac.update(body);
        
        let signature = mac.finalize();
        base64::engine::general_purpose::STANDARD.encode(signature.into_bytes())
    }
    
    /// Get current timestamp
    pub fn current_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_request_signing() {
        let signer = RequestSigner::new("test_key");
        let signature = signer.sign_request("GET", "/health", b"", 1234567890);
        
        // Should be deterministic
        let signature2 = signer.sign_request("GET", "/health", b"", 1234567890);
        assert_eq!(signature, signature2);
        
        // Different timestamp should produce different signature
        let signature3 = signer.sign_request("GET", "/health", b"", 1234567891);
        assert_ne!(signature, signature3);
    }
}
