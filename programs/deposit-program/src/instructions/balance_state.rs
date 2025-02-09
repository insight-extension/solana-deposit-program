use anchor_lang::prelude::*;

use crate::{
    constants::{MASTER_WALLET, USER_INFO_SEED},
    error::ErrorCode,
    UserInfo,
};

#[derive(Accounts)]
pub struct BalanceState<'info> {
    #[account(mut, address = MASTER_WALLET)]
    pub master: Signer<'info>,

    #[account(mut)]
    pub user: SystemAccount<'info>,

    #[account(
        mut,
        seeds = [USER_INFO_SEED, user.key().as_ref()],
        bump = user_info.bump
    )]
    pub user_info: Account<'info, UserInfo>,

    pub system_program: Program<'info, System>,
}

pub fn freeze_balance(ctx: Context<BalanceState>) -> Result<()> {
    let balance_status = ctx.accounts.user_info.is_balance_frozen;

    if balance_status {
        return Err(ErrorCode::BalanceAlreadyFrozen.into());
    }

    ctx.accounts.user_info.is_balance_frozen = true;

    Ok(())
}

pub fn unfreeze_balance(ctx: Context<BalanceState>) -> Result<()> {
    let balance_status = ctx.accounts.user_info.is_balance_frozen;

    if !balance_status {
        return Err(ErrorCode::BalanceNotFrozen.into());
    }

    ctx.accounts.user_info.is_balance_frozen = false;

    Ok(())
}
