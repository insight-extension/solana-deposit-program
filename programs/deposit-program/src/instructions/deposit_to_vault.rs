use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{send_tokens, UserInfo, USER_INFO_SEED};

#[derive(Accounts)]
pub struct DepositToVault<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mint::token_program = token_program)]
    pub token: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = token,
        associated_token::authority = user,
        associated_token::token_program = token_program,
    )]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init_if_needed,        
        payer = user,
        space = 8 + UserInfo::INIT_SPACE,
        seeds = [USER_INFO_SEED, user.key().as_ref()],
        bump
    )]
    user_info: Account<'info, UserInfo>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = token,
        associated_token::authority = user_info,
        associated_token::token_program = token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

pub fn deposit_to_vault_handler(ctx: Context<DepositToVault>, amount: u64) -> Result<()> {
    #[cfg(any(feature = "devnet", feature = "mainnet"))]
    {
        if ctx.accounts.token.key() != USDC_MINT {
            return Err(ErrorCode::InvalidToken.into());
        }
    }
    send_tokens(
        ctx.accounts.user_token_account.to_account_info(),
        ctx.accounts.token.to_account_info(), 
        ctx.accounts.vault.to_account_info(),
        ctx.accounts.user.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.token.decimals,
        amount,
    )?;
    save_user_info(ctx, amount)?;
    msg!("Deposited {} tokens to vault.", amount);
    Ok(())
}

fn save_user_info(ctx: Context<DepositToVault>, amount: u64) -> Result<()> {
    ctx.accounts.user_info.available_balance += amount;
    ctx.accounts.user_info.bump = ctx.bumps.user_info;
    Ok(())
}
