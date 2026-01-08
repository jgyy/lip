use anchor_lang::prelude::*;

#[error_code]
pub enum RbacError {
    #[msg("Unauthorized - caller does not have required role")]
    Unauthorized,

    #[msg("Invalid role value")]
    InvalidRole,

    #[msg("Cannot revoke super admin role")]
    CannotRevokeSuperAdmin,

    #[msg("Must maintain at least one admin")]
    MustHaveAdmin,

    #[msg("Role authority already initialized")]
    AlreadyInitialized,

    #[msg("Role authority not initialized")]
    NotInitialized,

    #[msg("System is under emergency pause")]
    EmergencyPaused,

    #[msg("Invalid role combination")]
    InvalidRoleCombination,
}
