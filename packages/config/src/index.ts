export const config = {
  api: {
    baseUrl: process.env.NEXT_PUBLIC_API_URL ?? "http://localhost:3001",
  },
  stellar: {
    network: (process.env.STELLAR_NETWORK ?? "testnet") as "testnet" | "mainnet",
    rpcUrl:
      process.env.STELLAR_RPC_URL ?? "https://soroban-testnet.stellar.org",
    contractId: process.env.CONTRACT_ID ?? "",
  },
} as const;
