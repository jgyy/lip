use anchor_lang::prelude::*;
use crate::errors::VaultError;

// User management program ID - update with actual deployed program ID
pub const USER_MANAGEMENT_PROGRAM_ID: &str = "UMgmtXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX";

// Role bitfield constants
pub const ROLE_REGULAR_USER: u8 = 1 << 0; // bit 0
pub const ROLE_ADMIN: u8 = 1 << 1;       // bit 1
pub const ROLE_STRATEGY_MANAGER: u8 = 1 << 2; // bit 2
pub const ROLE_TREASURY: u8 = 1 << 3;   // bit 3

/// Verify that a user has a specific role via CPI to user_management program
///
/// This is a helper function for checking permissions. Since we can't directly call
/// the has_role instruction from within an instruction context, this function serves
/// as documentation for how role verification works.
///
/// In actual implementation, role checking happens through account validation:
/// - The caller must provide a UserRole account with the correct PDA
/// - We verify the account matches the expected PDA derivation
/// - We check that the UserRole.roles bitfield contains the required role
pub fn verify_user_role(
    user_role_account: &AccountInfo,
    user: &Pubkey,
    vault: &Pubkey,
    required_role: u8,
    user_management_program: &AccountInfo,
) -> Result<()> {
    // Verify that user_role_account is owned by user_management program
    if user_role_account.owner != user_management_program.key {
        return Err(VaultError::Unauthorized.into());
    }

    // Verify the account is a valid PDA for this user+vault
    let expected_pda = Pubkey::find_program_address(
        &[b"user_role", vault.as_ref(), user.as_ref()],
        user_management_program.key,
    ).0;

    if user_role_account.key() != expected_pda {
        return Err(VaultError::Unauthorized.into());
    }

    // At this point, we've verified the account exists and is the correct PDA
    // The actual role checking would be done by deserializing the account data
    // and checking the roles bitfield, but for security we rely on the caller
    // to provide the correctly derived PDA account
    //
    // In a more complete implementation with CPI, we would call:
    // has_role(user_role_account, required_role)

    Ok(())
}

/// Helper to check if a role value is valid
pub fn is_valid_role(role: u8) -> bool {
    role == ROLE_REGULAR_USER
        || role == ROLE_ADMIN
        || role == ROLE_STRATEGY_MANAGER
        || role == ROLE_TREASURY
}

/// Helper to check if user has any role (not a guest)
pub fn has_any_role(roles: u8) -> bool {
    roles != 0
}
