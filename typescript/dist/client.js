"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.TruthlinkedClient = void 0;
const axios_1 = __importDefault(require("axios"));
const types_1 = require("./types");
const crypto_1 = require("./crypto");
class TruthlinkedClient {
    constructor(config) {
        if (!config.baseUrl.startsWith('https://')) {
            throw new types_1.TruthlinkedError('Base URL must use HTTPS', 400);
        }
        this.config = {
            timeout: 30000,
            signRequests: true,
            ...config
        };
        this.licenseKey = new crypto_1.SecureLicenseKey(this.config.licenseKey);
        this.client = axios_1.default.create({
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
            this.signer = new crypto_1.RequestSigner(this.config.licenseKey);
        }
        this.client.interceptors.request.use((config) => {
            if (this.signer && config.method && config.url) {
                const { timestamp, signature } = this.signer.sign(config.method.toUpperCase(), config.url, config.data);
                config.headers['X-Timestamp'] = timestamp;
                config.headers['X-Signature'] = signature;
            }
            config.headers['X-License-Key'] = this.licenseKey.asString();
            return config;
        });
        this.client.interceptors.response.use((response) => response, (error) => {
            if (error.response) {
                const data = error.response.data;
                throw new types_1.TruthlinkedError(data?.error || error.message, error.response.status, data);
            }
            throw new types_1.TruthlinkedError(error.message);
        });
    }
    async health() {
        const response = await this.client.get('/health');
        return response.data;
    }
    async requestToken(request) {
        const response = await this.client.post('/v1/token', request);
        return response.data;
    }
    async validateToken(token) {
        const response = await this.client.post('/v1/validate', { token });
        return response.data;
    }
    async authorize(token, permission) {
        try {
            const response = await this.client.post('/v1/authorize', {
                token,
                permission
            });
            return response.data.authorized === true;
        }
        catch (error) {
            if (error instanceof types_1.TruthlinkedError && error.statusCode === 403) {
                return false;
            }
            throw error;
        }
    }
    async submitWitness(submission) {
        const response = await this.client.post('/witness/submit', { submission });
        return response.data;
    }
    async getWitnessEvent(sequence, includeProof = false) {
        const response = await this.client.get(`/witness/event/${sequence}`, { params: { include_proof: includeProof } });
        return response.data;
    }
    async getLatestSTH() {
        const response = await this.client.get('/witness/sth/latest');
        return response.data;
    }
    async getSTH(treeSize) {
        const response = await this.client.get(`/witness/sth/${treeSize}`);
        return response.data;
    }
    async exportWitnessChain(startSeq, endSeq) {
        const params = {};
        if (startSeq !== undefined)
            params.start_seq = startSeq;
        if (endSeq !== undefined)
            params.end_seq = endSeq;
        const response = await this.client.get('/witness/export', {
            params,
            responseType: 'blob'
        });
        return response.data;
    }
    async witnessHealth() {
        const response = await this.client.get('/witness/health');
        return response.data;
    }
    destroy() {
        this.licenseKey.destroy();
        if (this.signer) {
            this.signer.destroy();
        }
    }
}
exports.TruthlinkedClient = TruthlinkedClient;
