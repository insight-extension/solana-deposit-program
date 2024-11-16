use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{
    constants::{MASTER_WALLET, USER_INFO_SEED},
    UserInfo,
};

#[derive(Accounts)]
pub struct PayPerTime<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut, address = MASTER_WALLET)]
    pub master: Signer<'info>,
    #[account(mint::token_program = token_program)]
    pub token: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        seeds = [USER_INFO_SEED, user.key().as_ref()],
        bump = user_info.bump
    )]
    pub user_info: Account<'info, UserInfo>,
    #[account(
        address = MASTER_WALLET
    )]
    pub master_wallet: SystemAccount<'info>,
    #[account(
        mut,
        associated_token::mint = token,
        associated_token::authority = master_wallet,
        associated_token::token_program = token_program
    )]
    pub master_wallet_token_account: InterfaceAccount<'info, TokenAccount>,
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

//TODO: implement this
pub fn pay_per_time_handler(_ctx: Context<PayPerTime>, _amount: u64) -> Result<()> {
    Ok(())
}
