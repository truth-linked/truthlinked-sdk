# @truthlinked/sdk

Official TypeScript/Node.js SDK for **Truthlinked Authority Fabric** - Post-quantum secure identity and authorization system.

## Features

- 🔐 **Post-Quantum Security** - Dilithium3 signatures, lattice-based cryptography
- 🎯 **Zero-Trust Architecture** - Every request cryptographically verified
- 📝 **Witness Chain** - Immutable audit log with Merkle proofs
- ⚡ **High Performance** - Optimized for production workloads
- 🛡️ **Type-Safe** - Full TypeScript support with comprehensive types
- 🔄 **Request Signing** - HMAC-SHA256 authentication
- 📦 **Lightweight** - Minimal dependencies

## Installation

```bash
npm install @truthlinked/sdk
```

## Quick Start

```typescript
import { TruthlinkedClient } from '@truthlinked/sdk';

// Initialize client
const client = new TruthlinkedClient({
  baseUrl: 'https://api.truthlinked.org',
  licenseKey: 'your-license-key-hex',
  signRequests: true
});

// Check server health
const health = await client.health();
console.log('Server status:', health.status);

// Request a token
const token = await client.requestToken({
  subject: 'user@example.com',
  permissions: ['read:data', 'write:data'],
  ttl: 3600
});

// Validate token
const validation = await client.validateToken(token.token);
if (validation.valid) {
  console.log('Token is valid!');
}

// Authorize action
const authorized = await client.authorize(token.token, 'read:data');
if (authorized) {
  console.log('Action authorized!');
}
```

## Witness Chain

Submit events to the immutable witness chain:

```typescript
// Submit witness event
const event = await client.submitWitness({
  afEventHash: '0x...',
  afMerkleRoot: '0x...',
  afSequence: 12345,
  afInstanceId: '0x...',
  oracleTime: Date.now(),
  afSignature: '0x...'
});

// Get event with Merkle proof
const witnessEvent = await client.getWitnessEvent(event.sequence, true);

// Get latest Signed Tree Head
const sth = await client.getLatestSTH();
console.log('Tree size:', sth.treeSize);

// Export chain for offline verification
const exportData = await client.exportWitnessChain(0, 1000);
```

## API Reference

### TruthlinkedClient

#### Constructor

```typescript
new TruthlinkedClient(config: TruthlinkedConfig)
```

**Config Options:**
- `baseUrl` (string, required) - Truthlinked server URL
- `licenseKey` (string, required) - Your license key (hex format)
- `timeout` (number, optional) - Request timeout in ms (default: 30000)
- `signRequests` (boolean, optional) - Enable request signing (default: true)

#### Methods

##### Token Management

```typescript
// Request new token
requestToken(request: TokenRequest): Promise<Token>

// Validate token
validateToken(token: string): Promise<ValidationResult>

// Authorize action
authorize(token: string, permission: string): Promise<boolean>
```

##### Witness Chain

```typescript
// Submit witness event
submitWitness(submission: WitnessSubmission): Promise<WitnessEvent>

// Get event by sequence
getWitnessEvent(sequence: number, includeProof?: boolean): Promise<WitnessEvent>

// Get latest STH
getLatestSTH(): Promise<SignedTreeHead>

// Get STH at specific size
getSTH(treeSize: number): Promise<SignedTreeHead>

// Export chain
exportWitnessChain(startSeq?: number, endSeq?: number): Promise<Blob>

// Check witness health
witnessHealth(): Promise<{ status: string; chainSize: number }>
```

##### Health

```typescript
// Check server health
health(): Promise<HealthStatus>
```

### Crypto Utilities

```typescript
import { generateNonce, sha256, generateKeyPair, signData, verifySignature } from '@truthlinked/sdk';

// Generate secure nonce
const nonce = generateNonce();

// Hash data
const hash = sha256('data');

// Generate Ed25519 keypair
const { publicKey, secretKey } = generateKeyPair();

// Sign data
const signature = signData('message', secretKey);

// Verify signature
const valid = verifySignature('message', signature, publicKey);
```

## Error Handling

```typescript
import { TruthlinkedError } from '@truthlinked/sdk';

try {
  await client.validateToken('invalid-token');
} catch (error) {
  if (error instanceof TruthlinkedError) {
    console.error('Status:', error.statusCode);
    console.error('Message:', error.message);
    console.error('Response:', error.response);
  }
}
```

## TypeScript Support

Full TypeScript definitions included:

```typescript
import type {
  TruthlinkedConfig,
  TokenRequest,
  Token,
  ValidationResult,
  WitnessSubmission,
  WitnessEvent,
  SignedTreeHead
} from '@truthlinked/sdk';
```

## Security Best Practices

1. **Never expose license keys** - Use environment variables
2. **Enable request signing** - Always set `signRequests: true`
3. **Validate responses** - Check signatures on critical operations
4. **Use HTTPS** - Always connect over TLS
5. **Rotate keys** - Implement key rotation policies
6. **Monitor witness chain** - Verify STH consistency

## Environment Variables

```bash
# Recommended setup
TRUTHLINKED_BASE_URL=https://api.truthlinked.org
TRUTHLINKED_LICENSE_KEY=your-hex-key
TRUTHLINKED_TIMEOUT=30000
```

```typescript
const client = new TruthlinkedClient({
  baseUrl: process.env.TRUTHLINKED_BASE_URL!,
  licenseKey: process.env.TRUTHLINKED_LICENSE_KEY!,
  timeout: parseInt(process.env.TRUTHLINKED_TIMEOUT || '30000')
});
```

## Examples

### Express.js Middleware

```typescript
import express from 'express';
import { TruthlinkedClient } from '@truthlinked/sdk';

const client = new TruthlinkedClient({
  baseUrl: process.env.TRUTHLINKED_BASE_URL!,
  licenseKey: process.env.TRUTHLINKED_LICENSE_KEY!
});

const authMiddleware = async (req, res, next) => {
  const token = req.headers.authorization?.replace('Bearer ', '');
  
  if (!token) {
    return res.status(401).json({ error: 'No token provided' });
  }

  try {
    const validation = await client.validateToken(token);
    if (!validation.valid) {
      return res.status(401).json({ error: 'Invalid token' });
    }
    req.user = validation.token;
    next();
  } catch (error) {
    res.status(500).json({ error: 'Authentication failed' });
  }
};

app.use(authMiddleware);
```

### Permission Check

```typescript
const requirePermission = (permission: string) => {
  return async (req, res, next) => {
    const token = req.headers.authorization?.replace('Bearer ', '');
    
    try {
      const authorized = await client.authorize(token!, permission);
      if (!authorized) {
        return res.status(403).json({ error: 'Insufficient permissions' });
      }
      next();
    } catch (error) {
      res.status(500).json({ error: 'Authorization failed' });
    }
  };
};

app.get('/admin', requirePermission('admin:access'), (req, res) => {
  res.json({ message: 'Admin access granted' });
});
```

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Support

- Documentation: https://docs.truthlinked.org
- Issues: https://github.com/truth-linked/truthlinked-sdk/issues
- Email: support@truthlinked.org

## Related

- [Rust SDK](https://crates.io/crates/truthlinked-sdk)
- [Python SDK](https://pypi.org/project/truthlinked-sdk)
- [Go SDK](https://github.com/truth-linked/truthlinked-sdk)
