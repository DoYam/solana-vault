# Solana Vault

A secure asset management Vault program on Solana blockchain.

> **Note**: This project was developed with the assistance of AI.

## Overview

This project is a Vault program developed using Solana + Anchor framework.

## Development Plan

### Phase 1: Development (4 hours)
- [v] Solana + Anchor installation
- [v] Vault program implementation
- [v] Test writing
- [ ] Local test passing

### Phase 2: Deployment (1 hour)
- [ ] Devnet deployment
- [ ] Program ID verification
- [ ] Solana Explorer verification
- [ ] Test re-execution

### Phase 3: Documentation (30 minutes)
- [v] README writing
- [v] Functionality explanation document
- [ ] Screenshots addition
- [v] GitHub upload

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

## Features

### 1. Initialize Vault

Creates a new vault account using PDA (Program Derived Address) for deterministic addressing.

```rust
pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    vault.owner = ctx.accounts.owner.key();
    vault.total_deposited = 0;
    vault.total_withdrawn = 0;
    Ok(())
}
```

**Key Points:**
- Uses PDA with seeds `[b"vault", owner.key()]` for unique vault address
- Owner pays for account creation (rent)
- Account space: 8 (discriminator) + 32 (Pubkey) + 8 (u64) + 8 (u64) = 56 bytes

### 2. Deposit SOL

Allows any user to deposit SOL into the vault.

```rust
pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    // Transfer SOL from user to vault
    anchor_lang::solana_program::program::invoke(
        &anchor_lang::solana_program::system_instruction::transfer(
            &user.key(),
            &vault.key(),
            amount,
        ),
        &[user.to_account_info(), vault.to_account_info(), ...],
    )?;
    
    // Update total deposited with overflow protection
    vault.total_deposited = vault.total_deposited
        .checked_add(amount)
        .ok_or(ErrorCode::Overflow)?;
    Ok(())
}
```

**Key Points:**
- Uses system program's transfer instruction for secure SOL transfer
- Updates `total_deposited` counter with overflow protection
- Any user can deposit, no authorization required

### 3. Withdraw SOL

Only the vault owner can withdraw funds from the vault.

```rust
pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    // Authorization check
    require!(
        vault.owner == ctx.accounts.owner.key(),
        ErrorCode::Unauthorized
    );
    
    // Sufficient funds check
    require!(
        **vault.to_account_info().lamports.borrow() >= amount,
        ErrorCode::InsufficientFunds
    );
    
    // Direct lamports manipulation for transfer
    **vault.to_account_info().try_borrow_mut_lamports()? -= amount;
    **ctx.accounts.recipient.to_account_info().try_borrow_mut_lamports()? += amount;
    
    Ok(())
}
```

**Key Points:**
- Owner-only authorization check
- Insufficient funds validation
- Direct lamports manipulation (more gas efficient than system transfer)
- Updates `total_withdrawn` counter

### 4. Account Structure

```rust
#[account]
pub struct Vault {
    pub owner: Pubkey,           // Vault owner's public key
    pub total_deposited: u64,    // Total deposited (lamports)
    pub total_withdrawn: u64,    // Total withdrawn (lamports)
}
```

**Key Points:**
- `#[account]` macro provides serialization/deserialization
- Stores owner for authorization
- Tracks deposit/withdraw statistics

### 5. Security Features

- **PDA (Program Derived Address)**: Deterministic vault addresses prevent address collisions
- **Owner Authorization**: Only owner can withdraw funds
- **Overflow Protection**: Uses `checked_add` to prevent integer overflow
- **Insufficient Funds Check**: Validates vault balance before withdrawal
- **Account Constraints**: Anchor's constraint system ensures account validity

## License

ISC

