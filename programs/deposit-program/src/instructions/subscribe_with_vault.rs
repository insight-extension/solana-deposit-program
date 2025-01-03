use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

use crate::{
    constants::{MASTER_WALLET, USDC_MINT, USER_SUBSCRIPTION_INFO_SEED},
    error::ErrorCode,
    get_subscription_level, UserSubscriptionInfo,
};

#[derive(Accounts)]
pub struct SubscribeWithVault<'info> {
    #[account(mut, address = MASTER_WALLET)]
    pub master: Signer<'info>,

    #[account(mut)]
    pub user: SystemAccount<'info>,

    #[account(mint::token_program = token_program)]
    pub token: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        seeds = [USER_SUBSCRIPTION_INFO_SEED, user.key().as_ref()],
        bump = user_subscription_info.bump
    )]
    pub user_subscription_info: Account<'info, UserSubscriptionInfo>,

    #[account(
        mut,
        associated_token::mint = token,
        associated_token::authority = master,
        associated_token::token_program = token_program
    )]
    pub master_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = token,
        associated_token::authority = user_subscription_info,
        associated_token::token_program = token_program,
    )]
    pub subscription_vault: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

pub fn subscribe_with_vault_handler(ctx: Context<SubscribeWithVault>, amount: u64) -> Result<()> {
    #[cfg(any(feature = "devnet", feature = "mainnet"))]
    {
        if ctx.accounts.token.key() != USDC_MINT {
            return Err(ErrorCode::InvalidToken.into());
        }
    }
    let current_timestamp = Clock::get()?.unix_timestamp;
    if ctx.accounts.user_subscription_info.expiration > current_timestamp {
        return Err(ErrorCode::AlreadySubscribed.into());
    } else if ctx.accounts.user_subscription_info.available_balance < amount {
        return Err(ErrorCode::InsufficientBalance.into());
    } else {
        let (subscription_cost, duration) = get_subscription_level(amount)?;
        send_to_master_wallet(&ctx, subscription_cost)?;
        msg!(
            "Subscription cost of {} tokens has been sent to the master wallet.",
            subscription_cost
        );
        update_user_info(ctx, subscription_cost, duration)?;
    }
    msg!("Subscription successful.");
    Ok(())
}

fn send_to_master_wallet(ctx: &Context<SubscribeWithVault>, amount: u64) -> Result<()> {
    let seeds = &[
        USER_SUBSCRIPTION_INFO_SEED,
        ctx.accounts.user.key.as_ref(),
        &[ctx.accounts.user_subscription_info.bump],
    ];
    let signer_seeds = &[&seeds[..]];
    let transfer_accounts = TransferChecked {
        from: ctx.accounts.subscription_vault.to_account_info(),
        mint: ctx.accounts.token.to_account_info(),
        to: ctx.accounts.master_token_account.to_account_info(),
        authority: ctx.accounts.user_subscription_info.to_account_info(),
    };
    let cpi_context = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        transfer_accounts,
        signer_seeds,
    );
    transfer_checked(cpi_context, amount, ctx.accounts.token.decimals)?;
    Ok(())
}

fn update_user_info(ctx: Context<SubscribeWithVault>, amount: u64, duration: u64) -> Result<()> {
    let current_timestamp = Clock::get()?.unix_timestamp;
    let expiration = current_timestamp + duration as i64;
    ctx.accounts.user_subscription_info.available_balance -= amount;
    ctx.accounts.user_subscription_info.expiration = expiration;
    Ok(())
}
