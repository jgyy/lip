use anchor_lang::prelude::*;

/// Role bitfield constants
pub const ROLE_REGULAR_USER: u8 = 1 << 0; // bit 0
pub const ROLE_ADMIN: u8 = 1 << 1;       // bit 1
pub const ROLE_STRATEGY_MANAGER: u8 = 1 << 2; // bit 2
pub const ROLE_TREASURY: u8 = 1 << 3;   // bit 3

/// User's role assignment for a specific vault
#[account]
pub struct UserRole {
    /// The user's wallet address
    pub user: Pubkey,
    /// Which vault this role applies to
    pub vault: Pubkey,
    /// Bitfield for multiple roles
    /// bit 0: RegularUser (can deposit/withdraw)
    /// bit 1: Admin (can initialize, harvest, manage roles)
    /// bit 2: StrategyManager (can register opportunities, rebalance)
    /// bit 3: Treasury (can withdraw fees)
    pub roles: u8,
    /// Timestamp when role was assigned
    pub assigned_at: i64,
    /// Admin who assigned this role
    pub assigned_by: Pubkey,
    /// PDA bump seed
    pub bump: u8,
}

impl UserRole {
    pub const LEN: usize = 8 + 32 + 32 + 1 + 8 + 32 + 1;

    /// Check if user has a specific role
    pub fn has_role(&self, role: u8) -> bool {
        (self.roles & role) != 0
    }

    /// Add a role to the user (via bitfield OR)
    pub fn add_role(&mut self, role: u8) {
        self.roles |= role;
    }

    /// Remove a role from the user (via bitfield AND with negation)
    pub fn remove_role(&mut self, role: u8) {
        self.roles &= !role;
    }
}

/// Role authority for a vault - manages role assignment
#[account]
pub struct RoleAuthority {
    /// Associated vault
    pub vault: Pubkey,
    /// Super admin who can never be revoked
    pub super_admin: Pubkey,
    /// Whether role authority has been initialized
    pub initialized: bool,
    /// Emergency pause flag
    pub emergency_pause: bool,
    /// PDA bump seed
    pub bump: u8,
}

impl RoleAuthority {
    pub const LEN: usize = 8 + 32 + 32 + 1 + 1 + 1;
}
