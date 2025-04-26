use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct UserInfo {
    pub per_hour_left: i64,
    pub is_balance_frozen: bool,
    pub subscription_ends_at: i64,
    pub bump: u8,
}
