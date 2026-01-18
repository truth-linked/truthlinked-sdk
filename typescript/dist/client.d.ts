import { TruthlinkedConfig, TokenRequest, Token, ValidationResult, WitnessSubmission, WitnessEvent, SignedTreeHead, HealthStatus } from './types';
export declare class TruthlinkedClient {
    private client;
    private signer?;
    private licenseKey;
    private config;
    constructor(config: TruthlinkedConfig);
    health(): Promise<HealthStatus>;
    requestToken(request: TokenRequest): Promise<Token>;
    validateToken(token: string): Promise<ValidationResult>;
    authorize(token: string, permission: string): Promise<boolean>;
    submitWitness(submission: WitnessSubmission): Promise<WitnessEvent>;
    getWitnessEvent(sequence: number, includeProof?: boolean): Promise<WitnessEvent>;
    getLatestSTH(): Promise<SignedTreeHead>;
    getSTH(treeSize: number): Promise<SignedTreeHead>;
    exportWitnessChain(startSeq?: number, endSeq?: number): Promise<Blob>;
    witnessHealth(): Promise<{
        status: string;
        chainSize: number;
    }>;
    destroy(): void;
}
