use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct UserTimedInfo {
    pub bump: u8,
}
