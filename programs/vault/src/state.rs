use anchor_lang::prelude::*;

/// Vault account - stores vault-level state and configuration
#[account]
pub struct Vault {
    /// Total SOL deposited in the vault (in lamports)
    pub total_assets: u64,
    /// Total shares outstanding
    pub total_shares: u64,
    /// Vault administrator/owner
    pub admin: Pubkey,
    /// Current strategy allocation (0-100%)
    pub strategy_allocation: u8,
    /// Total yield accrued (in lamports)
    pub total_yield: u64,
    /// Accumulated performance fees (in lamports)
    pub accumulated_fees: u64,
    /// Number of users deposited
    pub num_users: u64,
    /// Vault bump seed
    pub bump: u8,
}

impl Vault {
    pub const LEN: usize = 8 + 8 + 8 + 32 + 1 + 8 + 8 + 8 + 1;

    /// Calculate the share price (assets per share in lamports)
    pub fn share_price(&self) -> u64 {
        if self.total_shares == 0 {
            1 // 1 lamport per share when vault is empty
        } else {
            self.total_assets / self.total_shares
        }
    }

    /// Calculate shares to mint for a deposit amount
    pub fn shares_for_deposit(&self, amount: u64) -> u64 {
        if self.total_shares == 0 {
            amount // 1:1 ratio on first deposit
        } else {
            (amount * self.total_shares) / self.total_assets
        }
    }

    /// Calculate asset amount for a given share quantity
    pub fn assets_for_shares(&self, shares: u64) -> u64 {
        (shares * self.total_assets) / self.total_shares
    }
}

/// User position in the vault
#[account]
pub struct UserPosition {
    /// User's wallet address
    pub user: Pubkey,
    /// Vault this position belongs to
    pub vault: Pubkey,
    /// Number of shares owned
    pub shares: u64,
    /// Deposit timestamp (for time-weighted exit)
    pub deposit_timestamp: i64,
    /// Total amount deposited (for tracking)
    pub total_deposited: u64,
    /// User position bump seed
    pub bump: u8,
}

impl UserPosition {
    pub const LEN: usize = 8 + 32 + 32 + 8 + 8 + 8 + 1;
}

/// Strategy state - tracks yield opportunities
#[account]
pub struct StrategyOpportunity {
    /// Name of the protocol (e.g., "Meteora AMM", "Kamino Lending")
    pub protocol_name: [u8; 32],
    /// Current APY (as percentage * 100, e.g., 1050 = 10.50%)
    pub apy: u16,
    /// Volatility score (0-100)
    pub volatility: u8,
    /// Impermanent loss risk (0-100)
    pub il_risk: u8,
    /// Protocol safety score (0-100)
    pub safety_score: u8,
    /// Risk-adjusted score
    pub score: u16,
    /// Is this opportunity currently active
    pub active: bool,
    /// Bump seed
    pub bump: u8,
}

impl StrategyOpportunity {
    pub const LEN: usize = 8 + 32 + 2 + 1 + 1 + 1 + 2 + 1 + 1;
}
