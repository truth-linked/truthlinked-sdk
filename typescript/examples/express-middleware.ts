/**
 * Express.js middleware example
 */
import express from 'express';
import { TruthlinkedClient, TruthlinkedError } from '../src';

const app = express();
app.use(express.json());

const client = new TruthlinkedClient({
  baseUrl: process.env.TRUTHLINKED_BASE_URL!,
  licenseKey: process.env.TRUTHLINKED_LICENSE_KEY!
});

// Authentication middleware
const authenticate = async (req: any, res: any, next: any) => {
  const token = req.headers.authorization?.replace('Bearer ', '');
  
  if (!token) {
    return res.status(401).json({ error: 'No token provided' });
  }

  try {
    const validation = await client.validateToken(token);
    if (!validation.valid) {
      return res.status(401).json({ error: 'Invalid token' });
    }
    req.user = validation.token;
    next();
  } catch (error) {
    if (error instanceof TruthlinkedError) {
      return res.status(error.statusCode || 500).json({ error: error.message });
    }
    res.status(500).json({ error: 'Authentication failed' });
  }
};

// Permission middleware
const requirePermission = (permission: string) => {
  return async (req: any, res: any, next: any) => {
    const token = req.headers.authorization?.replace('Bearer ', '');
    
    try {
      const authorized = await client.authorize(token!, permission);
      if (!authorized) {
        return res.status(403).json({ error: 'Insufficient permissions' });
      }
      next();
    } catch (error) {
      res.status(500).json({ error: 'Authorization failed' });
    }
  };
};

// Protected routes
app.get('/api/data', authenticate, (req, res) => {
  res.json({ message: 'Protected data', user: req.user });
});

app.post('/api/admin', authenticate, requirePermission('admin:write'), (req, res) => {
  res.json({ message: 'Admin action performed' });
});

app.listen(3001, () => {
  console.log('Server running on http://localhost:3001');
});
