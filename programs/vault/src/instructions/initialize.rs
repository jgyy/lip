use anchor_lang::prelude::*;
use crate::state::Vault;
use crate::errors::VaultError;

pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    let vault = &mut ctx.accounts.vault;

    vault.total_assets = 0;
    vault.total_shares = 0;
    vault.admin = ctx.accounts.admin.key();
    vault.strategy_allocation = 50; // Default 50% allocation
    vault.total_yield = 0;
    vault.accumulated_fees = 0;
    vault.num_users = 0;
    vault.bump = ctx.bumps.vault;

    Ok(())
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    /// Vault account to initialize
    #[account(init, payer = admin, space = Vault::LEN)]
    pub vault: Account<'info, Vault>,

    /// Admin/owner of the vault
    #[account(mut)]
    pub admin: Signer<'info>,

    /// System program for account creation
    pub system_program: Program<'info, System>,
}
