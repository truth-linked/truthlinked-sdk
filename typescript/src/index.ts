export { TruthlinkedClient } from './client';
export {
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
export {
  RequestSigner,
  SecureLicenseKey,
  generateNonce,
  sha256,
  generateKeyPair,
  signData,
  verifySignature,
  constantTimeEqual
} from './crypto';

import { TruthlinkedClient } from './client';
export default { TruthlinkedClient };
