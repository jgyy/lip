use anchor_lang::prelude::*;
use crate::state::{Opportunity, StrategyState};
use crate::scoring::ScoringEngine;
use crate::errors::StrategyError;

pub fn register_opportunity(
    ctx: Context<RegisterOpportunity>,
    protocol_id: [u8; 32],
    apy: u16,
    volatility: u8,
    il_risk: u8,
    safety_score: u8,
) -> Result<()> {
    // Validate inputs
    if volatility > 100 || il_risk > 100 || safety_score > 100 {
        return Err(StrategyError::InvalidOpportunity.into());
    }

    // Calculate risk-adjusted score
    let score = ScoringEngine::calculate_score(apy, volatility, il_risk, safety_score);

    let opportunity = &mut ctx.accounts.opportunity;
    opportunity.protocol_id = protocol_id;
    opportunity.apy = apy;
    opportunity.volatility = volatility;
    opportunity.il_risk = il_risk;
    opportunity.safety_score = safety_score;
    opportunity.score = score;
    opportunity.active = true;
    opportunity.last_updated = Clock::get()?.unix_timestamp;
    opportunity.bump = ctx.bumps.opportunity;

    // Update strategy state
    let strategy = &mut ctx.accounts.strategy_state;
    if strategy.num_opportunities < 255 {
        strategy.num_opportunities = strategy.num_opportunities.checked_add(1)
            .ok_or(StrategyError::OverflowError)?;
    }

    // Update best opportunity if this is better
    if strategy.best_opportunity == 0 || score > ctx.accounts.best_opp_account.score {
        strategy.best_opportunity = strategy.num_opportunities - 1;
    }

    Ok(())
}

#[derive(Accounts)]
pub struct RegisterOpportunity<'info> {
    /// Strategy state
    #[account(mut)]
    pub strategy_state: Account<'info, StrategyState>,

    /// New opportunity being registered
    #[account(init, payer = admin, space = Opportunity::LEN)]
    pub opportunity: Account<'info, Opportunity>,

    /// Reference to best opportunity (for comparison)
    #[account(mut)]
    pub best_opp_account: Account<'info, Opportunity>,

    /// Admin signer
    #[account(mut)]
    pub admin: Signer<'info>,

    /// System program
    pub system_program: Program<'info, System>,
}
