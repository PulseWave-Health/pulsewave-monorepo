# Pulsewave Monorepo

Customer validation pipeline built on Stellar/Soroban.

## Structure

```
pulsewave-monorepo/
├── apps/
│   ├── web/        # Next.js dashboard (port 3000)
│   ├── api/        # Node.js/Express REST API (port 3001)
│   └── mobile/     # React Native (Expo) mobile app
├── contracts/
│   └── pulsewave/  # Soroban smart contract (Rust)
└── packages/
    ├── types/      # Shared TypeScript types
    └── config/     # Shared config (API URL, Stellar network)
```

## Pipeline

1. **Mobile** — customer submits data hash via app → calls API
2. **API** — stores submission, relays to Soroban contract
3. **Contract** — on-chain record: `pending → validated | rejected`
4. **Web** — admin dashboard to review and validate customers

## Getting Started

```bash
# Install dependencies
pnpm install

# Run all apps in dev mode
pnpm dev

# Build everything
pnpm build
```

## Contract (Soroban)

```bash
# Build
cd contracts/pulsewave
cargo build --target wasm32-unknown-unknown --release

# Test
cargo test
```

## Environment Variables

Copy `.env.example` to `.env` in each app:

| Variable              | Description                        |
|-----------------------|------------------------------------|
| `STELLAR_NETWORK`     | `testnet` or `mainnet`             |
| `STELLAR_RPC_URL`     | Soroban RPC endpoint               |
| `CONTRACT_ID`         | Deployed contract address          |
| `NEXT_PUBLIC_API_URL` | API base URL for the web frontend  |
