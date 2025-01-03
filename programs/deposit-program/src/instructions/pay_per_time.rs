use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

use crate::{
    constants::{MASTER_WALLET, USDC_MINT, USER_TIMED_INFO_SEED},
    error::ErrorCode,
    UserTimedInfo,
};

#[derive(Accounts)]
pub struct PayPerTime<'info> {
    #[account(mut, address = MASTER_WALLET)]
    pub master: Signer<'info>,

    #[account(mut)]
    pub user: SystemAccount<'info>,

    #[account(mint::token_program = token_program)]
    pub token: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        seeds = [USER_TIMED_INFO_SEED, user.key().as_ref()],
        bump = user_timed_info.bump
    )]
    pub user_timed_info: Account<'info, UserTimedInfo>,

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
        associated_token::authority = user_timed_info,
        associated_token::token_program = token_program,
    )]
    pub timed_vault: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

pub fn pay_per_time_handler(ctx: Context<PayPerTime>, amount: u64) -> Result<()> {
    #[cfg(any(feature = "devnet", feature = "mainnet"))]
    {
        if ctx.accounts.token.key() != USDC_MINT {
            return Err(ErrorCode::InvalidToken.into());
        }
    }
    send_to_master_wallet(&ctx, amount)?;
    update_user_info(ctx, amount)?;
    msg!(
        "Payment of {} tokens has been sent to the master wallet.",
        amount
    );
    Ok(())
}

fn send_to_master_wallet(ctx: &Context<PayPerTime>, amount: u64) -> Result<()> {
    let seeds = &[
        USER_TIMED_INFO_SEED,
        ctx.accounts.user.key.as_ref(),
        &[ctx.accounts.user_timed_info.bump],
    ];
    let signer_seeds = &[&seeds[..]];
    let transfer_accounts = TransferChecked {
        from: ctx.accounts.timed_vault.to_account_info(),
        mint: ctx.accounts.token.to_account_info(),
        to: ctx.accounts.master_token_account.to_account_info(),
        authority: ctx.accounts.user_timed_info.to_account_info(),
    };
    let cpi_context = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        transfer_accounts,
        signer_seeds,
    );
    transfer_checked(cpi_context, amount, ctx.accounts.token.decimals)?;
    Ok(())
}

fn update_user_info(ctx: Context<PayPerTime>, amount: u64) -> Result<()> {
    ctx.accounts.user_timed_info.available_balance -= amount;
    Ok(())
}
