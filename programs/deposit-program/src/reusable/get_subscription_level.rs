use crate::{constants::SUBSCRIPTION_LEVELS, error::ErrorCode};

pub fn get_subscription_level(amount: u64) -> Result<(u64, u64), ErrorCode> {
    match SUBSCRIPTION_LEVELS
        .iter()
        .rev()
        .find(|(cost, _)| amount >= *cost)
    {
        Some(level) => Ok(*level),
        None => Err(ErrorCode::InsufficientBalance),
    }
}
