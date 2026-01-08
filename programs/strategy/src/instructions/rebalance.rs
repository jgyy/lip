use anchor_lang::prelude::*;
use crate::state::StrategyState;
use crate::scoring::ScoringEngine;
use crate::errors::StrategyError;

const REBALANCE_COOLDOWN: i64 = 3600; // 1 hour minimum between rebalances

pub fn rebalance(ctx: Context<Rebalance>) -> Result<()> {
    let strategy = &mut ctx.accounts.strategy_state;
    let current_time = Clock::get()?.unix_timestamp;

    // Check cooldown
    let time_since_last = current_time.checked_sub(strategy.last_rebalance)
        .ok_or(StrategyError::OverflowError)?;

    if time_since_last < REBALANCE_COOLDOWN {
        return Err(StrategyError::RebalanceCooldown.into());
    }

    // Check if rebalancing is needed
    let current_score = ctx.accounts.current_best.score;
    let best_score = ctx.accounts.best_opportunity.score;
    let should_rebalance = ScoringEngine::should_rebalance(
        current_score,
        best_score,
        strategy.rebalance_threshold,
    );

    if !should_rebalance {
        return Err(StrategyError::ScoreTooLow.into());
    }

    // Update strategy state
    strategy.last_rebalance = current_time;

    // Note: Actual fund movement would be done via CPI calls to vault/protocol programs
    // For MVP, we just emit the rebalancing decision

    msg!("Rebalancing triggered: Moving from score {} to score {}", current_score, best_score);

    Ok(())
}

#[derive(Accounts)]
pub struct Rebalance<'info> {
    /// Strategy state
    #[account(mut)]
    pub strategy_state: Account<'info, StrategyState>,

    /// Current best opportunity
    pub current_best: AccountInfo<'info>,

    /// New best opportunity
    pub best_opportunity: AccountInfo<'info>,

    /// Admin signer
    pub admin: Signer<'info>,
}
