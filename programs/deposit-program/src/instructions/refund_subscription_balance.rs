use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

use crate::{
    constants::{MASTER_WALLET, USDC_MINT, USER_SUBSCRIPTION_INFO_SEED},
    error::ErrorCode,
    UserSubscriptionInfo,
};

#[derive(Accounts)]
pub struct RefundSubscriptionBalance<'info> {
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
        associated_token::authority = user_subscription_info,
        associated_token::token_program = token_program,
    )]
    pub subscription_vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = token,
        associated_token::authority = user,
        associated_token::token_program = token_program,
    )]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

pub fn refund_subscription_balance_handler(ctx: Context<RefundSubscriptionBalance>) -> Result<()> {
    #[cfg(any(feature = "devnet", feature = "mainnet"))]
    {
        if ctx.accounts.token.key() != USDC_MINT {
            return Err(ErrorCode::InvalidToken.into());
        }
    }
    let amount = ctx.accounts.user_subscription_info.available_balance;
    if amount == 0 {
        return Err(ErrorCode::InsufficientBalance.into());
    }
    send_to_user_from_subscription_vault(&ctx, amount)?;
    ctx.accounts.user_subscription_info.available_balance = 0;
    msg!("Refunded {} tokens to user.", amount);
    Ok(())
}

fn send_to_user_from_subscription_vault(
    ctx: &Context<RefundSubscriptionBalance>,
    amount: u64,
) -> Result<()> {
    let user_key = ctx.accounts.user.key();
    let seeds = &[
        USER_SUBSCRIPTION_INFO_SEED,
        user_key.as_ref(),
        &[ctx.accounts.user_subscription_info.bump],
    ];
    let signer_seeds = &[&seeds[..]];
    let transfer_accounts = TransferChecked {
        from: ctx.accounts.subscription_vault.to_account_info(),
        mint: ctx.accounts.token.to_account_info(),
        to: ctx.accounts.user_token_account.to_account_info(),
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
