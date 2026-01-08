use anchor_lang::prelude::*;
use crate::state::StrategyState;

pub fn initialize(ctx: Context<Initialize>, rebalance_threshold: u16) -> Result<()> {
    let strategy_state = &mut ctx.accounts.strategy_state;

    strategy_state.vault = ctx.accounts.vault.key();
    strategy_state.best_opportunity = 0;
    strategy_state.num_opportunities = 0;
    strategy_state.rebalance_threshold = rebalance_threshold;
    strategy_state.last_rebalance = Clock::get()?.unix_timestamp;
    strategy_state.deployed_value = 0;
    strategy_state.bump = ctx.bumps.strategy_state;

    Ok(())
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    /// Strategy state to initialize
    #[account(init, payer = admin, space = StrategyState::LEN)]
    pub strategy_state: Account<'info, StrategyState>,

    /// Vault this strategy serves
    pub vault: AccountInfo<'info>,

    /// Admin signer
    #[account(mut)]
    pub admin: Signer<'info>,

    /// System program
    pub system_program: Program<'info, System>,
}
