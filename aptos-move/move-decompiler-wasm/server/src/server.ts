import express, { Request, Response } from 'express';
import cors from 'cors';
import path from 'path';
import { AptosClient } from './aptos-client';
import { NetworkCache } from './cache';

const app = express();
const PORT = process.env.PORT || 3000;

// Initialize Aptos clients for different networks
const clients = {
  mainnet: new AptosClient('https://fullnode.mainnet.aptoslabs.com/v1'),
  testnet: new AptosClient('https://fullnode.testnet.aptoslabs.com/v1'),
  devnet: new AptosClient('https://fullnode.devnet.aptoslabs.com/v1'),
};

// Initialize cache (5 minute TTL)
const cache = new NetworkCache(300);

// Middleware
app.use(cors());
app.use(express.json());
app.use(express.static(path.join(__dirname, '../../')));

// Health check
app.get('/api/health', (req: Request, res: Response) => {
  res.json({
    status: 'ok',
    server: 'move-decompiler-server',
    version: '1.0.0',
    networks: Object.keys(clients),
  });
});

// Get account modules (list)
app.get('/api/modules/:network/:address', async (req: Request, res: Response) => {
  try {
    const { network, address } = req.params;

    // Validate network
    if (!['mainnet', 'testnet', 'devnet'].includes(network)) {
      return res.status(400).json({
        error: 'Invalid network. Use: mainnet, testnet, or devnet'
      });
    }

    // Check cache
    const cacheKey = `modules:${network}:${address}`;
    const cached = cache.get(cacheKey);
    if (cached) {
      console.log(`Cache hit: ${cacheKey}`);
      return res.json({ ...cached, cached: true });
    }

    console.log(`Fetching modules for ${address} on ${network}...`);
    const client = clients[network as keyof typeof clients];
    const modules = await client.getAccountModules(address);

    const result = {
      network,
      address,
      modules: modules.map(m => ({
        name: m.name,
        address: m.address,
        abi: m.abi,
      })),
      count: modules.length,
      timestamp: new Date().toISOString(),
    };

    // Cache the result
    cache.set(cacheKey, result);

    res.json(result);
  } catch (error: any) {
    console.error('Error fetching modules:', error);
    res.status(500).json({
      error: error.message || 'Failed to fetch modules',
      details: error.response?.data || error.toString(),
    });
  }
});

// Get specific module bytecode
app.get('/api/module/:network/:address/:moduleName', async (req: Request, res: Response) => {
  try {
    const { network, address, moduleName } = req.params;

    // Validate network
    if (!['mainnet', 'testnet', 'devnet'].includes(network)) {
      return res.status(400).json({
        error: 'Invalid network. Use: mainnet, testnet, or devnet'
      });
    }

    // Check cache
    const cacheKey = `module:${network}:${address}:${moduleName}`;
    const cached = cache.get(cacheKey);
    if (cached) {
      console.log(`Cache hit: ${cacheKey}`);
      return res.json({ ...cached, cached: true });
    }

    console.log(`Fetching module ${moduleName} for ${address} on ${network}...`);
    const client = clients[network as keyof typeof clients];
    const module = await client.getAccountModule(address, moduleName);

    if (!module || !module.bytecode) {
      return res.status(404).json({
        error: `Module ${moduleName} not found at address ${address}`
      });
    }

    // Convert hex string to byte array
    const bytecodeHex = module.bytecode.startsWith('0x')
      ? module.bytecode.slice(2)
      : module.bytecode;

    const bytecodeArray = Array.from(
      Buffer.from(bytecodeHex, 'hex')
    );

    const result = {
      network,
      address,
      moduleName,
      bytecode: bytecodeArray,
      bytecodeHex: module.bytecode,
      bytecodeSize: bytecodeArray.length,
      abi: module.abi,
      timestamp: new Date().toISOString(),
    };

    // Cache the result
    cache.set(cacheKey, result);

    res.json(result);
  } catch (error: any) {
    console.error('Error fetching module:', error);
    res.status(500).json({
      error: error.message || 'Failed to fetch module',
      details: error.response?.data || error.toString(),
    });
  }
});

// Search for modules by partial name
app.get('/api/search/:network/:address/:query', async (req: Request, res: Response) => {
  try {
    const { network, address, query } = req.params;

    if (!['mainnet', 'testnet', 'devnet'].includes(network)) {
      return res.status(400).json({
        error: 'Invalid network. Use: mainnet, testnet, or devnet'
      });
    }

    const client = clients[network as keyof typeof clients];
    const modules = await client.getAccountModules(address);

    const filteredModules = modules
      .filter(m => m.name.toLowerCase().includes(query.toLowerCase()))
      .map(m => ({
        name: m.name,
        address: m.address,
        functions: m.abi?.exposed_functions?.length || 0,
        structs: m.abi?.structs?.length || 0,
      }));

    res.json({
      network,
      address,
      query,
      results: filteredModules,
      count: filteredModules.length,
    });
  } catch (error: any) {
    console.error('Error searching modules:', error);
    res.status(500).json({
      error: error.message || 'Failed to search modules',
    });
  }
});

// Get popular/well-known addresses
app.get('/api/well-known', (req: Request, res: Response) => {
  const wellKnownAddresses = [
    {
      name: 'Aptos Framework',
      address: '0x1',
      network: 'all',
      description: 'Core Aptos framework modules (coin, account, etc.)',
      popularModules: ['coin', 'account', 'timestamp', 'event', 'aptos_coin'],
    },
    {
      name: 'Aptos Token',
      address: '0x3',
      network: 'all',
      description: 'Token standard implementation',
      popularModules: ['token', 'token_transfers'],
    },
    {
      name: 'Aptos Names',
      address: '0x867ed1f6bf916171b1de3ee92849b8978b7d1b9e0a8cc982a3d19d535dfd9c0c',
      network: 'mainnet',
      description: 'Aptos Name Service (ANS)',
      popularModules: ['domains'],
    },
  ];

  res.json(wellKnownAddresses);
});

// Serve the web app
app.get('/', (req: Request, res: Response) => {
  res.sendFile(path.join(__dirname, '../../demo/index.html'));
});

// Start server
app.listen(PORT, () => {
  console.log(`
╔════════════════════════════════════════════════════════════╗
║                                                            ║
║        🚀 Move Decompiler Server Running!                 ║
║                                                            ║
║        URL: http://localhost:${PORT}                          ║
║        Demo: http://localhost:${PORT}/demo/                   ║
║                                                            ║
║        Networks: mainnet, testnet, devnet                  ║
║                                                            ║
╚════════════════════════════════════════════════════════════╝

API Endpoints:
  GET  /api/health
  GET  /api/modules/:network/:address
  GET  /api/module/:network/:address/:moduleName
  GET  /api/search/:network/:address/:query
  GET  /api/well-known

Examples:
  curl http://localhost:${PORT}/api/modules/mainnet/0x1
  curl http://localhost:${PORT}/api/module/mainnet/0x1/coin
  curl http://localhost:${PORT}/api/well-known
  `);
});
