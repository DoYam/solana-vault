use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111111");

#[program]
pub mod solana_vault {
    use super::*;

    /// Initialize a new vault for the owner
    /// Creates a PDA (Program Derived Address) vault account
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.owner = ctx.accounts.owner.key();
        vault.total_deposited = 0;
        vault.total_withdrawn = 0;
        msg!("Vault initialized for owner: {:?}", vault.owner);
        Ok(())
    }

    /// Deposit SOL into the vault
    /// Transfers lamports from user to vault account
    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        let user = &ctx.accounts.user;
        
        // Transfer SOL from user to vault using system program
        anchor_lang::solana_program::program::invoke(
            &anchor_lang::solana_program::system_instruction::transfer(
                &user.key(),
                &vault.key(),
                amount,
            ),
            &[
                user.to_account_info(),
                vault.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        // Update total deposited amount with overflow protection
        vault.total_deposited = vault.total_deposited
            .checked_add(amount)
            .ok_or(ErrorCode::Overflow)?;
        
        msg!("Deposited {} lamports to vault", amount);
        Ok(())
    }

    /// Withdraw SOL from the vault
    /// Only the vault owner can withdraw funds
    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        
        // Check if caller is the vault owner
        require!(
            vault.owner == ctx.accounts.owner.key(),
            ErrorCode::Unauthorized
        );

        // Check if vault has sufficient funds
        require!(
            **vault.to_account_info().lamports.borrow() >= amount,
            ErrorCode::InsufficientFunds
        );

        // Transfer lamports from vault to recipient
        **vault.to_account_info().try_borrow_mut_lamports()? -= amount;
        **ctx.accounts.recipient.to_account_info().try_borrow_mut_lamports()? += amount;

        // Update total withdrawn amount with overflow protection
        vault.total_withdrawn = vault.total_withdrawn
            .checked_add(amount)
            .ok_or(ErrorCode::Overflow)?;

        msg!("Withdrew {} lamports from vault", amount);
        Ok(())
    }
}

/// Vault account structure
/// Stores owner address and deposit/withdraw statistics
#[account]
pub struct Vault {
    pub owner: Pubkey,           // Vault owner's public key
    pub total_deposited: u64,    // Total amount deposited (in lamports)
    pub total_withdrawn: u64,    // Total amount withdrawn (in lamports)
}

/// Accounts required for vault initialization
/// Creates a new vault PDA account
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,                                    // Initialize new account
        payer = owner,                           // Owner pays for account creation
        space = 8 + 32 + 8 + 8,                 // Discriminator + owner + 2 u64
        seeds = [b"vault", owner.key().as_ref()], // PDA seeds
        bump                                     // Bump seed for PDA derivation
    )]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub owner: Signer<'info>,                    // Vault owner (must sign)
    pub system_program: Program<'info, System>,  // System program for account creation
}

/// Accounts required for deposit operation
/// User deposits SOL into the vault
#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(
        mut,                                      // Vault account is mutable
        seeds = [b"vault", vault.owner.as_ref()], // Find vault by PDA seeds
        bump
    )]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub user: Signer<'info>,                     // User depositing funds (must sign)
    pub system_program: Program<'info, System>,  // System program for transfer
}

/// Accounts required for withdraw operation
/// Owner withdraws SOL from the vault
#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(
        mut,                                      // Vault account is mutable
        seeds = [b"vault", owner.key().as_ref()], // Find vault by PDA seeds
        bump
    )]
    pub vault: Account<'info, Vault>,
    pub owner: Signer<'info>,                     // Vault owner (must sign)
    #[account(mut)]
    pub recipient: SystemAccount<'info>,          // Recipient of withdrawn funds
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized: Only vault owner can withdraw")]
    Unauthorized,
    #[msg("Insufficient funds in vault")]
    InsufficientFunds,
    #[msg("Arithmetic overflow")]
    Overflow,
}
