import { generateNonce, sha256, constantTimeEqual, SecureLicenseKey, RequestSigner } from '../crypto';

describe('Crypto', () => {
  describe('generateNonce', () => {
    it('generates 64 character hex string', () => {
      const nonce = generateNonce();
      expect(nonce).toHaveLength(64);
      expect(nonce).toMatch(/^[0-9a-f]+$/);
    });

    it('generates unique nonces', () => {
      const nonce1 = generateNonce();
      const nonce2 = generateNonce();
      expect(nonce1).not.toBe(nonce2);
    });
  });

  describe('sha256', () => {
    it('hashes string correctly', () => {
      const hash = sha256('test');
      expect(hash).toHaveLength(64);
      expect(hash).toBe('9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08');
    });

    it('produces consistent hashes', () => {
      const hash1 = sha256('test');
      const hash2 = sha256('test');
      expect(hash1).toBe(hash2);
    });
  });

  describe('constantTimeEqual', () => {
    it('returns true for equal strings', () => {
      expect(constantTimeEqual('test', 'test')).toBe(true);
    });

    it('returns false for different strings', () => {
      expect(constantTimeEqual('test', 'fail')).toBe(false);
    });

    it('returns false for different lengths', () => {
      expect(constantTimeEqual('test', 'testing')).toBe(false);
    });
  });

  describe('SecureLicenseKey', () => {
    it('redacts key correctly', () => {
      const key = new SecureLicenseKey('tl_free_secret123456789');
      expect(key.redacted()).toBe('tl_...789');
    });

    it('redacts short keys', () => {
      const key = new SecureLicenseKey('short');
      expect(key.redacted()).toBe('***');
    });

    it('zeroizes on destroy', () => {
      const key = new SecureLicenseKey('test_key');
      key.destroy();
      expect(key.asString()).toBe('\0'.repeat(8));
    });
  });

  describe('RequestSigner', () => {
    it('signs requests consistently', () => {
      const signer = new RequestSigner('test_key');
      const sig1 = signer.sign('GET', '/test', null);
      const sig2 = signer.sign('GET', '/test', null);
      
      expect(sig1.signature).toBe(sig2.signature);
    });

    it('produces different signatures for different methods', () => {
      const signer = new RequestSigner('test_key');
      const sig1 = signer.sign('GET', '/test', null);
      const sig2 = signer.sign('POST', '/test', null);
      
      expect(sig1.signature).not.toBe(sig2.signature);
    });

    it('verifies signatures correctly', () => {
      const signer = new RequestSigner('test_key');
      const { timestamp, signature } = signer.sign('GET', '/test', null);
      
      expect(signer.verify('GET', '/test', '', signature, timestamp)).toBe(true);
    });
  });
});
