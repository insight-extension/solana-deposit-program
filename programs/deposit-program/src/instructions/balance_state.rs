use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

use crate::{
    constants::{MASTER_WALLET, USDC_MINT, USER_TIMED_INFO_SEED},
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
        seeds = [b"user_info", user.key().as_ref()],
        bump = user_timed_info.bump
    )]
    pub user_timed_info: Account<'info, UserInfo>,

    pub system_program: Program<'info, System>,
}

pub fn freeze_balance(ctx: Context<BalanceState>) -> Result<()> {
    let balance_status = ctx.accounts.user_timed_info.is_balance_frozen;

    if balance_status {
        return Err(ErrorCode::BalanceAlreadyFrozen.into());
    }

    ctx.accounts.user_timed_info.is_balance_frozen = true;

    Ok(())
}
