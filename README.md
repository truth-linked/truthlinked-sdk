# Truthlinked SDK (Rust)

Official Rust SDK for **Truthlinked Authority Fabric** - Zero-Trust Authorization System

[![Crates.io](https://img.shields.io/crates/v/truthlinked-sdk.svg)](https://crates.io/crates/truthlinked-sdk)
[![Documentation](https://docs.rs/truthlinked-sdk/badge.svg)](https://docs.rs/truthlinked-sdk)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

## Features

- ✅ **Type-safe API** - Compile-time guarantees, no runtime surprises
- ✅ **Secure by default** - HTTPS-only, TLS validation, memory protection
- ✅ **Production-ready** - Connection pooling, timeouts, error handling
- ✅ **Zero server coupling** - Standalone SDK, no dependencies on server code
- ✅ **Memory safe** - License keys automatically zeroized
- ✅ **No credential leakage** - Safe error messages, redacted logging

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
truthlinked-sdk = "0.1"
tokio = { version = "1.0", features = ["full"] }
```

## Quick Start

```rust
use truthlinked_sdk::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client
    let client = Client::new(
        "https://api.truthlinked.org",
        "tl_free_..."  // Your license key
    )?;
    
    // Check health
    let health = client.health().await?;
    println!("Status: {}", health.status);
    
    // Get shadow decisions (breach detections)
    let decisions = client.get_shadow_decisions().await?;
    let breaches = decisions.iter()
        .filter(|d| d.breach_prevented)
        .count();
    println!("Breaches prevented: {}", breaches);
    
    // Get compliance report
    let sox = client.get_sox_report().await?;
    println!("SOX: {} events", sox.total_events);
    
    Ok(())
}
```

## Examples

Run examples with your license key:

```bash
export TRUTHLINKED_LICENSE_KEY="tl_free_..."

# Health check
cargo run --example health_check

# Shadow mode (breach detection)
cargo run --example shadow_report

# Compliance reports
cargo run --example compliance_report

# Usage statistics
cargo run --example usage_stats
```

## API Reference

### Client Creation

```rust
let client = Client::new(base_url, license_key)?;
```

**Security:**
- Enforces HTTPS (rejects HTTP)
- Validates TLS certificates
- 30-second timeout
- Connection pooling

### Health Check

```rust
let health = client.health().await?;
```

No authentication required.

### Shadow Mode (Breach Detection)

```rust
// Get breach detections
let decisions = client.get_shadow_decisions().await?;

// Replay IAM logs
let result = client.replay_iam_logs(logs, "aws-cloudtrail").await?;
```

**Supported adapters:**
- `aws-cloudtrail` - AWS CloudTrail logs
- `azure-ad` - Azure AD logs
- `gcp-audit` - GCP Audit logs
- `okta` - Okta System Log
- `auth0` - Auth0 logs

### Token Exchange

```rust
use rand::Rng;

// Generate nonce and channel binding
let nonce: [u8; 32] = rand::thread_rng().gen();
let channel_binding: [u8; 32] = rand::thread_rng().gen();

// Exchange SSO token for AF token
let response = client.exchange_token(
    sso_token,
    vec!["read:users".to_string()],
    nonce,
    channel_binding,
).await?;

println!("AF Token: {}", response.af_token);
```

**Requires:** Professional tier or higher

### Token Validation

```rust
let result = client.validate_token(token_id).await?;

if result.valid {
    println!("Subject: {:?}", result.subject);
    println!("Scope: {:?}", result.scope);
}
```

### Compliance Reports

```rust
// SOX compliance
let sox = client.get_sox_report().await?;
println!("Period: {}", sox.period);
println!("Events: {}", sox.total_events);
println!("Complete: {}", sox.audit_trail_complete);

// PCI-DSS compliance
let pci = client.get_pci_report().await?;
println!("Access controls: {}", pci.access_controls_enforced);
println!("Encryption: {}", pci.encryption_verified);
```

### Audit Logs

```rust
let logs = client.get_audit_logs().await?;

for log in logs {
    println!("{}: {} by {}", log.timestamp, log.action, log.subject);
}
```

### Usage Statistics

```rust
let usage = client.get_usage().await?;

println!("Tier: {}", usage.tier);
println!("Usage: {} / {}", usage.usage, usage.limit);
println!("Days remaining: {}", usage.days_remaining);
```

## Error Handling

```rust
use truthlinked_sdk::TruthlinkedError;

match client.get_shadow_decisions().await {
    Ok(decisions) => println!("Got {} decisions", decisions.len()),
    Err(TruthlinkedError::Unauthorized) => {
        eprintln!("Invalid license key");
    }
    Err(TruthlinkedError::Forbidden) => {
        eprintln!("Tier doesn't allow this operation");
    }
    Err(TruthlinkedError::RateLimitExceeded(msg)) => {
        eprintln!("Rate limit: {}", msg);
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

## Security

### Threat Mitigations

✅ **T1: Credential Leakage**
- License keys zeroized from memory
- Redacted in logs and errors
- Never serialized in full

✅ **T2: Man-in-the-Middle**
- HTTPS enforced (no HTTP fallback)
- TLS certificate validation
- Uses rustls (no OpenSSL)

✅ **T3: Replay Attacks**
- Nonce required for token exchange
- Timestamp validation server-side

✅ **T4: Dependency Vulnerabilities**
- Minimal dependencies (6 total)
- All dependencies audited
- No transitive vulnerabilities

✅ **T5: Memory Safety**
- Rust memory safety guarantees
- Zeroize for sensitive data
- No use-after-free possible

✅ **T6: Information Disclosure**
- Safe error messages
- No internal details leaked
- No stack traces in production

### Best Practices

```rust
// ✅ DO: Store license key in environment
let key = std::env::var("TRUTHLINKED_LICENSE_KEY")?;

// ❌ DON'T: Hardcode license key
let key = "tl_free_...";  // Never do this!

// ✅ DO: Use HTTPS
Client::new("https://api.truthlinked.org", key)?;

// ❌ DON'T: Use HTTP
Client::new("http://api.truthlinked.org", key)?;  // Rejected!

// ✅ DO: Handle errors properly
match client.health().await {
    Ok(health) => println!("OK: {}", health.status),
    Err(e) => eprintln!("Error: {}", e),
}

// ❌ DON'T: Unwrap in production
let health = client.health().await.unwrap();  // Can panic!
```

## License Tiers

| Tier | Price | Features |
|------|-------|----------|
| **Free** | $0/mo | Shadow mode, compliance reports, 1k requests/mo, **5 Pro features/day**, **3 Enterprise features/day** |
| **Professional** | $2,500/mo | + Token exchange, 500k requests/mo, **2 Enterprise features/day** |
| **Enterprise** | $25,000/mo | + Full enforcement, unlimited requests |
| **Government** | $100,000/mo | + Air-gapped deployment, unlimited retention |

## Support

- **Documentation**: https://docs.truthlinked.org
- **API Reference**: https://docs.rs/truthlinked-sdk
- **Issues**: https://github.com/truthlinked/sdk/issues
- **Email**: support@truthlinked.org

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Contributing

Contributions welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) first.

---

**Built with ❤️ by Truthlinked**
