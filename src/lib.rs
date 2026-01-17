//! # Truthlinked SDK
//!
//! Official Rust SDK for **Truthlinked Authority Fabric** - Zero-Trust Authorization System
//!
//! ## Overview
//!
//! The Truthlinked Authority Fabric is a zero-trust authorization system that sits above
//! traditional IAM systems (AWS IAM, Azure AD, Okta, etc.) as a final enforcement layer.
//! It can override any IAM decision with cryptographic proof and provides breach detection
//! through shadow mode analysis.
//!
//! ## Features
//!
//! - **Type-safe API**: Compile-time guarantees, no runtime surprises
//! - **Secure by default**: HTTPS-only, TLS certificate validation, memory protection
//! - **Production-ready**: Connection pooling, automatic retries, timeout handling
//! - **Zero dependencies on server**: Standalone SDK, no coupling
//! - **Memory safety**: License keys automatically zeroized from memory
//! - **No credential leakage**: Safe error messages, redacted logging
//!
//! ## Security Architecture
//!
//! ### Threat Mitigations
//!
//! | Threat | Mitigation |
//! |--------|------------|
//! | **Credential Leakage** | License keys zeroized from memory, redacted in logs/errors |
//! | **Man-in-the-Middle** | HTTPS enforced, TLS certificate validation, rustls |
//! | **Replay Attacks** | Nonce support for token exchange, server-side validation |
//! | **Dependency Vulnerabilities** | Minimal dependencies (6 total), all audited |
//! | **Memory Safety** | Rust guarantees + zeroize for sensitive data |
//! | **Information Disclosure** | Safe error messages, no internal details leaked |
//!
//! ### Security Guarantees
//!
//! - **HTTPS Enforcement**: HTTP requests are rejected at client creation
//! - **Certificate Validation**: Self-signed certificates are rejected
//! - **Memory Protection**: Sensitive data is zeroized when no longer needed
//! - **Safe Error Handling**: Error messages never contain credentials or internal details
//! - **Constant-Time Operations**: Where applicable, operations are constant-time
//!
//! ## Quick Start
//!
//! Add to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! truthlinked-sdk = "0.1"
//! tokio = { version = "1.0", features = ["full"] }
//! ```
//!
//! Basic usage:
//! ```rust,no_run
//! use truthlinked_sdk::Client;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create client (enforces HTTPS)
//!     let client = Client::new(
//!         "https://api.truthlinked.org",
//!         std::env::var("TRUTHLINKED_LICENSE_KEY")?
//!     )?;
//!     
//!     // Check server health
//!     let health = client.health().await?;
//!     println!("Server status: {}", health.status);
//!     
//!     // Get shadow decisions (breach detections)
//!     let decisions = client.get_shadow_decisions().await?;
//!     let breaches = decisions.iter()
//!         .filter(|d| d.breach_prevented)
//!         .count();
//!     println!("Breaches prevented: {}", breaches);
//!     
//!     // Get compliance reports
//!     let sox = client.get_sox_report().await?;
//!     println!("SOX compliance: {} events", sox.total_events);
//!     
//!     Ok(())
//! }
//! ```
//!
//! ## License Tiers
//!
//! | Tier | Price | Features |
//! |------|-------|----------|
//! | **Free** | $0/mo | Shadow mode, compliance reports, 1k requests/mo |
//! | **Professional** | $2,500/mo | + Token exchange, 500k requests/mo |
//! | **Enterprise** | $25,000/mo | + Full enforcement, unlimited requests |
//! | **Government** | $100,000/mo | + Air-gapped deployment |
//!
//! ## Error Handling
//!
//! All errors implement `std::error::Error` and provide safe, actionable error messages:
//!
//! ```rust,no_run
//! # use truthlinked_sdk::{Client, TruthlinkedError};
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = Client::new("https://api.truthlinked.org", "key")?;
//! match client.get_shadow_decisions().await {
//!     Ok(decisions) => println!("Got {} decisions", decisions.len()),
//!     Err(TruthlinkedError::Unauthorized) => {
//!         eprintln!("Invalid license key - check TRUTHLINKED_LICENSE_KEY");
//!     }
//!     Err(TruthlinkedError::Forbidden) => {
//!         eprintln!("License tier doesn't support this operation");
//!     }
//!     Err(TruthlinkedError::RateLimitExceeded(msg)) => {
//!         eprintln!("Rate limit exceeded: {}", msg);
//!     }
//!     Err(e) => eprintln!("Error: {}", e),
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Best Practices
//!
//! ### Secure Configuration
//! ```rust,no_run
//! use truthlinked_sdk::Client;
//! 
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // ✅ DO: Store license key in environment variable
//! let key = std::env::var("TRUTHLINKED_LICENSE_KEY")?;
//! let client = Client::new("https://api.truthlinked.org", key)?;
//!
//! // ❌ DON'T: Hardcode license keys
//! let client = Client::new("https://api.truthlinked.org", "tl_free_...")?;
//! # Ok(())
//! # }
//! ```
//!
//! ### Error Handling
//! ```rust,no_run
//! # use truthlinked_sdk::Client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = Client::new("https://api.truthlinked.org", "key")?;
//! // ✅ DO: Handle errors gracefully
//! match client.health().await {
//!     Ok(health) => println!("Status: {}", health.status),
//!     Err(e) => eprintln!("Health check failed: {}", e),
//! }
//!
//! // ❌ DON'T: Use unwrap() in production
//! let health = client.health().await.unwrap();  // Can panic!
//! # Ok(())
//! # }
//! ```
//!
//! ### Thread Safety
//! ```rust,no_run
//! # use truthlinked_sdk::Client;
//! # use std::sync::Arc;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // ✅ DO: Share client across threads with Arc
//! let client = Arc::new(Client::new("https://api.truthlinked.org", "key")?);
//! let client_clone = client.clone();
//!
//! tokio::spawn(async move {
//!     let _ = client_clone.health().await;
//! });
//! # Ok(())
//! # }
//! ```
//!
//! ## Support
//!
//! - **Documentation**: <https://docs.truthlinked.org>
//! - **API Reference**: <https://docs.rs/truthlinked-sdk>
//! - **Issues**: <https://github.com/truthlinked/sdk/issues>
//! - **Email**: support@truthlinked.org

mod builder;
mod client;
mod error;
mod license;
mod logging;
mod retry;
mod signing;
mod types;

pub use builder::ClientBuilder;
pub use client::Client;
pub use error::{TruthlinkedError, Result};
pub use logging::{LoggingConfig, LogLevel};
pub use retry::RetryConfig;
pub use types::*;

// Re-export for convenience
pub use license::LicenseKey;

// Re-export specific items for testing
pub use signing::RequestSigner;
pub use retry::RetryExecutor;
pub use logging::RequestLogger;
