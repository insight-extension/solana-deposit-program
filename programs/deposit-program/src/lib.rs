pub mod constants;
pub mod error;
pub mod instructions;
pub mod reusable;
pub mod state;
use anchor_lang::prelude::*;
pub use instructions::*;
pub use reusable::*;
pub use state::*;

declare_id!("6vsijynzE22W8A4kkvv2Kq7a36ZEFPvJP9kBgNtN43mK");

#[program]
pub mod deposit_program {
    use super::*;

    pub fn subscribe(ctx: Context<Subscribe>, amount: u64) -> Result<()> {
        instructions::subscribe::subscribe_handler(ctx, amount)
    }

    pub fn refund_balance(ctx: Context<RefundBalance>) -> Result<()> {
        instructions::refund_balance::refund_balance_handler(ctx)
    }

    pub fn deposit_to_vault(ctx: Context<DepositToVault>, amount: u64) -> Result<()> {
        instructions::deposit_to_vault::deposit_to_vault_handler(ctx, amount)
    }

    pub fn subscribe_with_vault(ctx: Context<SubscribeWithVault>, amount: u64) -> Result<()> {
        instructions::subscribe_with_vault::subscribe_with_vault_handler(ctx, amount)
    }

    pub fn subscribe_by_time(ctx: Context<SubscribeByTime>, amount: u64) -> Result<()> {
        instructions::subscribe_by_time::subscribe_by_time_handler(ctx, amount)
    }
}
