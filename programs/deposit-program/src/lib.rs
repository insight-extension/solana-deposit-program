pub mod constants;
pub mod error;
pub mod instructions;
pub mod reusable;
pub mod state;
use anchor_lang::prelude::*;
pub use instructions::*;
pub use reusable::*;
pub use state::*;

declare_id!("7ttwWrz4cAwKJQqCN6TAWPTznAUkWUjb1qouiZzPNxwP");

#[program]
pub mod deposit_program {
    use super::*;

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        instructions::deposit::deposit(ctx, amount)
    }

    pub fn freeze_balance(ctx: Context<BalanceState>) -> Result<()> {
        instructions::balance_state::freeze_balance(ctx)
    }

    pub fn unfreeze_balance(ctx: Context<BalanceState>) -> Result<()> {
        instructions::balance_state::unfreeze_balance(ctx)
    }

    pub fn pay_per_minute_and_unfreeze_balance(
        ctx: Context<PayPerMinuteAndUnfreezeBalance>,
        amount: u64,
    ) -> Result<()> {
        instructions::pay_per_minute_and_unfreeze_balance::pay_per_minute_and_unfreeze_balance(
            ctx, amount,
        )
    }

    pub fn pay_per_hour_and_unfreeze_balance(
        ctx: Context<PayPerHourAndUnfreezeBalance>,
        amount: u64,
        per_hour_left: i64,
    ) -> Result<()> {
        instructions::pay_per_hour_and_unfreeze_balance::pay_per_hour_and_unfreeze_balance(
            ctx,
            amount,
            per_hour_left,
        )
    }

    pub fn refund(ctx: Context<Refund>, amount: u64) -> Result<()> {
        instructions::refund::refund(ctx, amount)
    }
}
