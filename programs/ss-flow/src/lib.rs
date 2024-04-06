use anchor_lang::prelude::*;

use whirlpool_cpi::state::OpenPositionBumps;

declare_id!("Eh3RT5S8YUbkCG8kfRwZBWETuot3AebXYaZLTgRey5SA");

pub mod errors;
pub mod event;
pub mod instructions;
pub mod state;
pub mod utils;

use instructions::*;

#[program]
pub mod ss_flow {
    use super::*;

    pub fn initialize_pool(
        ctx: Context<InitializeSSPool>,
        amount: u64,
        proportion: u64,
    ) -> Result<()> {
        return instructions::initialize_ss_pool::handler(ctx, amount, proportion);
    }

    pub fn add_token_a(ctx: Context<AddTokenA>, amount: u64) -> Result<()> {
        return instructions::add_token_a::handler(ctx, amount);
    }

    // pub fn refund_token_b(ctx: Context<RefundTokenB>, amount: u64) -> Result<()> {
    //     return instructions::refund_token_b::handler(ctx, amount);
    // }

    pub fn settle(
        ctx: Context<Settle>,
        precent: u64,
        liquidity_amount: u128,
        token_max_b: u64,
    ) -> Result<()> {
        return instructions::settle::handler(ctx, precent, liquidity_amount, token_max_b);
    }
}
