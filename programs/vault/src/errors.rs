use anchor_lang::prelude::*;

#[error_code]
pub enum VaultError {
    #[msg("Unauthorized - only admin can perform this action")]
    Unauthorized,

    #[msg("Invalid amount - must be greater than zero")]
    InvalidAmount,

    #[msg("Insufficient balance")]
    InsufficientBalance,

    #[msg("Invalid strategy allocation")]
    InvalidAllocation,

    #[msg("Vault is empty")]
    VaultEmpty,

    #[msg("Time lock not satisfied - must wait before withdrawing")]
    TimeLockActive,

    #[msg("Invalid shares amount")]
    InvalidShares,

    #[msg("Overflow in calculation")]
    OverflowError,

    #[msg("No yield available to harvest")]
    NoYield,

    #[msg("Invalid strategy opportunity")]
    InvalidStrategy,

    #[msg("RBAC: Invalid role value")]
    InvalidRole,

    #[msg("RBAC: System is under emergency pause")]
    EmergencyPaused,

    #[msg("RBAC: Role authority not found")]
    RoleAuthorityNotFound,
}
