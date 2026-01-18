export declare class RequestSigner {
    private signingKey;
    constructor(licenseKey: string);
    sign(method: string, path: string, body?: any): {
        timestamp: string;
        signature: string;
    };
    verify(method: string, path: string, body: string, signature: string, timestamp: string): boolean;
    destroy(): void;
}
export declare class SecureLicenseKey {
    private key;
    constructor(key: string);
    asString(): string;
    redacted(): string;
    destroy(): void;
    toString(): string;
}
export declare function generateNonce(): string;
export declare function sha256(data: string | Buffer): string;
export declare function generateKeyPair(): {
    publicKey: string;
    secretKey: string;
};
export declare function signData(data: string, secretKey: string): string;
export declare function verifySignature(data: string, signature: string, publicKey: string): boolean;
export declare function constantTimeEqual(a: string, b: string): boolean;
