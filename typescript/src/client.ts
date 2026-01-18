import axios, { AxiosInstance, AxiosError } from 'axios';
import {
  TruthlinkedConfig,
  TokenRequest,
  Token,
  ValidationResult,
  WitnessSubmission,
  WitnessEvent,
  SignedTreeHead,
  HealthStatus,
  TruthlinkedError
} from './types';
import { RequestSigner, SecureLicenseKey } from './crypto';

export class TruthlinkedClient {
  private client: AxiosInstance;
  private signer?: RequestSigner;
  private licenseKey: SecureLicenseKey;
  private config: TruthlinkedConfig;

  constructor(config: TruthlinkedConfig) {
    if (!config.baseUrl.startsWith('https://')) {
      throw new TruthlinkedError('Base URL must use HTTPS', 400);
    }

    this.config = {
      timeout: 30000,
      signRequests: true,
      ...config
    };

    this.licenseKey = new SecureLicenseKey(this.config.licenseKey);

    this.client = axios.create({
      baseURL: this.config.baseUrl,
      timeout: this.config.timeout,
      headers: {
        'Content-Type': 'application/json',
        'User-Agent': 'Truthlinked-SDK-TS/1.0.0'
      },
      httpsAgent: undefined,
      maxRedirects: 5,
      validateStatus: (status) => status < 500
    });

    if (this.config.signRequests) {
      this.signer = new RequestSigner(this.config.licenseKey);
    }

    this.client.interceptors.request.use((config) => {
      if (this.signer && config.method && config.url) {
        const { timestamp, signature } = this.signer.sign(
          config.method.toUpperCase(),
          config.url,
          config.data
        );
        config.headers['X-Timestamp'] = timestamp;
        config.headers['X-Signature'] = signature;
      }
      config.headers['X-License-Key'] = this.licenseKey.asString();
      return config;
    });

    this.client.interceptors.response.use(
      (response) => response,
      (error: AxiosError) => {
        if (error.response) {
          const data = error.response.data as any;
          throw new TruthlinkedError(
            data?.error || error.message,
            error.response.status,
            data
          );
        }
        throw new TruthlinkedError(error.message);
      }
    );
  }

  async health(): Promise<HealthStatus> {
    const response = await this.client.get<HealthStatus>('/health');
    return response.data;
  }

  async requestToken(request: TokenRequest): Promise<Token> {
    const response = await this.client.post<Token>('/v1/token', request);
    return response.data;
  }

  async validateToken(token: string): Promise<ValidationResult> {
    const response = await this.client.post<ValidationResult>('/v1/validate', { token });
    return response.data;
  }

  async authorize(token: string, permission: string): Promise<boolean> {
    try {
      const response = await this.client.post('/v1/authorize', {
        token,
        permission
      });
      return response.data.authorized === true;
    } catch (error) {
      if (error instanceof TruthlinkedError && error.statusCode === 403) {
        return false;
      }
      throw error;
    }
  }

  async submitWitness(submission: WitnessSubmission): Promise<WitnessEvent> {
    const response = await this.client.post<WitnessEvent>(
      '/witness/submit',
      { submission }
    );
    return response.data;
  }

  async getWitnessEvent(sequence: number, includeProof = false): Promise<WitnessEvent> {
    const response = await this.client.get<WitnessEvent>(
      `/witness/event/${sequence}`,
      { params: { include_proof: includeProof } }
    );
    return response.data;
  }

  async getLatestSTH(): Promise<SignedTreeHead> {
    const response = await this.client.get<SignedTreeHead>('/witness/sth/latest');
    return response.data;
  }

  async getSTH(treeSize: number): Promise<SignedTreeHead> {
    const response = await this.client.get<SignedTreeHead>(`/witness/sth/${treeSize}`);
    return response.data;
  }

  async exportWitnessChain(startSeq?: number, endSeq?: number): Promise<Blob> {
    const params: any = {};
    if (startSeq !== undefined) params.start_seq = startSeq;
    if (endSeq !== undefined) params.end_seq = endSeq;

    const response = await this.client.get('/witness/export', {
      params,
      responseType: 'blob'
    });
    return response.data;
  }

  async witnessHealth(): Promise<{ status: string; chainSize: number }> {
    const response = await this.client.get('/witness/health');
    return response.data;
  }

  destroy(): void {
    this.licenseKey.destroy();
    if (this.signer) {
      this.signer.destroy();
    }
  }
}
