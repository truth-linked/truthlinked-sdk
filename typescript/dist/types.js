"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.TruthlinkedError = void 0;
class TruthlinkedError extends Error {
    constructor(message, statusCode, response) {
        super(message);
        this.statusCode = statusCode;
        this.response = response;
        this.name = 'TruthlinkedError';
    }
}
exports.TruthlinkedError = TruthlinkedError;
