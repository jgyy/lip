use anchor_lang::prelude::*;

pub mod state;
pub mod errors;

pub use state::*;
pub use errors::*;

declare_id!("BE2bDcazEUS5YpSW2JgJ2CAeTErp8LrpkZWAuW8WXa4h");

#[program]
pub mod user_management {
    use super::*;

    /// Initialize role authority for a vault
    pub fn initialize_role_authority(ctx: Context<InitializeRoleAuthority>) -> Result<()> {
        if ctx.accounts.role_authority.initialized {
            return Err(RbacError::AlreadyInitialized.into());
        }

        let vault_key = ctx.accounts.vault.key();
        let admin_key = ctx.accounts.admin.key();

        let role_authority = &mut ctx.accounts.role_authority;
        role_authority.vault = vault_key;
        role_authority.super_admin = admin_key;
        role_authority.initialized = true;
        role_authority.emergency_pause = false;
        role_authority.bump = ctx.bumps.role_authority;

        let user_role = &mut ctx.accounts.user_role;
        user_role.user = admin_key;
        user_role.vault = vault_key;
        user_role.roles = ROLE_ADMIN;
        user_role.assigned_at = Clock::get()?.unix_timestamp;
        user_role.assigned_by = admin_key;
        user_role.bump = ctx.bumps.user_role;

        Ok(())
    }

    /// Assign one or more roles to a user
    pub fn assign_role(
        ctx: Context<AssignRole>,
        target_user: Pubkey,
        role: u8,
    ) -> Result<()> {
        if ctx.accounts.role_authority.emergency_pause {
            return Err(RbacError::EmergencyPaused.into());
        }

        if !ctx.accounts.role_authority.initialized {
            return Err(RbacError::NotInitialized.into());
        }

        if !ctx.accounts.admin_role.has_role(ROLE_ADMIN) {
            return Err(RbacError::Unauthorized.into());
        }

        if role == 0 || role > 15 {
            return Err(RbacError::InvalidRole.into());
        }

        let user_role = &mut ctx.accounts.target_user_role;

        if user_role.user == Pubkey::default() {
            user_role.user = target_user;
            user_role.vault = ctx.accounts.role_authority.vault;
            user_role.roles = role;
            user_role.bump = ctx.bumps.target_user_role;
        } else {
            user_role.add_role(role);
        }

        user_role.assigned_at = Clock::get()?.unix_timestamp;
        user_role.assigned_by = ctx.accounts.admin.key();

        Ok(())
    }

    /// Revoke a role from a user
    pub fn revoke_role(
        ctx: Context<RevokeRole>,
        target_user: Pubkey,
        role: u8,
    ) -> Result<()> {
        if ctx.accounts.role_authority.emergency_pause {
            return Err(RbacError::EmergencyPaused.into());
        }

        if !ctx.accounts.role_authority.initialized {
            return Err(RbacError::NotInitialized.into());
        }

        if !ctx.accounts.admin_role.has_role(ROLE_ADMIN) {
            return Err(RbacError::Unauthorized.into());
        }

        let role_authority = &ctx.accounts.role_authority;
        let target_role = &mut ctx.accounts.target_user_role;

        if target_user == role_authority.super_admin && (role & ROLE_ADMIN) != 0 {
            return Err(RbacError::CannotRevokeSuperAdmin.into());
        }

        target_role.remove_role(role);

        if (role & ROLE_ADMIN) != 0 {
            if role_authority.super_admin == target_user && !target_role.has_role(ROLE_ADMIN) {
                return Err(RbacError::MustHaveAdmin.into());
            }
        }

        target_role.assigned_at = Clock::get()?.unix_timestamp;
        target_role.assigned_by = ctx.accounts.admin.key();

        Ok(())
    }

    /// Check if a user has a specific role (view function)
    pub fn has_role(ctx: Context<HasRole>, role: u8) -> Result<bool> {
        if ctx.accounts.role_authority.emergency_pause {
            return Ok(false);
        }

        if !ctx.accounts.role_authority.initialized {
            return Err(RbacError::NotInitialized.into());
        }

        Ok(ctx.accounts.user_role.has_role(role))
    }

    /// Toggle emergency pause (super admin only)
    pub fn emergency_pause(ctx: Context<EmergencyPause>) -> Result<()> {
        let role_authority = &mut ctx.accounts.role_authority;
        let super_admin = role_authority.super_admin;

        if ctx.accounts.caller.key() != super_admin {
            return Err(RbacError::Unauthorized.into());
        }

        if !role_authority.initialized {
            return Err(RbacError::NotInitialized.into());
        }

        role_authority.emergency_pause = !role_authority.emergency_pause;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeRoleAuthority<'info> {
    /// CHECK: Vault address is only used to derive PDAs, no validation needed
    pub vault: UncheckedAccount<'info>,

    #[account(
        init,
        payer = admin,
        space = RoleAuthority::LEN,
        seeds = [b"role_authority", vault.key().as_ref()],
        bump
    )]
    pub role_authority: Account<'info, RoleAuthority>,

    #[account(
        init_if_needed,
        payer = admin,
        space = UserRole::LEN,
        seeds = [b"user_role", vault.key().as_ref(), admin.key().as_ref()],
        bump
    )]
    pub user_role: Account<'info, UserRole>,

    #[account(mut)]
    pub admin: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(target_user: Pubkey)]
pub struct AssignRole<'info> {
    #[account(mut)]
    pub role_authority: Account<'info, RoleAuthority>,

    pub admin_role: Account<'info, UserRole>,

    #[account(
        init_if_needed,
        payer = admin,
        space = UserRole::LEN,
        seeds = [b"user_role", role_authority.vault.as_ref(), target_user.as_ref()],
        bump
    )]
    pub target_user_role: Account<'info, UserRole>,

    #[account(mut)]
    pub admin: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(target_user: Pubkey)]
pub struct RevokeRole<'info> {
    #[account(mut)]
    pub role_authority: Account<'info, RoleAuthority>,

    pub admin_role: Account<'info, UserRole>,

    #[account(
        mut,
        seeds = [b"user_role", role_authority.vault.as_ref(), target_user.as_ref()],
        bump = target_user_role.bump
    )]
    pub target_user_role: Account<'info, UserRole>,

    #[account(mut)]
    pub admin: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct HasRole<'info> {
    pub role_authority: Account<'info, RoleAuthority>,

    pub user_role: Account<'info, UserRole>,
}

#[derive(Accounts)]
pub struct EmergencyPause<'info> {
    #[account(mut)]
    pub role_authority: Account<'info, RoleAuthority>,

    pub caller: Signer<'info>,
}
