export interface TruthlinkedConfig {
    baseUrl: string;
    licenseKey: string;
    timeout?: number;
    signRequests?: boolean;
}
export interface TokenRequest {
    subject: string;
    permissions: string[];
    ttl?: number;
    metadata?: Record<string, any>;
}
export interface Token {
    id: string;
    token: string;
    subject: string;
    permissions: string[];
    expiresAt: number;
    createdAt: number;
}
export interface ValidationResult {
    valid: boolean;
    token?: Token;
    error?: string;
    validatedAt: number;
}
export interface WitnessSubmission {
    afEventHash: string;
    afMerkleRoot: string;
    afSequence: number;
    afInstanceId: string;
    oracleTime: number;
    afSignature: string;
}
export interface WitnessEvent {
    sequence: number;
    timestamp: number;
    submission: WitnessSubmission;
    prevHash: string;
    eventHash: string;
    proof?: string;
}
export interface SignedTreeHead {
    treeSize: number;
    timestamp: number;
    rootHash: string;
    signature: string;
    keyVersion: number;
}
export interface HealthStatus {
    status: string;
    version: string;
}
export declare class TruthlinkedError extends Error {
    statusCode?: number | undefined;
    response?: any | undefined;
    constructor(message: string, statusCode?: number | undefined, response?: any | undefined);
}
