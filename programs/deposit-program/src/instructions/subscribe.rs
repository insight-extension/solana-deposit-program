use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{
    constants::{MASTER_WALLET, USDC_MINT, USER_SUBSCRIPTION_INFO_SEED},
    error::ErrorCode,
    get_subscription_level, send_tokens, UserSubscriptionInfo,
};

#[derive(Accounts)]
pub struct Subscribe<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut, address = MASTER_WALLET)]
    pub master: Signer<'info>,
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
        space = 8 + UserSubscriptionInfo::INIT_SPACE,
        seeds = [USER_SUBSCRIPTION_INFO_SEED, user.key().as_ref()],
        bump
    )]
    pub user_subscription_info: Account<'info, UserSubscriptionInfo>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = token,
        associated_token::authority = user_subscription_info,
        associated_token::token_program = token_program,
    )]
    pub subscription_vault: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = token,
        associated_token::authority = master,
        associated_token::token_program = token_program
    )]
    pub master_token_account: InterfaceAccount<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

pub fn subscribe_handler(ctx: Context<Subscribe>, amount: u64) -> Result<()> {
    #[cfg(any(feature = "devnet", feature = "mainnet"))]
    {
        if ctx.accounts.token.key() != USDC_MINT {
            return Err(ErrorCode::InvalidToken.into());
        }
    }
    let (subscription_cost, duration) = get_subscription_level(amount)?;
    // Calculate the amount to send to the vault
    let vault_amount = amount - subscription_cost;
    let current_timestamp = Clock::get()?.unix_timestamp;
    // Check if the user already has an active subscription
    if ctx.accounts.user_subscription_info.expiration > current_timestamp {
        msg!("User already has an active subscription.");
        return Err(ErrorCode::AlreadySubscribed.into());
    } else {
        msg!("No active subscription found. Proceeding with new subscription.");
        send_to_master_wallet(&ctx, subscription_cost)?; // Send subscription cost to master wallet
        if vault_amount > 0 {
            send_to_vault(&ctx, vault_amount)?; // Send remaining amount to vault
        }
        save_user_info(ctx, vault_amount, duration)?; // Save user info with updated balance
    }
    msg!("Subscription processed successfully.");
    Ok(())
}

fn send_to_master_wallet(ctx: &Context<Subscribe>, amount: u64) -> Result<()> {
    send_tokens(
        ctx.accounts.user_token_account.to_account_info(),
        ctx.accounts.token.to_account_info(),
        ctx.accounts.master_token_account.to_account_info(),
        ctx.accounts.user.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.token.decimals,
        amount,
    )?;
    msg!("Transferred {} tokens to master wallet.", amount);
    Ok(())
}

fn send_to_vault(ctx: &Context<Subscribe>, amount: u64) -> Result<()> {
    send_tokens(
        ctx.accounts.user_token_account.to_account_info(),
        ctx.accounts.token.to_account_info(),
        ctx.accounts.subscription_vault.to_account_info(),
        ctx.accounts.user.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.token.decimals,
        amount,
    )?;
    Ok(())
}

fn save_user_info(ctx: Context<Subscribe>, available_balance: u64, duration: u64) -> Result<()> {
    let current_timestamp = Clock::get()?.unix_timestamp;
    let expiration = current_timestamp + duration as i64;
    let bump = ctx.bumps.user_subscription_info;
    ctx.accounts
        .user_subscription_info
        .set_inner(UserSubscriptionInfo {
            available_balance,
            expiration,
            bump,
        });
    Ok(())
}
