"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || (function () {
    var ownKeys = function(o) {
        ownKeys = Object.getOwnPropertyNames || function (o) {
            var ar = [];
            for (var k in o) if (Object.prototype.hasOwnProperty.call(o, k)) ar[ar.length] = k;
            return ar;
        };
        return ownKeys(o);
    };
    return function (mod) {
        if (mod && mod.__esModule) return mod;
        var result = {};
        if (mod != null) for (var k = ownKeys(mod), i = 0; i < k.length; i++) if (k[i] !== "default") __createBinding(result, mod, k[i]);
        __setModuleDefault(result, mod);
        return result;
    };
})();
Object.defineProperty(exports, "__esModule", { value: true });
exports.SecureLicenseKey = exports.RequestSigner = void 0;
exports.generateNonce = generateNonce;
exports.sha256 = sha256;
exports.generateKeyPair = generateKeyPair;
exports.signData = signData;
exports.verifySignature = verifySignature;
exports.constantTimeEqual = constantTimeEqual;
const crypto = __importStar(require("crypto"));
const nacl = __importStar(require("tweetnacl"));
const util = __importStar(require("tweetnacl-util"));
class RequestSigner {
    constructor(licenseKey) {
        const hmac = crypto.createHmac('sha256', 'truthlinked-request-signing-v1');
        hmac.update(licenseKey);
        this.signingKey = hmac.digest();
    }
    sign(method, path, body) {
        const timestamp = Math.floor(Date.now() / 1000).toString();
        const bodyStr = body ? JSON.stringify(body) : '';
        const message = `${method}\n${path}\n${timestamp}\n${bodyStr}`;
        const hmac = crypto.createHmac('sha256', this.signingKey);
        hmac.update(message);
        const signature = hmac.digest('base64');
        return { timestamp, signature };
    }
    verify(method, path, body, signature, timestamp) {
        const message = `${method}\n${path}\n${timestamp}\n${body}`;
        const hmac = crypto.createHmac('sha256', this.signingKey);
        hmac.update(message);
        const expected = hmac.digest('base64');
        return crypto.timingSafeEqual(Buffer.from(signature, 'base64'), Buffer.from(expected, 'base64'));
    }
    destroy() {
        this.signingKey.fill(0);
    }
}
exports.RequestSigner = RequestSigner;
class SecureLicenseKey {
    constructor(key) {
        this.key = key;
    }
    asString() {
        return this.key;
    }
    redacted() {
        const len = this.key.length;
        if (len > 8) {
            return `${this.key.slice(0, 3)}...${this.key.slice(-3)}`;
        }
        return '***';
    }
    destroy() {
        this.key = '\0'.repeat(this.key.length);
    }
    toString() {
        return this.redacted();
    }
}
exports.SecureLicenseKey = SecureLicenseKey;
function generateNonce() {
    return crypto.randomBytes(32).toString('hex');
}
function sha256(data) {
    return crypto.createHash('sha256').update(data).digest('hex');
}
function generateKeyPair() {
    const keypair = nacl.sign.keyPair();
    return {
        publicKey: util.encodeBase64(keypair.publicKey),
        secretKey: util.encodeBase64(keypair.secretKey)
    };
}
function signData(data, secretKey) {
    const message = Buffer.from(data, 'utf8');
    const key = util.decodeBase64(secretKey);
    const signature = nacl.sign.detached(message, key);
    return util.encodeBase64(signature);
}
function verifySignature(data, signature, publicKey) {
    const message = Buffer.from(data, 'utf8');
    const sig = util.decodeBase64(signature);
    const key = util.decodeBase64(publicKey);
    return nacl.sign.detached.verify(message, sig, key);
}
function constantTimeEqual(a, b) {
    if (a.length !== b.length) {
        return false;
    }
    return crypto.timingSafeEqual(Buffer.from(a), Buffer.from(b));
}
