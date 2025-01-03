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

    pub fn deposit_to_timed_vault(ctx: Context<DepositToTimedVault>, amount: u64) -> Result<()> {
        instructions::deposit_to_timed_vault::deposit_to_timed_vault_handler(ctx, amount)
    }

    pub fn deposit_to_subscription_vault(
        ctx: Context<DepositToSubscriptionVault>,
        amount: u64,
    ) -> Result<()> {
        instructions::deposit_to_subscription_vault::deposit_to_subscription_vault_handler(
            ctx, amount,
        )
    }

    pub fn refund_timed_balance(ctx: Context<RefundTimedBalance>) -> Result<()> {
        instructions::refund_timed_balance::refund_timed_balance_handler(ctx)
    }

    pub fn refund_subscription_balance(ctx: Context<RefundSubscriptionBalance>) -> Result<()> {
        instructions::refund_subscription_balance::refund_subscription_balance_handler(ctx)
    }

    pub fn subscribe_with_vault(ctx: Context<SubscribeWithVault>, amount: u64) -> Result<()> {
        instructions::subscribe_with_vault::subscribe_with_vault_handler(ctx, amount)
    }

    pub fn pay_per_time(ctx: Context<PayPerTime>, amount: u64) -> Result<()> {
        instructions::pay_per_time::pay_per_time_handler(ctx, amount)
    }
}
