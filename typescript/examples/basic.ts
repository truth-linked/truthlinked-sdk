/**
 * Basic usage example
 */
import { TruthlinkedClient } from '../src';

async function main() {
  // Initialize client
  const client = new TruthlinkedClient({
    baseUrl: process.env.TRUTHLINKED_BASE_URL || 'http://localhost:3000',
    licenseKey: process.env.TRUTHLINKED_LICENSE_KEY || '0'.repeat(64),
    signRequests: true
  });

  try {
    // Check health
    console.log('Checking server health...');
    const health = await client.health();
    console.log('✓ Server status:', health.status);

    // Request token
    console.log('\nRequesting token...');
    const token = await client.requestToken({
      subject: 'user@example.com',
      permissions: ['read:data', 'write:data'],
      ttl: 3600
    });
    console.log('✓ Token created:', token.id);

    // Validate token
    console.log('\nValidating token...');
    const validation = await client.validateToken(token.token);
    console.log('✓ Token valid:', validation.valid);

    // Authorize action
    console.log('\nAuthorizing action...');
    const authorized = await client.authorize(token.token, 'read:data');
    console.log('✓ Authorized:', authorized);

  } catch (error) {
    console.error('Error:', error);
    process.exit(1);
  }
}

main();
