import * as crypto from 'crypto';
import * as nacl from 'tweetnacl';
import * as util from 'tweetnacl-util';

export class RequestSigner {
  private signingKey: Buffer;

  constructor(licenseKey: string) {
    const hmac = crypto.createHmac('sha256', 'truthlinked-request-signing-v1');
    hmac.update(licenseKey);
    this.signingKey = hmac.digest();
  }

  sign(method: string, path: string, body?: any): { timestamp: string; signature: string } {
    const timestamp = Math.floor(Date.now() / 1000).toString();
    const bodyStr = body ? JSON.stringify(body) : '';
    const message = `${method}\n${path}\n${timestamp}\n${bodyStr}`;
    
    const hmac = crypto.createHmac('sha256', this.signingKey);
    hmac.update(message);
    const signature = hmac.digest('base64');

    return { timestamp, signature };
  }

  verify(method: string, path: string, body: string, signature: string, timestamp: string): boolean {
    const message = `${method}\n${path}\n${timestamp}\n${body}`;
    const hmac = crypto.createHmac('sha256', this.signingKey);
    hmac.update(message);
    const expected = hmac.digest('base64');
    
    return crypto.timingSafeEqual(
      Buffer.from(signature, 'base64'),
      Buffer.from(expected, 'base64')
    );
  }

  destroy(): void {
    this.signingKey.fill(0);
  }
}

export class SecureLicenseKey {
  private key: string;

  constructor(key: string) {
    this.key = key;
  }

  asString(): string {
    return this.key;
  }

  redacted(): string {
    const len = this.key.length;
    if (len > 8) {
      return `${this.key.slice(0, 3)}...${this.key.slice(-3)}`;
    }
    return '***';
  }

  destroy(): void {
    this.key = '\0'.repeat(this.key.length);
  }

  toString(): string {
    return this.redacted();
  }
}

export function generateNonce(): string {
  return crypto.randomBytes(32).toString('hex');
}

export function sha256(data: string | Buffer): string {
  return crypto.createHash('sha256').update(data).digest('hex');
}

export function generateKeyPair(): { publicKey: string; secretKey: string } {
  const keypair = nacl.sign.keyPair();
  return {
    publicKey: util.encodeBase64(keypair.publicKey),
    secretKey: util.encodeBase64(keypair.secretKey)
  };
}

export function signData(data: string, secretKey: string): string {
  const message = Buffer.from(data, 'utf8');
  const key = util.decodeBase64(secretKey);
  const signature = nacl.sign.detached(message, key);
  return util.encodeBase64(signature);
}

export function verifySignature(data: string, signature: string, publicKey: string): boolean {
  const message = Buffer.from(data, 'utf8');
  const sig = util.decodeBase64(signature);
  const key = util.decodeBase64(publicKey);
  return nacl.sign.detached.verify(message, sig, key);
}

export function constantTimeEqual(a: string, b: string): boolean {
  if (a.length !== b.length) {
    return false;
  }
  return crypto.timingSafeEqual(Buffer.from(a), Buffer.from(b));
}


