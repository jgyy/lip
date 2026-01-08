use anchor_lang::prelude::*;

/// Strategy configuration and state
#[account]
pub struct StrategyState {
    /// Associated vault
    pub vault: Pubkey,
    /// Current best opportunity
    pub best_opportunity: u8,
    /// Number of opportunities evaluated
    pub num_opportunities: u8,
    /// Threshold for rebalancing (score difference)
    pub rebalance_threshold: u16,
    /// Last rebalance timestamp
    pub last_rebalance: i64,
    /// Total value currently deployed in strategies
    pub deployed_value: u64,
    /// Bump seed
    pub bump: u8,
}

impl StrategyState {
    pub const LEN: usize = 8 + 32 + 1 + 1 + 2 + 8 + 8 + 1;
}

/// Yield opportunity from a protocol
#[account]
pub struct Opportunity {
    /// Protocol identifier (e.g., "Meteora_SOL_USDC", "Kamino_SOL")
    pub protocol_id: [u8; 32],
    /// Current APY (as percentage * 100, e.g., 1050 = 10.50%)
    pub apy: u16,
    /// Volatility score (0-100, where 100 is most volatile)
    pub volatility: u8,
    /// Impermanent loss risk (0-100)
    pub il_risk: u8,
    /// Protocol safety score (0-100, where 100 is safest)
    pub safety_score: u8,
    /// Calculated risk-adjusted score
    pub score: u16,
    /// Is this opportunity active
    pub active: bool,
    /// Last update timestamp
    pub last_updated: i64,
    /// Bump seed
    pub bump: u8,
}

impl Opportunity {
    pub const LEN: usize = 8 + 32 + 2 + 1 + 1 + 1 + 2 + 1 + 8 + 1;
}

/// Represents a deployed position in a protocol
#[account]
pub struct DeployedPosition {
    /// Which opportunity this is deployed to
    pub opportunity: Pubkey,
    /// Amount deployed (in lamports)
    pub amount: u64,
    /// Timestamp of deployment
    pub deployment_timestamp: i64,
    /// Accumulated yield from this position
    pub yield_earned: u64,
    /// Bump seed
    pub bump: u8,
}

impl DeployedPosition {
    pub const LEN: usize = 8 + 32 + 8 + 8 + 8 + 1;
}
