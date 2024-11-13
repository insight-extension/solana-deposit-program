use anchor_lang::prelude::*;

#[constant]
pub const MIN_DEPOSIT: u64 = 1_000_000; // 1 USDC
pub const SUBSCRIPTION_LEVELS: [(u64, u64); 3] = [
    (5_000_000, 30 * 24 * 60 * 60),   // 5 USDC, 30 days
    (10_000_000, 90 * 24 * 60 * 60),  // 10 USDC, 90 days
    (20_000_000, 180 * 24 * 60 * 60), // 20 USDC, 180 days
];
// Master Wallet for testing.
pub const LOCAL_MASTER_WALLET: Pubkey = pubkey!("71q6LEWUkPZhYChjAcZcuxVVyDqdEyjf95etzte2PzwK");
pub const USER_INFO_SEED: &[u8] = b"user_info";
