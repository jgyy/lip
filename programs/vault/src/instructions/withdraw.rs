use anchor_lang::prelude::*;
use anchor_lang::system_program;
use crate::state::{Vault, UserPosition};
use crate::errors::VaultError;

const TIME_LOCK_DURATION: i64 = 24 * 60 * 60; // 24 hours

pub fn withdraw(ctx: Context<Withdraw>, shares: u64) -> Result<()> {
    if shares == 0 {
        return Err(VaultError::InvalidShares.into());
    }

    let vault = &mut ctx.accounts.vault;
    let user_position = &mut ctx.accounts.user_position;

    // Check time lock (prevent immediate withdrawal)
    let current_time = Clock::get()?.unix_timestamp;
    let time_since_deposit = current_time.checked_sub(user_position.deposit_timestamp)
        .ok_or(VaultError::OverflowError)?;

    if time_since_deposit < TIME_LOCK_DURATION {
        return Err(VaultError::TimeLockActive.into());
    }

    // Verify user has enough shares
    if shares > user_position.shares {
        return Err(VaultError::InsufficientBalance.into());
    }

    // Calculate assets to withdraw
    let assets = vault.assets_for_shares(shares);
    if assets == 0 {
        return Err(VaultError::InvalidAmount.into());
    }

    // Transfer SOL back to user
    // In a real implementation, this would use a PDA signer
    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.vault_account.to_account_info(),
                to: ctx.accounts.user.to_account_info(),
            },
        ),
        assets,
    )?;

    // Update vault state
    vault.total_assets = vault.total_assets.checked_sub(assets)
        .ok_or(VaultError::OverflowError)?;
    vault.total_shares = vault.total_shares.checked_sub(shares)
        .ok_or(VaultError::OverflowError)?;

    // Update user position
    user_position.shares = user_position.shares.checked_sub(shares)
        .ok_or(VaultError::OverflowError)?;

    Ok(())
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    /// Vault account
    #[account(mut)]
    pub vault: Account<'info, Vault>,

    /// User's wallet
    #[account(mut)]
    pub user: Signer<'info>,

    /// Vault SOL account
    #[account(mut)]
    pub vault_account: AccountInfo<'info>,

    /// User position
    #[account(
        mut,
        seeds = [b"user_position", vault.key().as_ref(), user.key().as_ref()],
        bump = user_position.bump
    )]
    pub user_position: Account<'info, UserPosition>,

    /// System program
    pub system_program: Program<'info, System>,
}
