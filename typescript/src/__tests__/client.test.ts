import { TruthlinkedClient } from '../client';
import { TruthlinkedError } from '../types';

describe('TruthlinkedClient', () => {
  describe('constructor', () => {
    it('enforces HTTPS', () => {
      expect(() => {
        new TruthlinkedClient({
          baseUrl: 'http://insecure.com',
          licenseKey: 'test_key'
        });
      }).toThrow('Base URL must use HTTPS');
    });

    it('accepts HTTPS URLs', () => {
      expect(() => {
        new TruthlinkedClient({
          baseUrl: 'https://secure.com',
          licenseKey: 'test_key'
        });
      }).not.toThrow();
    });

    it('sets default timeout', () => {
      const client = new TruthlinkedClient({
        baseUrl: 'https://api.test.com',
        licenseKey: 'test_key'
      });
      
      expect(client['config'].timeout).toBe(30000);
    });

    it('enables request signing by default', () => {
      const client = new TruthlinkedClient({
        baseUrl: 'https://api.test.com',
        licenseKey: 'test_key'
      });
      
      expect(client['config'].signRequests).toBe(true);
    });
  });

  describe('destroy', () => {
    it('cleans up sensitive data', () => {
      const client = new TruthlinkedClient({
        baseUrl: 'https://api.test.com',
        licenseKey: 'test_key'
      });
      
      expect(() => client.destroy()).not.toThrow();
    });
  });
});
