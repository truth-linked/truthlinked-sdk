# SDK Comparison: Rust vs TypeScript

## Feature Parity Matrix

| Feature | Rust SDK | TypeScript SDK | Status |
|---------|----------|----------------|--------|
| **Core Client** | ✅ | ✅ | Complete |
| Token Request | ✅ | ✅ | Complete |
| Token Validation | ✅ | ✅ | Complete |
| Authorization | ✅ | ✅ | Complete |
| Request Signing | ✅ | ✅ | Complete |
| **Witness Chain** | ✅ | ✅ | Complete |
| Submit Event | ✅ | ✅ | Complete |
| Get Event | ✅ | ✅ | Complete |
| Get STH | ✅ | ✅ | Complete |
| Export Chain | ✅ | ✅ | Complete |
| **Crypto** | ✅ | ✅ | Complete |
| HMAC Signing | ✅ | ✅ | Complete |
| Nonce Generation | ✅ | ✅ | Complete |
| SHA-256 Hashing | ✅ | ✅ | Complete |
| Ed25519 Signing | ✅ | ✅ | Complete |
| **Error Handling** | ✅ | ✅ | Complete |
| **Type Safety** | ✅ | ✅ | Complete |
| **Examples** | ✅ | ✅ | Complete |
| **Documentation** | ✅ | ✅ | Complete |

## API Comparison

### Rust SDK
```rust
use truthlinked_sdk::{TruthlinkedClient, TokenRequest};

let client = TruthlinkedClient::new(
    "https://api.truthlinked.org",
    "license-key"
)?;

let token = client.request_token(TokenRequest {
    subject: "user@example.com".to_string(),
    permissions: vec!["read:data".to_string()],
    ttl: Some(3600),
    metadata: None,
}).await?;
```

### TypeScript SDK
```typescript
import { TruthlinkedClient } from '@truthlinked/sdk';

const client = new TruthlinkedClient({
  baseUrl: 'https://api.truthlinked.org',
  licenseKey: 'license-key'
});

const token = await client.requestToken({
  subject: 'user@example.com',
  permissions: ['read:data'],
  ttl: 3600
});
```

## Key Differences

### Type System
- **Rust**: Compile-time guarantees, zero-cost abstractions
- **TypeScript**: Runtime type checking, flexible typing

### Error Handling
- **Rust**: Result<T, E> with explicit error handling
- **TypeScript**: try/catch with TruthlinkedError class

### Async Model
- **Rust**: Tokio async runtime, futures
- **TypeScript**: Native async/await, Promises

### Performance
- **Rust**: Native performance, zero overhead
- **TypeScript**: V8 JIT compilation, excellent for I/O

### Deployment
- **Rust**: Single binary, no runtime dependencies
- **TypeScript**: Node.js runtime required

## Use Cases

### Choose Rust SDK When:
- Building high-performance services
- Need minimal resource footprint
- Deploying to embedded systems
- Require compile-time safety guarantees
- Building CLI tools

### Choose TypeScript SDK When:
- Building web applications
- Using Node.js/Express ecosystem
- Need rapid development
- Integrating with JavaScript frontends
- Building serverless functions

## Installation

### Rust
```bash
cargo add truthlinked-sdk
```

### TypeScript
```bash
npm install @truthlinked/sdk
```

## Both SDKs Provide:
- ✅ Full API coverage
- ✅ Post-quantum security
- ✅ Request signing
- ✅ Witness chain support
- ✅ Comprehensive documentation
- ✅ Production-ready code
- ✅ MIT license
- ✅ Active maintenance

## Conclusion

Both SDKs are **production-ready** and provide **complete feature parity**. Choose based on your technology stack and performance requirements.
