use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient balance")]
    InsufficientBalance,
    #[msg("Unauthorized Withdrawal")]
    UnauthorizedWithdrawal,
    #[msg("Already subscribed")]
    AlreadySubscribed,
    #[msg("Invalid token")]
    InvalidToken,
    #[msg("Invalid deposit type")]
    InvalidDepositType,
}
