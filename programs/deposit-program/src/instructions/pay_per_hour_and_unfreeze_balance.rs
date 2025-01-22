use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

use crate::{
    constants::{MASTER_WALLET, USER_INFO_SEED},
    UserInfo,
};

#[derive(Accounts)]
pub struct PayPerHourAndUnfreezeBalance<'info> {
    #[account(mut, address = MASTER_WALLET)]
    pub master: Signer<'info>,

    #[account(mut)]
    pub user: SystemAccount<'info>,

    #[account(mint::token_program = token_program)]
    pub token: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        seeds = [USER_INFO_SEED, user.key().as_ref()],
        bump = user_info.bump
    )]
    pub user_info: Account<'info, UserInfo>,

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
        associated_token::authority = user_info,
        associated_token::token_program = token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

pub fn pay_per_hour_and_unfreeze_balance(
    ctx: Context<PayPerHourAndUnfreezeBalance>,
    amount: u64,
    per_hour_left: i64,
) -> Result<()> {
    #[cfg(any(feature = "devnet", feature = "mainnet"))]
    {
        if ctx.accounts.token.key() != USDC_MINT {
            return Err(ErrorCode::InvalidToken.into());
        }
    }

    send_to_master_wallet(&ctx, amount)?;
    msg!(
        "Payment of {} tokens has been sent to the master wallet.",
        amount
    );

    ctx.accounts.user_info.per_hour_left = per_hour_left;
    msg!("Balance has been frozen for {} hours.", per_hour_left);
    ctx.accounts.user_info.is_balance_frozen = false;
    msg!("Balance has been unfrozen.");

    Ok(())
}

fn send_to_master_wallet(ctx: &Context<PayPerHourAndUnfreezeBalance>, amount: u64) -> Result<()> {
    let seeds = &[
        USER_INFO_SEED,
        ctx.accounts.user.key.as_ref(),
        &[ctx.accounts.user_info.bump],
    ];

    let signer_seeds = &[&seeds[..]];

    let transfer_accounts = TransferChecked {
        from: ctx.accounts.vault.to_account_info(),
        mint: ctx.accounts.token.to_account_info(),
        to: ctx.accounts.master_token_account.to_account_info(),
        authority: ctx.accounts.user_info.to_account_info(),
    };

    let cpi_context = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        transfer_accounts,
        signer_seeds,
    );

    transfer_checked(cpi_context, amount, ctx.accounts.token.decimals)?;

    Ok(())
}
