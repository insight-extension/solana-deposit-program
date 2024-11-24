use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct UserTimedInfo {
    pub available_balance: u64,
    pub bump: u8,
}
