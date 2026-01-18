# Truthlinked TypeScript SDK - Examples

## Running Examples

### Prerequisites

```bash
npm install
npm run build
```

### Set Environment Variables

```bash
export TRUTHLINKED_BASE_URL=http://localhost:3000
export TRUTHLINKED_LICENSE_KEY=your-hex-key
```

### Basic Usage

```bash
npx ts-node examples/basic.ts
```

### Witness Chain

```bash
npx ts-node examples/witness.ts
```

### Express Middleware

```bash
npm install express @types/express
npx ts-node examples/express-middleware.ts
```

## Example Descriptions

- **basic.ts** - Token request, validation, and authorization
- **witness.ts** - Witness chain submission and verification
- **express-middleware.ts** - Authentication and authorization middleware for Express.js
