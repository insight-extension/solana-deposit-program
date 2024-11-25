use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct UserSubscriptionInfo {
    pub available_balance: u64,
    pub expiration: i64,
    pub bump: u8,
}
