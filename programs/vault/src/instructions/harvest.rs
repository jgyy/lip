use anchor_lang::prelude::*;
use crate::state::Vault;
use crate::errors::VaultError;

/// Harvest yield from strategies (called by authority/keeper)
/// This simulates the vault earning yield from deployed strategies
pub fn harvest(ctx: Context<Harvest>, yield_amount: u64) -> Result<()> {
    if yield_amount == 0 {
        return Err(VaultError::NoYield.into());
    }

    require_keys_eq!(ctx.accounts.vault.admin, ctx.accounts.admin.key(), VaultError::Unauthorized);

    let vault = &mut ctx.accounts.vault;

    // Calculate performance fee (10% of yield)
    let fee = yield_amount / 10;
    let net_yield = yield_amount.checked_sub(fee)
        .ok_or(VaultError::OverflowError)?;

    // Update vault state
    vault.total_yield = vault.total_yield.checked_add(net_yield)
        .ok_or(VaultError::OverflowError)?;
    vault.accumulated_fees = vault.accumulated_fees.checked_add(fee)
        .ok_or(VaultError::OverflowError)?;
    vault.total_assets = vault.total_assets.checked_add(net_yield)
        .ok_or(VaultError::OverflowError)?;

    Ok(())
}

#[derive(Accounts)]
pub struct Harvest<'info> {
    /// Vault account
    #[account(mut)]
    pub vault: Account<'info, Vault>,

    /// Admin (only admin can harvest)
    pub admin: Signer<'info>,
}
