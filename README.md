# Solana Vault

A secure asset management Vault program on Solana blockchain.

## Overview

This project is a Vault program developed using Solana + Anchor framework.

## Development Plan

### Phase 1: Development (4 hours)
- [x] Solana + Anchor installation
- [ ] Vault program implementation
- [ ] Test writing
- [ ] Local test passing

### Phase 2: Deployment (1 hour)
- [ ] Devnet deployment
- [ ] Program ID verification
- [ ] Solana Explorer verification
- [ ] Test re-execution

### Phase 3: Documentation (30 minutes)
- [x] README writing
- [ ] Screenshots addition
- [x] GitHub upload

## Tech Stack

- **Solana CLI**: 1.18.20
- **Anchor**: 0.32.1
- **Rust**: 1.92.0

## Getting Started

### Prerequisites

- Rust (1.92.0 or higher)
- Solana CLI
- Anchor CLI
- Node.js (v20.18.0 or higher recommended)

### Installation

```bash
npm install
```

or

```bash
yarn install
```

### Build

```bash
anchor build
```

### Test

```bash
anchor test
```

## Project Structure

```
solana-anchor-project/
├── Anchor.toml
├── programs/
│   └── solana-vault/
│       └── src/
│           └── lib.rs
├── tests/
│   └── solana-vault.ts
└── migrations/
```

## License

ISC

