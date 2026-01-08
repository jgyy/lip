use anchor_lang::prelude::*;
use crate::state::Vault;
use crate::errors::VaultError;

/// Update vault settings (Admin role)
pub fn update_settings(
    ctx: Context<UpdateSettings>,
    strategy_allocation: Option<u8>,
) -> Result<()> {
    let vault = &mut ctx.accounts.vault;

    // Verify caller is vault admin (for now, without RBAC integration)
    require_keys_eq!(vault.admin, ctx.accounts.admin.key(), VaultError::Unauthorized);

    // Update strategy allocation if provided
    if let Some(allocation) = strategy_allocation {
        if allocation > 100 {
            return Err(VaultError::InvalidAllocation.into());
        }
        vault.strategy_allocation = allocation;
    }

    Ok(())
}

#[derive(Accounts)]
pub struct UpdateSettings<'info> {
    /// Vault account
    #[account(mut)]
    pub vault: Account<'info, Vault>,

    /// Admin (must match vault.admin)
    pub admin: Signer<'info>,
}
