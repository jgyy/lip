use anchor_lang::prelude::*;
use crate::state::{Opportunity, StrategyState};
use crate::scoring::ScoringEngine;

pub fn evaluate(
    ctx: Context<Evaluate>,
    new_apy: u16,
    new_volatility: u8,
    new_il_risk: u8,
    new_safety: u8,
) -> Result<()> {
    let opportunity = &mut ctx.accounts.opportunity;
    let old_score = opportunity.score;

    // Update opportunity data
    opportunity.apy = new_apy;
    opportunity.volatility = new_volatility;
    opportunity.il_risk = new_il_risk;
    opportunity.safety_score = new_safety;

    // Recalculate score
    let new_score = ScoringEngine::calculate_score(
        new_apy,
        new_volatility,
        new_il_risk,
        new_safety,
    );
    opportunity.score = new_score;
    opportunity.last_updated = Clock::get()?.unix_timestamp;

    // Update strategy state if this is now the best
    let strategy = &mut ctx.accounts.strategy_state;
    if new_score > ctx.accounts.current_best.score {
        strategy.best_opportunity = ctx.accounts.opportunity_index;
    }

    Ok(())
}

#[derive(Accounts)]
pub struct Evaluate<'info> {
    /// Strategy state
    #[account(mut)]
    pub strategy_state: Account<'info, StrategyState>,

    /// Opportunity to evaluate
    #[account(mut)]
    pub opportunity: Account<'info, Opportunity>,

    /// Current best opportunity (for comparison)
    pub current_best: Account<'info, Opportunity>,

    /// Index of the opportunity being evaluated
    pub opportunity_index: u8,

    /// Admin signer
    pub admin: Signer<'info>,
}
