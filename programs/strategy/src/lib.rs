use anchor_lang::prelude::*;

mod errors;
mod scoring;
mod state;
mod integrations;
mod rbac_helper;
mod instructions;

use errors::StrategyError;
use state::{StrategyState, Opportunity};
use scoring::ScoringEngine;
use instructions::*;

declare_id!("EUWvahvmdyPRgmwcFuqJdJ4FX8S2syWGc8XdTdafshZ2");

#[program]
pub mod strategy {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, rebalance_threshold: u16) -> Result<()> {
        instructions::initialize(ctx, rebalance_threshold)
    }

    pub fn register_opportunity(
        ctx: Context<RegisterOpportunity>,
        protocol_id: [u8; 32],
        apy: u16,
        volatility: u8,
        il_risk: u8,
        safety_score: u8,
    ) -> Result<()> {
        instructions::register_opportunity(ctx, protocol_id, apy, volatility, il_risk, safety_score)
    }

    pub fn evaluate(
        ctx: Context<Evaluate>,
        new_apy: u16,
        new_volatility: u8,
        new_il_risk: u8,
        new_safety: u8,
    ) -> Result<()> {
        instructions::evaluate(ctx, new_apy, new_volatility, new_il_risk, new_safety)
    }

    pub fn rebalance(ctx: Context<Rebalance>) -> Result<()> {
        instructions::rebalance(ctx)
    }
}
