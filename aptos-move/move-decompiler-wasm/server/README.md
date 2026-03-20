# Move Decompiler Server (TypeScript)

**Node.js/Express server with on-chain module lookup**

This TypeScript server provides:
- ✅ Static file serving for the web app
- ✅ REST API to fetch modules from Aptos blockchain
- ✅ Support for mainnet, testnet, and devnet
- ✅ In-memory caching for performance
- ✅ Well-known address database

## Quick Start

### Prerequisites

- Node.js 18+ and npm
- The WASM module already built (in `../pkg/`)

### Install Dependencies

```bash
cd /opt/git/aptos-core/aptos-move/move-decompiler-wasm/server
npm install
```

### Run Development Server

```bash
npm run dev
```

The server will start on http://localhost:3000

### Build for Production

```bash
npm run build
npm start
```

## API Endpoints

### GET /api/health

Health check endpoint.

**Response:**
```json
{
  "status": "ok",
  "server": "move-decompiler-server",
  "version": "1.0.0",
  "networks": ["mainnet", "testnet", "devnet"]
}
```

### GET /api/modules/:network/:address

Get all modules for an account.

**Parameters:**
- `network`: mainnet | testnet | devnet
- `address`: Account address (0x1, 0x42, or full address)

**Example:**
```bash
curl http://localhost:3000/api/modules/mainnet/0x1
```

**Response:**
```json
{
  "network": "mainnet",
  "address": "0x0000...0001",
  "modules": [
    {
      "name": "coin",
      "address": "0x1",
      "abi": { ... }
    }
  ],
  "count": 42,
  "timestamp": "2024-03-20T..."
}
```

### GET /api/module/:network/:address/:moduleName

Get a specific module's bytecode.

**Parameters:**
- `network`: mainnet | testnet | devnet
- `address`: Account address
- `moduleName`: Module name (e.g., "coin")

**Example:**
```bash
curl http://localhost:3000/api/module/mainnet/0x1/coin
```

**Response:**
```json
{
  "network": "mainnet",
  "address": "0x0000...0001",
  "moduleName": "coin",
  "bytecode": [161, 28, 235, ...],
  "bytecodeHex": "0xa11ceb0b...",
  "bytecodeSize": 12345,
  "abi": { ... },
  "timestamp": "2024-03-20T..."
}
```

### GET /api/search/:network/:address/:query

Search modules by name.

**Parameters:**
- `network`: mainnet | testnet | devnet
- `address`: Account address
- `query`: Search query (partial module name)

**Example:**
```bash
curl http://localhost:3000/api/search/mainnet/0x1/coin
```

### GET /api/well-known

Get list of well-known addresses.

**Response:**
```json
[
  {
    "name": "Aptos Framework",
    "address": "0x1",
    "network": "all",
    "description": "Core Aptos framework modules",
    "popularModules": ["coin", "account", "timestamp"]
  }
]
```

## Environment Variables

```bash
# Server port (default: 3000)
PORT=3000

# Cache TTL in seconds (default: 300)
CACHE_TTL=300
```

## Project Structure

```
server/
├── src/
│   ├── server.ts        # Main Express app
│   ├── aptos-client.ts  # Aptos blockchain client
│   └── cache.ts         # In-memory cache
├── dist/                # Compiled JavaScript (after build)
├── package.json
├── tsconfig.json
└── README.md
```

## Development

### Watch Mode

```bash
npm run dev
```

Changes auto-reload with tsx watch.

### Type Checking

```bash
npm run type-check
```

### Build

```bash
npm run build
```

Output goes to `dist/` directory.

## Caching

The server caches API responses for 5 minutes (300 seconds) by default.

**Cache keys:**
- Modules list: `modules:{network}:{address}`
- Specific module: `module:{network}:{address}:{moduleName}`

**Cache stats:**

The cache automatically cleans up expired entries every minute.

## Features

### Multi-Network Support

The server connects to three Aptos networks:

```typescript
const clients = {
  mainnet: new AptosClient('https://fullnode.mainnet.aptoslabs.com/v1'),
  testnet: new AptosClient('https://fullnode.testnet.aptoslabs.com/v1'),
  devnet: new AptosClient('https://fullnode.devnet.aptoslabs.com/v1'),
};
```

### Address Normalization

Addresses are automatically normalized:
- `0x1` → `0x0000000000000000000000000000000000000000000000000000000000000001`
- Removes `0x` prefix, pads to 64 characters, re-adds `0x`

### Error Handling

Graceful error handling with detailed error messages:

```json
{
  "error": "Failed to fetch module",
  "details": "Module not found at address 0x1"
}
```

## Testing

### Test Endpoints

```bash
# Health check
curl http://localhost:3000/api/health

# Get Aptos Framework modules
curl http://localhost:3000/api/modules/mainnet/0x1

# Get specific module
curl http://localhost:3000/api/module/mainnet/0x1/coin

# Search modules
curl http://localhost:3000/api/search/mainnet/0x1/coin

# Well-known addresses
curl http://localhost:3000/api/well-known
```

### Test with the Web App

1. Start the server: `npm run dev`
2. Open browser: http://localhost:3000/demo/index-enhanced.html
3. Try the on-chain lookup features!

## Production Deployment

### Docker (Optional)

Create `Dockerfile`:

```dockerfile
FROM node:20-alpine

WORKDIR /app

COPY package*.json ./
RUN npm ci --production

COPY . .
RUN npm run build

EXPOSE 3000

CMD ["node", "dist/server.js"]
```

Build and run:

```bash
docker build -t move-decompiler-server .
docker run -p 3000:3000 move-decompiler-server
```

### Environment-Based Config

```bash
# Production
PORT=8080 NODE_ENV=production npm start

# Development
PORT=3000 npm run dev
```

## Troubleshooting

### "Cannot find module" errors

Make sure you ran `npm install`:

```bash
npm install
```

### TypeScript compilation errors

Check your Node.js version:

```bash
node --version  # Should be 18+
```

### CORS errors in browser

The server has CORS enabled for all origins. If you still see errors, check:
- Browser console for specific error
- Make sure server is running on expected port
- Try accessing from `localhost` instead of `127.0.0.1`

### Aptos API errors

Check network connectivity:

```bash
curl https://fullnode.mainnet.aptoslabs.com/v1
```

If down, try different network (testnet, devnet).

## Future Enhancements

- [ ] WebSocket support for real-time updates
- [ ] Rate limiting
- [ ] User authentication
- [ ] Request logging
- [ ] Metrics/monitoring
- [ ] GraphQL API
- [ ] Module dependency graph visualization
- [ ] Historical module version tracking

## License

Apache-2.0
