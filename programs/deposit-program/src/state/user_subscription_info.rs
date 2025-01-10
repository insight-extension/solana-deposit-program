use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct UserSubscriptionInfo {
    pub expiration: i64,
    pub bump: u8,
}
