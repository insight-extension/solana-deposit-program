use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

use crate::{
    constants::{MASTER_WALLET, USER_INFO_SEED},
    error::ErrorCode,
    UserSubscriptionInfo,
};

#[derive(Accounts)]
pub struct RefundBalance<'info> {
    #[account(mut)]
    pub user: Signer<'info>, // User must sign the transaction
    #[account(mut, address = MASTER_WALLET)]
    pub master: Signer<'info>,
    #[account(mint::token_program = token_program)]
    pub token: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        seeds = [USER_INFO_SEED, user.key().as_ref()],
        bump = user_info.bump
    )]
    pub user_info: Account<'info, UserSubscriptionInfo>,
    #[account(
        mut,
        associated_token::mint = token,
        associated_token::authority = user_info, // Authority should be user_info
        associated_token::token_program = token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = token,
        associated_token::authority = user, // User must have authority here
        associated_token::token_program = token_program,
    )]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

pub fn refund_balance_handler(ctx: Context<RefundBalance>) -> Result<()> {
    #[cfg(any(feature = "devnet", feature = "mainnet"))]
    {
        if ctx.accounts.token.key() != USDC_MINT {
            return Err(ErrorCode::InvalidToken.into());
        }
    }
    let amount = ctx.accounts.user_info.available_balance;
    if amount == 0 {
        return Err(ErrorCode::InsufficientBalance.into());
    }
    // Transfer the available balance from "vault" back to the user's token account
    send_to_user(&ctx, amount)?;
    // Update the UserInfo balance to zero after refund
    ctx.accounts.user_info.available_balance = 0;
    msg!("Refunded {} tokens to user.", amount);
    Ok(())
}

fn send_to_user(ctx: &Context<RefundBalance>, amount: u64) -> Result<()> {
    let user_key = ctx.accounts.user.key();
    let seeds = &[
        USER_INFO_SEED,
        user_key.as_ref(),
        &[ctx.accounts.user_info.bump],
    ];
    let signer_seeds = &[&seeds[..]];
    let transfer_accounts = TransferChecked {
        from: ctx.accounts.vault.to_account_info(),
        mint: ctx.accounts.token.to_account_info(),
        to: ctx.accounts.user_token_account.to_account_info(),
        authority: ctx.accounts.user_info.to_account_info(),
    };
    let cpi_context = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        transfer_accounts,
        signer_seeds,
    );
    // Perform the token transfer
    transfer_checked(cpi_context, amount, ctx.accounts.token.decimals)?;
    Ok(())
}
