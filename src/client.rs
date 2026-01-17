use crate::error::{Result, TruthlinkedError};
use crate::license::LicenseKey;
use crate::logging::{LoggingConfig, RequestLogger, RequestTimer};
use crate::retry::{RetryConfig, RetryExecutor};
use crate::signing::RequestSigner;
use crate::types::*;
use reqwest::{Client as HttpClient, StatusCode};
use std::time::Duration;

/// Truthlinked Authority Fabric API client
///
/// Provides type-safe access to the Truthlinked Authority Fabric API with
/// enterprise-grade security and reliability features.
///
/// # Security Features
/// - HTTPS-only communication (HTTP requests are rejected)
/// - TLS certificate validation (no self-signed certificates)
/// - License key memory protection (zeroized on drop)
/// - Safe error handling (no credential leakage)
/// - Connection pooling with reasonable limits
/// - Request timeouts to prevent hanging
///
/// # Thread Safety
/// This client is `Send + Sync` and can be safely shared across threads.
/// Consider using `Arc<Client>` for shared access.
///
/// # Example
/// ```rust,no_run
/// use truthlinked_sdk::Client;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = Client::new(
///         "https://api.truthlinked.org",
///         std::env::var("TRUTHLINKED_LICENSE_KEY")?
///     )?;
///     
///     let health = client.health().await?;
///     println!("Server status: {}", health.status);
///     Ok(())
/// }
/// ```
pub struct Client {
    /// HTTP client with security hardening and connection pooling
    http_client: HttpClient,
    /// Base URL for API requests (must be HTTPS)
    base_url: String,
    /// License key with automatic memory protection
    license_key: LicenseKey,
    /// Request signer for replay attack prevention
    signer: RequestSigner,
    /// Retry executor with exponential backoff
    retry_executor: RetryExecutor,
    /// Request/response logger with credential redaction
    logger: RequestLogger,
}

impl Client {
    /// Creates a new Truthlinked API client
    /// 
    /// # Arguments
    /// * `base_url` - API base URL (must be HTTPS)
    /// * `license_key` - Your Truthlinked license key
    /// 
    /// # Security Guarantees
    /// - Enforces HTTPS only (HTTP requests are rejected at client creation)
    /// - Uses rustls TLS implementation (no OpenSSL vulnerabilities)
    /// - Validates TLS certificates (rejects self-signed certificates)
    /// - Configures reasonable timeouts (prevents indefinite hanging)
    /// - Enables connection pooling (improves performance and reliability)
    /// 
    /// # Errors
    /// Returns `TruthlinkedError::InvalidRequest` if:
    /// - Base URL does not start with "https://"
    /// - HTTP client cannot be configured
    /// 
    /// # Example
    /// ```rust,no_run
    /// use truthlinked_sdk::Client;
    /// 
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::new(
    ///     "https://api.truthlinked.org",
    ///     "tl_free_..."
    /// )?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(base_url: impl Into<String>, license_key: impl Into<String>) -> Result<Self> {
        let base_url_string = base_url.into();
        let license_key_string = license_key.into();
        
        // Enforce HTTPS
        if !base_url_string.starts_with("https://") {
            return Err(TruthlinkedError::InvalidRequest(
                "Base URL must use HTTPS".to_string()
            ));
        }
        
        // Build HTTP client with security settings
        let http_client = HttpClient::builder()
            .timeout(Duration::from_secs(30))
            .connect_timeout(Duration::from_secs(10))
            .pool_idle_timeout(Duration::from_secs(90))
            .pool_max_idle_per_host(10)
            .https_only(true)  // Enforce HTTPS
            .build()?;
        
        Ok(Self {
            http_client,
            signer: RequestSigner::new(&license_key_string),
            base_url: base_url_string,
            license_key: LicenseKey::new(license_key_string),
            retry_executor: RetryExecutor::new(RetryConfig::production()),
            logger: RequestLogger::new(LoggingConfig::production()),
        })
    }
    
    /// Create client with custom configuration (used by ClientBuilder)
    pub(crate) fn with_config(
        http_client: HttpClient,
        base_url: String,
        license_key: String,
        retry_config: RetryConfig,
        logging_config: LoggingConfig,
    ) -> Result<Self> {
        Ok(Self {
            http_client,
            base_url,
            signer: RequestSigner::new(&license_key),
            license_key: LicenseKey::new(license_key),
            retry_executor: RetryExecutor::new(retry_config),
            logger: RequestLogger::new(logging_config),
        })
    }
    
    /// Performs a health check against the Truthlinked API
    /// 
    /// This endpoint does not require authentication and can be used to verify
    /// that the API is accessible and responding correctly.
    /// 
    /// # Returns
    /// - `Ok(HealthResponse)` - Server is healthy and responding
    /// - `Err(TruthlinkedError)` - Network error or server unavailable
    /// 
    /// # Example
    /// ```rust,no_run
    /// # use truthlinked_sdk::Client;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::new("https://api.truthlinked.org", "key")?;
    /// let health = client.health().await?;
    /// assert_eq!(health.status, "healthy");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn health(&self) -> Result<HealthResponse> {
        let url = format!("{}/health", self.base_url);
        
        self.retry_executor.execute(|| async {
            let timer = RequestTimer::new();
            let timestamp = RequestSigner::current_timestamp();
            let signature = self.signer.sign_request("GET", "/health", b"", timestamp);
            
            // Log request
            let timestamp_str = timestamp.to_string();
            let headers = vec![
                ("X-Timestamp", timestamp_str.as_str()),
                ("X-Signature", signature.as_str()),
            ];
            self.logger.log_request("GET", &url, &headers, b"");
            
            match self.http_client
                .get(&url)
                .header("X-Timestamp", timestamp.to_string())
                .header("X-Signature", signature)
                .send()
                .await
            {
                Ok(response) => {
                    let status = response.status().as_u16();
                    let response_headers = vec![]; // Would extract from response
                    
                    match response.status() {
                        StatusCode::OK => {
                            let body = response.bytes().await?;
                            self.logger.log_response(status, &response_headers, &body, timer.elapsed());
                            
                            let health: HealthResponse = serde_json::from_slice(&body)?;
                            Ok(health)
                        }
                        _ => {
                            let body = response.bytes().await?;
                            self.logger.log_response(status, &response_headers, &body, timer.elapsed());
                            self.handle_error_status(StatusCode::from_u16(status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR))
                        }
                    }
                }
                Err(e) => {
                    self.logger.log_error("GET", &url, &e.to_string(), timer.elapsed());
                    Err(e.into())
                }
            }
        }).await
    }
    
    /// Exchanges an SSO token for an Authority Fabric token
    /// 
    /// This operation requires a Professional tier license or higher.
    /// The SSO token is validated and, if successful, an AF token is issued
    /// with the requested scope (potentially narrowed based on policy).
    /// 
    /// # Arguments
    /// * `sso_token` - Valid SSO token from your identity provider
    /// * `requested_scope` - List of permissions requested (e.g., ["read:users"])
    /// * `nonce` - 32-byte cryptographic nonce (prevents replay attacks)
    /// * `channel_binding` - 32-byte channel binding (prevents MITM attacks)
    /// 
    /// # Security Notes
    /// - Nonce must be cryptographically random and unique per request
    /// - Channel binding should be derived from the TLS channel
    /// - The granted scope may be narrower than requested based on policy
    /// 
    /// # Errors
    /// - `Unauthorized` - Invalid license key or SSO token
    /// - `Forbidden` - License tier doesn't support token exchange
    /// - `InvalidRequest` - Malformed request parameters
    /// 
    /// # Example
    /// ```rust,no_run
    /// # use truthlinked_sdk::Client;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::new("https://api.truthlinked.org", "tl_pro_...")?;
    /// use rand::Rng;
    /// 
    /// let nonce: [u8; 32] = rand::thread_rng().gen();
    /// let channel_binding: [u8; 32] = rand::thread_rng().gen();
    /// 
    /// let response = client.exchange_token(
    ///     "eyJ0eXAiOiJKV1QiLCJhbGc...",
    ///     vec!["read:users".to_string()],
    ///     nonce,
    ///     channel_binding,
    /// ).await?;
    /// 
    /// println!("AF Token: {}", response.af_token);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn exchange_token(
        &self,
        sso_token: impl Into<String>,
        requested_scope: Vec<String>,
        nonce: [u8; 32],
        channel_binding: [u8; 32],
    ) -> Result<TokenResponse> {
        let url = format!("{}/v1/tokens", self.base_url);
        
        let request = TokenRequest {
            sso_token: sso_token.into(),
            requested_scope,
            nonce: hex::encode(nonce),
            channel_binding: hex::encode(channel_binding),
        };
        
        let response = self.http_client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.license_key.as_str()))
            .json(&request)
            .send()
            .await?;
        
        self.handle_response(response).await
    }
    
    /// Validate AF token
    pub async fn validate_token(&self, token_id: impl Into<String>) -> Result<ValidateResponse> {
        let url = format!("{}/v1/tokens/{}/validate", self.base_url, token_id.into());
        
        let response = self.http_client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.license_key.as_str()))
            .send()
            .await?;
        
        self.handle_response(response).await
    }
    
    /// Retrieves shadow decisions showing breach prevention activity
    /// 
    /// Shadow mode runs your IAM decisions through the Authority Fabric policy
    /// engine in parallel, identifying cases where IAM would have allowed access
    /// but AF would have denied it (indicating a potential security breach).
    /// 
    /// This endpoint is available to all license tiers.
    /// 
    /// # Returns
    /// A list of shadow decisions, where each decision represents a divergence
    /// between IAM and AF policy evaluation. Decisions with `breach_prevented: true`
    /// indicate cases where AF would have prevented a security breach.
    /// 
    /// # Example
    /// ```rust,no_run
    /// # use truthlinked_sdk::Client;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::new("https://api.truthlinked.org", "tl_free_...")?;
    /// let decisions = client.get_shadow_decisions().await?;
    /// 
    /// let breaches_prevented = decisions.iter()
    ///     .filter(|d| d.breach_prevented)
    ///     .count();
    /// 
    /// println!("Breaches prevented: {}", breaches_prevented);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_shadow_decisions(&self) -> Result<Vec<ShadowDecision>> {
        let url = format!("{}/v1/shadow/decisions", self.base_url);
        
        let response = self.http_client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.license_key.as_str()))
            .send()
            .await?;
        
        self.handle_response(response).await
    }
    
    /// Replay IAM logs through AF policy engine
    pub async fn replay_iam_logs(
        &self,
        logs: Vec<String>,
        adapter: impl Into<String>,
    ) -> Result<ReplayResponse> {
        let url = format!("{}/v1/shadow/replay", self.base_url);
        
        let request = ReplayRequest {
            logs,
            adapter: adapter.into(),
        };
        
        let response = self.http_client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.license_key.as_str()))
            .json(&request)
            .send()
            .await?;
        
        self.handle_response(response).await
    }
    
    /// Get SOX compliance report
    pub async fn get_sox_report(&self) -> Result<SoxReport> {
        let url = format!("{}/v1/compliance/sox", self.base_url);
        
        let response = self.http_client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.license_key.as_str()))
            .send()
            .await?;
        
        self.handle_response(response).await
    }
    
    /// Get PCI-DSS compliance report
    pub async fn get_pci_report(&self) -> Result<PciReport> {
        let url = format!("{}/v1/compliance/pci", self.base_url);
        
        let response = self.http_client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.license_key.as_str()))
            .send()
            .await?;
        
        self.handle_response(response).await
    }
    
    /// Get audit logs
    pub async fn get_audit_logs(&self) -> Result<Vec<AuditLog>> {
        let url = format!("{}/v1/audit/logs", self.base_url);
        
        let response = self.http_client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.license_key.as_str()))
            .send()
            .await?;
        
        self.handle_response(response).await
    }
    
    /// Get usage statistics
    pub async fn get_usage(&self) -> Result<UsageResponse> {
        let url = format!("{}/v1/usage", self.base_url);
        
        let response = self.http_client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.license_key.as_str()))
            .send()
            .await?;
        
        self.handle_response(response).await
    }
    
    /// Handle HTTP response with proper error mapping
    async fn handle_response<T: serde::de::DeserializeOwned>(
        &self,
        response: reqwest::Response,
    ) -> Result<T> {
        let status = response.status();
        
        match status {
            StatusCode::OK => {
                response.json::<T>().await.map_err(|_| TruthlinkedError::InvalidResponse)
            }
            StatusCode::UNAUTHORIZED => Err(TruthlinkedError::Unauthorized),
            StatusCode::FORBIDDEN => Err(TruthlinkedError::Forbidden),
            StatusCode::TOO_MANY_REQUESTS => {
                let body = response.text().await.unwrap_or_default();
                Err(TruthlinkedError::RateLimitExceeded(body))
            }
            StatusCode::BAD_REQUEST | StatusCode::UNPROCESSABLE_ENTITY => {
                let body = response.text().await.unwrap_or_default();
                Err(TruthlinkedError::InvalidRequest(body))
            }
            _ if status.is_server_error() => Err(TruthlinkedError::ServerError),
            _ => Err(TruthlinkedError::InvalidResponse),
        }
    }
}

impl std::fmt::Debug for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Client")
            .field("base_url", &self.base_url)
            .field("license_key", &self.license_key.redacted())
            .finish()
    }
}

impl Client {
    /// Handle HTTP error status codes
    fn handle_error_status<T>(&self, status: StatusCode) -> Result<T> {
        match status {
            StatusCode::UNAUTHORIZED => Err(TruthlinkedError::Unauthorized),
            StatusCode::FORBIDDEN => Err(TruthlinkedError::Forbidden),
            StatusCode::TOO_MANY_REQUESTS => {
                Err(TruthlinkedError::RateLimitExceeded("Rate limit exceeded".to_string()))
            }
            StatusCode::BAD_REQUEST | StatusCode::UNPROCESSABLE_ENTITY => {
                Err(TruthlinkedError::InvalidRequest("Invalid request".to_string()))
            }
            _ if status.is_server_error() => Err(TruthlinkedError::ServerError),
            _ => Err(TruthlinkedError::InvalidResponse),
        }
    }
}
