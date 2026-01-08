use anchor_lang::prelude::*;
use anchor_lang::system_program;

mod errors;
mod state;
mod rbac_helper;
mod instructions;

use errors::VaultError;
use state::{Vault, UserPosition};
use instructions::*;

declare_id!("76MQ83iPkH4ERPWq8cDKwo7KaQrSpkdwF6qdTbHi7Q7j");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize(ctx)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        instructions::deposit(ctx, amount)
    }

    pub fn withdraw(ctx: Context<Withdraw>, shares: u64) -> Result<()> {
        instructions::withdraw(ctx, shares)
    }

    pub fn harvest(ctx: Context<Harvest>, yield_amount: u64) -> Result<()> {
        instructions::harvest(ctx, yield_amount)
    }

    pub fn withdraw_fees(ctx: Context<WithdrawFees>) -> Result<()> {
        instructions::withdraw_fees(ctx)
    }

    pub fn update_settings(
        ctx: Context<UpdateSettings>,
        strategy_allocation: Option<u8>,
    ) -> Result<()> {
        instructions::update_settings(ctx, strategy_allocation)
    }
}

