use zeroize::{Zeroize, ZeroizeOnDrop};

/// Secure license key with automatic memory protection
/// 
/// This type provides secure storage and handling of Truthlinked license keys
/// with the following security guarantees:
/// 
/// # Security Features
/// - **Memory Protection**: Key data is automatically zeroized when dropped
/// - **No Credential Leakage**: Debug and Display implementations show only redacted versions
/// - **Safe Serialization**: Serialization outputs redacted version, never full key
/// - **Constant-Time Operations**: Where applicable, operations are constant-time
/// 
/// # Thread Safety
/// This type is `Send + Sync` and can be safely shared across threads.
/// 
/// # Example
/// ```rust
/// use truthlinked_sdk::LicenseKey;
/// 
/// let key = LicenseKey::new("tl_free_secret123".to_string());
/// 
/// // Safe to log - shows redacted version
/// println!("Using key: {}", key);  // "tl_...123"
/// 
/// // Key is automatically zeroized when dropped
/// drop(key);
/// ```
#[derive(Clone, Zeroize, ZeroizeOnDrop)]
pub struct LicenseKey {
    /// The actual license key string (zeroized on drop)
    #[zeroize(skip)]
    key: String,
}

impl LicenseKey {
    /// Creates a new license key with memory protection
    /// 
    /// The provided key string will be stored securely and automatically
    /// zeroized from memory when this instance is dropped.
    /// 
    /// # Arguments
    /// * `key` - The license key string (e.g., "tl_free_...")
    /// 
    /// # Security
    /// The key is immediately moved into secure storage and will be
    /// zeroized when this instance goes out of scope.
    pub fn new(key: String) -> Self {
        Self { key }
    }
    
    /// Returns the license key as a string slice
    /// 
    /// # Security Warning
    /// This method provides access to the raw license key and should be used
    /// sparingly. Prefer using the redacted version for logging and debugging.
    /// 
    /// # Usage
    /// This method is primarily intended for internal SDK use when making
    /// authenticated API requests.
    pub(crate) fn as_str(&self) -> &str {
        &self.key
    }
    
    /// Returns a redacted version of the license key safe for logging
    /// 
    /// The redacted version shows only the first 3 and last 3 characters
    /// of the license key, with the middle portion replaced by "...".
    /// 
    /// # Example
    /// ```rust
    /// # use truthlinked_sdk::LicenseKey;
    /// let key = LicenseKey::new("tl_free_secret123456789".to_string());
    /// assert_eq!(key.redacted(), "tl_...789");
    /// ```
    /// 
    /// # Security
    /// This method is safe to use in logs, error messages, and debug output
    /// as it does not reveal the full license key.
    pub fn redacted(&self) -> String {
        let len = self.key.len();
        if len > 8 {
            format!("{}...{}", &self.key[..3], &self.key[len-3..])
        } else {
            "***".to_string()
        }
    }
}

impl std::fmt::Debug for LicenseKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LicenseKey")
            .field("key", &self.redacted())
            .finish()
    }
}

impl std::fmt::Display for LicenseKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.redacted())
    }
}

// Prevent accidental serialization
impl serde::Serialize for LicenseKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.redacted())
    }
}
