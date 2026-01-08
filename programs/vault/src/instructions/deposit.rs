use anchor_lang::prelude::*;
use anchor_lang::system_program;
use crate::state::{Vault, UserPosition};
use crate::errors::VaultError;

pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    if amount == 0 {
        return Err(VaultError::InvalidAmount.into());
    }

    let vault = &mut ctx.accounts.vault;

    // Calculate shares to mint
    let shares = vault.shares_for_deposit(amount);
    if shares == 0 {
        return Err(VaultError::InvalidShares.into());
    }

    // Transfer SOL from user to vault
    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.user.to_account_info(),
                to: ctx.accounts.vault_account.to_account_info(),
            },
        ),
        amount,
    )?;

    // Update vault state
    vault.total_assets = vault
        .total_assets
        .checked_add(amount)
        .ok_or(VaultError::OverflowError)?;
    vault.total_shares = vault
        .total_shares
        .checked_add(shares)
        .ok_or(VaultError::OverflowError)?;

    // Create or update user position
    let user_position = &mut ctx.accounts.user_position;
    if user_position.shares == 0 {
        // New user
        user_position.user = ctx.accounts.user.key();
        user_position.vault = vault.key();
        user_position.deposit_timestamp = Clock::get()?.unix_timestamp;
        user_position.total_deposited = amount;
        vault.num_users = vault.num_users.checked_add(1).ok_or(VaultError::OverflowError)?;
    } else {
        // Existing user - add to total deposited
        user_position.total_deposited = user_position
            .total_deposited
            .checked_add(amount)
            .ok_or(VaultError::OverflowError)?;
    }

    user_position.shares = user_position
        .shares
        .checked_add(shares)
        .ok_or(VaultError::OverflowError)?;
    user_position.bump = ctx.bumps.user_position;

    Ok(())
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    /// Vault account
    #[account(mut)]
    pub vault: Account<'info, Vault>,

    /// User's wallet
    #[account(mut)]
    pub user: Signer<'info>,

    /// Vault SOL account (PDA that holds funds)
    /// In this MVP, we use a simple account. Production would use token accounts.
    #[account(mut)]
    pub vault_account: AccountInfo<'info>,

    /// User position tracking
    #[account(
        init_if_needed,
        payer = user,
        space = UserPosition::LEN,
        seeds = [b"user_position", vault.key().as_ref(), user.key().as_ref()],
        bump
    )]
    pub user_position: Account<'info, UserPosition>,

    /// System program
    pub system_program: Program<'info, System>,
}
