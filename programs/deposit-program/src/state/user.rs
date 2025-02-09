use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct UserInfo {
    pub per_hour_left: i64,
    pub is_balance_frozen: bool,
    pub bump: u8,
}
