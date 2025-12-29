use anchor_lang::prelude::*;

declare_id!("HaV2KjsCW2hPKM3Z43F2w6JJELccBieXDcsPnuV8Qy77");

#[program]
pub mod solana_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Solana Vault initialized: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
