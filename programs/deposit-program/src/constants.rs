use anchor_lang::prelude::*;

#[constant]
pub const MIN_DEPOSIT: u64 = 1_000_000; // 1 USDC
pub const SUBSCRIPTION_LEVELS: [(u64, u64); 3] = [
    (5_000_000, 30 * 24 * 60 * 60),   // 5 USDC, 30 days
    (10_000_000, 90 * 24 * 60 * 60),  // 10 USDC, 90 days
    (20_000_000, 180 * 24 * 60 * 60), // 20 USDC, 180 days
];
pub const USER_INFO_SEED: &[u8] = b"user_info";

#[cfg(feature = "localnet")]
pub const MASTER_WALLET: Pubkey = pubkey!("71q6LEWUkPZhYChjAcZcuxVVyDqdEyjf95etzte2PzwK");

#[cfg(feature = "devnet")]
pub const MASTER_WALLET: Pubkey = pubkey!("71q6LEWUkPZhYChjAcZcuxVVyDqdEyjf95etzte2PzwK");
#[cfg(feature = "devnet")]
pub const USDC_MINT: Pubkey = pubkey!("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU");

#[cfg(feature = "mainnet")]
pub const MASTER_WALLET: Pubkey = pubkey!("71q6LEWUkPZhYChjAcZcuxVVyDqdEyjf95etzte2PzwK");
#[cfg(feature = "mainnet")]
pub const USDC_MINT: Pubkey = pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
