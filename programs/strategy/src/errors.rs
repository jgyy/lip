use anchor_lang::prelude::*;

#[error_code]
pub enum StrategyError {
    #[msg("Unauthorized - only admin can perform this action")]
    Unauthorized,

    #[msg("Invalid opportunity data")]
    InvalidOpportunity,

    #[msg("No opportunities available")]
    NoOpportunities,

    #[msg("Opportunity score too low")]
    ScoreTooLow,

    #[msg("Invalid rebalance threshold")]
    InvalidThreshold,

    #[msg("Rebalance cooldown not satisfied")]
    RebalanceCooldown,

    #[msg("Insufficient deployed value")]
    InsufficientDeployed,

    #[msg("Invalid deployment amount")]
    InvalidAmount,

    #[msg("Overflow in calculation")]
    OverflowError,

    #[msg("No yield to harvest")]
    NoYield,
}
