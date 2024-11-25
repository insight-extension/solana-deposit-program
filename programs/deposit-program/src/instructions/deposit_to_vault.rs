use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{
    constants::{USER_SUBSCRIPTION_INFO_SEED, USER_TIMED_INFO_SEED}, error::ErrorCode, send_tokens, DepositType, UserSubscriptionInfo, UserTimedInfo
};

#[derive(Accounts)]
pub struct DepositToVault<'info> {
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
        space = 8 + UserSubscriptionInfo::INIT_SPACE,
        seeds = [USER_SUBSCRIPTION_INFO_SEED, user.key().as_ref()],
        bump
    )]
    pub user_subscription_info: Box<Account<'info, UserSubscriptionInfo>>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = token,
        associated_token::authority = user_timed_info,
        associated_token::token_program = token_program,
    )]
    pub timed_vault: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = token,
        associated_token::authority = user_subscription_info,
        associated_token::token_program = token_program,
    )]
    pub subscription_vault: Box<InterfaceAccount<'info, TokenAccount>>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}


pub fn deposit_to_vault_handler(ctx: Context<DepositToVault>, deposit_type: DepositType, amount: u64) -> Result<()> {
    #[cfg(any(feature = "devnet", feature = "mainnet"))]
    {
        if ctx.accounts.token.key() != USDC_MINT {
            return Err(ErrorCode::InvalidToken.into());
        }
    }
    if deposit_type == DepositType::Subscription {
        send_tokens(
            ctx.accounts.user_token_account.to_account_info(),
            ctx.accounts.token.to_account_info(), 
            ctx.accounts.subscription_vault.to_account_info(),
            ctx.accounts.user.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.token.decimals,
            amount,
        )?;
        save_user_subscription_info(ctx, amount)?;
        msg!("Deposited {} tokens to subscription vault.", amount);
    } else if deposit_type == DepositType::Timed {
        send_tokens(
            ctx.accounts.user_token_account.to_account_info(),
            ctx.accounts.token.to_account_info(), 
            ctx.accounts.timed_vault.to_account_info(),
            ctx.accounts.user.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.token.decimals,
            amount,
        )?;
        save_user_timed_info(ctx, amount)?;
        msg!("Deposited {} tokens to timed vault.", amount);
    } else {
        return Err(ErrorCode::InvalidDepositType.into());
    }
    Ok(())
}

fn save_user_subscription_info(ctx: Context<DepositToVault>, amount: u64) -> Result<()> {
    ctx.accounts.user_subscription_info.available_balance += amount;
    ctx.accounts.user_subscription_info.bump = ctx.bumps.user_subscription_info;
    Ok(())
}

fn save_user_timed_info(ctx: Context<DepositToVault>, amount: u64) -> Result<()> {
    ctx.accounts.user_timed_info.available_balance += amount;
    ctx.accounts.user_timed_info.bump = ctx.bumps.user_timed_info;
    Ok(())
}
