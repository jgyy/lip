use anchor_lang::prelude::*;
use anchor_lang::system_program;

mod errors;
mod state;

use errors::VaultError;
use state::{Vault, UserPosition};

declare_id!("2nMr2an62tQzRbqBHuVn9oPxgkZfTfr5Sku2dKejU7Lu");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.total_assets = 0;
        vault.total_shares = 0;
        vault.admin = ctx.accounts.admin.key();
        vault.strategy_allocation = 50;
        vault.total_yield = 0;
        vault.accumulated_fees = 0;
        vault.num_users = 0;
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        if amount == 0 {
            return Err(VaultError::InvalidAmount.into());
        }

        let vault = &mut ctx.accounts.vault;
        let shares = vault.shares_for_deposit(amount);

        if shares == 0 {
            return Err(VaultError::InvalidShares.into());
        }

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

        vault.total_assets = vault.total_assets.checked_add(amount)
            .ok_or(VaultError::OverflowError)?;
        vault.total_shares = vault.total_shares.checked_add(shares)
            .ok_or(VaultError::OverflowError)?;

        let user_position = &mut ctx.accounts.user_position;
        if user_position.shares == 0 {
            user_position.user = ctx.accounts.user.key();
            user_position.vault = vault.key();
            user_position.deposit_timestamp = Clock::get()?.unix_timestamp;
            user_position.total_deposited = amount;
            vault.num_users = vault.num_users.checked_add(1)
                .ok_or(VaultError::OverflowError)?;
        } else {
            user_position.total_deposited = user_position.total_deposited.checked_add(amount)
                .ok_or(VaultError::OverflowError)?;
        }

        user_position.shares = user_position.shares.checked_add(shares)
            .ok_or(VaultError::OverflowError)?;

        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, shares: u64) -> Result<()> {
        if shares == 0 {
            return Err(VaultError::InvalidShares.into());
        }

        const TIME_LOCK_DURATION: i64 = 24 * 60 * 60;

        let vault = &mut ctx.accounts.vault;
        let user_position = &mut ctx.accounts.user_position;

        let current_time = Clock::get()?.unix_timestamp;
        let time_since_deposit = current_time.checked_sub(user_position.deposit_timestamp)
            .ok_or(VaultError::OverflowError)?;

        if time_since_deposit < TIME_LOCK_DURATION {
            return Err(VaultError::TimeLockActive.into());
        }

        if shares > user_position.shares {
            return Err(VaultError::InsufficientBalance.into());
        }

        let assets = vault.assets_for_shares(shares);
        if assets == 0 {
            return Err(VaultError::InvalidAmount.into());
        }

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

        vault.total_assets = vault.total_assets.checked_sub(assets)
            .ok_or(VaultError::OverflowError)?;
        vault.total_shares = vault.total_shares.checked_sub(shares)
            .ok_or(VaultError::OverflowError)?;

        user_position.shares = user_position.shares.checked_sub(shares)
            .ok_or(VaultError::OverflowError)?;

        Ok(())
    }

    pub fn harvest(ctx: Context<Harvest>, yield_amount: u64) -> Result<()> {
        if yield_amount == 0 {
            return Err(VaultError::NoYield.into());
        }

        require_keys_eq!(ctx.accounts.vault.admin, ctx.accounts.admin.key(), VaultError::Unauthorized);

        let vault = &mut ctx.accounts.vault;
        let fee = yield_amount / 10;
        let net_yield = yield_amount.checked_sub(fee)
            .ok_or(VaultError::OverflowError)?;

        vault.total_yield = vault.total_yield.checked_add(net_yield)
            .ok_or(VaultError::OverflowError)?;
        vault.accumulated_fees = vault.accumulated_fees.checked_add(fee)
            .ok_or(VaultError::OverflowError)?;
        vault.total_assets = vault.total_assets.checked_add(net_yield)
            .ok_or(VaultError::OverflowError)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = admin, space = Vault::LEN)]
    pub vault: Account<'info, Vault>,

    #[account(mut)]
    pub admin: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub vault_account: AccountInfo<'info>,

    #[account(mut)]
    pub user_position: Account<'info, UserPosition>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub vault_account: AccountInfo<'info>,

    #[account(mut)]
    pub user_position: Account<'info, UserPosition>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Harvest<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,

    pub admin: Signer<'info>,
}
