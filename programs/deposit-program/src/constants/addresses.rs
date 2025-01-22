use anchor_lang::prelude::*;

#[cfg(feature = "localnet")]
pub const MASTER_WALLET: Pubkey = pubkey!("71q6LEWUkPZhYChjAcZcuxVVyDqdEyjf95etzte2PzwK");
// note: this one isn't actually used.
#[cfg(feature = "localnet")]
pub const USDC_MINT: Pubkey = pubkey!("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU");
#[cfg(feature = "devnet")]
pub const MASTER_WALLET: Pubkey = pubkey!("71q6LEWUkPZhYChjAcZcuxVVyDqdEyjf95etzte2PzwK");
#[cfg(feature = "devnet")]
pub const USDC_MINT: Pubkey = pubkey!("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU");
#[cfg(feature = "mainnet")]
pub const MASTER_WALLET: Pubkey = pubkey!("71q6LEWUkPZhYChjAcZcuxVVyDqdEyjf95etzte2PzwK");
#[cfg(feature = "mainnet")]
pub const USDC_MINT: Pubkey = pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
