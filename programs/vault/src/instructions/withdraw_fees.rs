use anchor_lang::prelude::*;
use anchor_lang::system_program;
use crate::state::Vault;
use crate::errors::VaultError;

/// Withdraw accumulated performance fees (Treasury role)
pub fn withdraw_fees(ctx: Context<WithdrawFees>) -> Result<()> {
    let vault = &mut ctx.accounts.vault;

    // Check if there are any fees to withdraw
    if vault.accumulated_fees == 0 {
        return Err(VaultError::InvalidAmount.into());
    }

    let fee_amount = vault.accumulated_fees;

    // Transfer fees to treasury
    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.vault_account.to_account_info(),
                to: ctx.accounts.treasury.to_account_info(),
            },
        ),
        fee_amount,
    )?;

    // Reset accumulated fees
    vault.accumulated_fees = 0;

    Ok(())
}

#[derive(Accounts)]
pub struct WithdrawFees<'info> {
    /// Vault account
    #[account(mut)]
    pub vault: Account<'info, Vault>,

    /// Vault SOL account (PDA that holds funds)
    #[account(mut)]
    pub vault_account: AccountInfo<'info>,

    /// Treasury wallet that receives the fees
    #[account(mut)]
    pub treasury: Signer<'info>,

    /// System program
    pub system_program: Program<'info, System>,
}
