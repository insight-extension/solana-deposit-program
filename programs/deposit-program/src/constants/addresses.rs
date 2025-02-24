use anchor_lang::prelude::*;

#[cfg(feature = "localnet")]
pub const MASTER_WALLET: Pubkey = pubkey!("71q6LEWUkPZhYChjAcZcuxVVyDqdEyjf95etzte2PzwK");
// note: this one isn't actually used.
#[cfg(feature = "localnet")]
pub const INSIGHT_MINT: Pubkey = pubkey!("3L5GoXVKdhb9rRcxehKW94TebLaKtd8G8Q6kEyPxwHba");
#[cfg(feature = "devnet")]
pub const MASTER_WALLET: Pubkey = pubkey!("71q6LEWUkPZhYChjAcZcuxVVyDqdEyjf95etzte2PzwK");
#[cfg(feature = "devnet")]
pub const INSIGHT_MINT: Pubkey = pubkey!("3L5GoXVKdhb9rRcxehKW94TebLaKtd8G8Q6kEyPxwHba");
#[cfg(feature = "mainnet")]
pub const MASTER_WALLET: Pubkey = pubkey!("71q6LEWUkPZhYChjAcZcuxVVyDqdEyjf95etzte2PzwK");
#[cfg(feature = "mainnet")]
pub const INSIGHT_MINT: Pubkey = pubkey!("3L5GoXVKdhb9rRcxehKW94TebLaKtd8G8Q6kEyPxwHba");
