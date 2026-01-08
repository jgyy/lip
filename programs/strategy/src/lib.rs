use anchor_lang::prelude::*;

mod errors;
mod scoring;
mod state;
mod integrations;

use errors::StrategyError;
use state::{StrategyState, Opportunity};
use scoring::ScoringEngine;

declare_id!("4Xwd8CQkmhKzDYc8d1uEZd4iG3q1Wq3oCqgZVJrmmP9U");

#[program]
pub mod strategy {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, rebalance_threshold: u16) -> Result<()> {
        let strategy_state = &mut ctx.accounts.strategy_state;
        strategy_state.vault = ctx.accounts.vault.key();
        strategy_state.best_opportunity = 0;
        strategy_state.num_opportunities = 0;
        strategy_state.rebalance_threshold = rebalance_threshold;
        strategy_state.last_rebalance = Clock::get()?.unix_timestamp;
        strategy_state.deployed_value = 0;
        Ok(())
    }

    pub fn register_opportunity(
        ctx: Context<RegisterOpportunity>,
        protocol_id: [u8; 32],
        apy: u16,
        volatility: u8,
        il_risk: u8,
        safety_score: u8,
    ) -> Result<()> {
        if volatility > 100 || il_risk > 100 || safety_score > 100 {
            return Err(StrategyError::InvalidOpportunity.into());
        }

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

        let strategy = &mut ctx.accounts.strategy_state;
        if strategy.num_opportunities < 255 {
            strategy.num_opportunities = strategy.num_opportunities.checked_add(1)
                .ok_or(StrategyError::OverflowError)?;
        }

        Ok(())
    }

    pub fn evaluate(
        ctx: Context<Evaluate>,
        new_apy: u16,
        new_volatility: u8,
        new_il_risk: u8,
        new_safety: u8,
    ) -> Result<()> {
        if new_volatility > 100 || new_il_risk > 100 || new_safety > 100 {
            return Err(StrategyError::InvalidOpportunity.into());
        }

        let opportunity = &mut ctx.accounts.opportunity;

        let new_score = ScoringEngine::calculate_score(
            new_apy,
            new_volatility,
            new_il_risk,
            new_safety,
        );

        opportunity.apy = new_apy;
        opportunity.volatility = new_volatility;
        opportunity.il_risk = new_il_risk;
        opportunity.safety_score = new_safety;
        opportunity.score = new_score;
        opportunity.last_updated = Clock::get()?.unix_timestamp;

        Ok(())
    }

    pub fn rebalance(ctx: Context<Rebalance>) -> Result<()> {
        const REBALANCE_COOLDOWN: i64 = 3600;

        let strategy = &mut ctx.accounts.strategy_state;
        let current_time = Clock::get()?.unix_timestamp;

        let time_since_last = current_time.checked_sub(strategy.last_rebalance)
            .ok_or(StrategyError::OverflowError)?;

        if time_since_last < REBALANCE_COOLDOWN {
            return Err(StrategyError::RebalanceCooldown.into());
        }

        strategy.last_rebalance = current_time;

        msg!("Rebalancing executed successfully");

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = admin, space = StrategyState::LEN)]
    pub strategy_state: Account<'info, StrategyState>,

    pub vault: AccountInfo<'info>,

    #[account(mut)]
    pub admin: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RegisterOpportunity<'info> {
    #[account(mut)]
    pub strategy_state: Account<'info, StrategyState>,

    #[account(init, payer = admin, space = Opportunity::LEN)]
    pub opportunity: Account<'info, Opportunity>,

    #[account(mut)]
    pub admin: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Evaluate<'info> {
    #[account(mut)]
    pub strategy_state: Account<'info, StrategyState>,

    #[account(mut)]
    pub opportunity: Account<'info, Opportunity>,

    pub admin: Signer<'info>,
}

#[derive(Accounts)]
pub struct Rebalance<'info> {
    #[account(mut)]
    pub strategy_state: Account<'info, StrategyState>,

    pub admin: Signer<'info>,
}
