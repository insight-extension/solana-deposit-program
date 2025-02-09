use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid token")]
    InvalidToken,
    #[msg("Balance already frozen")]
    BalanceAlreadyFrozen,
    #[msg("Balance frozen")]
    BalanceFrozen,
    #[msg("Balance not frozen")]
    BalanceNotFrozen,
}
