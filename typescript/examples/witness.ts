/**
 * Witness chain example
 */
import { TruthlinkedClient, sha256 } from '../src';

async function main() {
  const client = new TruthlinkedClient({
    baseUrl: process.env.TRUTHLINKED_BASE_URL || 'http://localhost:3000',
    licenseKey: process.env.TRUTHLINKED_LICENSE_KEY || '0'.repeat(64)
  });

  try {
    // Submit witness event
    console.log('Submitting witness event...');
    const event = await client.submitWitness({
      afEventHash: sha256('event-data'),
      afMerkleRoot: sha256('merkle-root'),
      afSequence: 1,
      afInstanceId: sha256('instance-id'),
      oracleTime: Date.now(),
      afSignature: '0'.repeat(128)
    });
    console.log('✓ Event submitted:', event.sequence);

    // Get event with proof
    console.log('\nGetting event with Merkle proof...');
    const witnessEvent = await client.getWitnessEvent(event.sequence, true);
    console.log('✓ Event retrieved with proof');

    // Get latest STH
    console.log('\nGetting latest STH...');
    const sth = await client.getLatestSTH();
    console.log('✓ Tree size:', sth.treeSize);
    console.log('✓ Root hash:', sth.rootHash);

    // Check witness health
    console.log('\nChecking witness health...');
    const health = await client.witnessHealth();
    console.log('✓ Witness status:', health.status);

  } catch (error) {
    console.error('Error:', error);
    process.exit(1);
  }
}

main();
