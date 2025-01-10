use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{
    constants::{USDC_MINT, USER_TIMED_INFO_SEED}, send_tokens, UserTimedInfo, error::ErrorCode,
};

#[derive(Accounts)]
pub struct DepositToTimedVault<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mint::token_program = token_program)]
    pub token: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        mut,
        associated_token::mint = token,
        associated_token::authority = user,
        associated_token::token_program = token_program,
    )]
    pub user_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init_if_needed,        
        payer = user,
        space = 8 + UserTimedInfo::INIT_SPACE,
        seeds = [USER_TIMED_INFO_SEED, user.key().as_ref()],
        bump
    )]
    pub user_timed_info: Box<Account<'info, UserTimedInfo>>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = token,
        associated_token::authority = user_timed_info,
        associated_token::token_program = token_program,
    )]
    pub timed_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

pub fn deposit_to_timed_vault_handler(ctx: Context<DepositToTimedVault>, amount: u64) -> Result<()> {
    #[cfg(any(feature = "devnet", feature = "mainnet"))]
    {
        if ctx.accounts.token.key() != USDC_MINT {
            return Err(ErrorCode::InvalidToken.into());
        }
    }

    send_tokens(
        ctx.accounts.user_token_account.to_account_info(),
        ctx.accounts.token.to_account_info(), 
        ctx.accounts.timed_vault.to_account_info(),
        ctx.accounts.user.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.token.decimals,
        amount,
    )?;

    ctx.accounts.user_timed_info.bump = ctx.bumps.user_timed_info;
    msg!("Deposited {} tokens to timed vault.", amount);

    Ok(())
}
