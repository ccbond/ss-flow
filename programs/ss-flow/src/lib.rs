use crate::instrucions::add_token_a::AddTokenA;
use crate::instrucions::initialize_pool::InitializePool;
use crate::instrucions::refund_token_b::RefundTokenB;
use crate::instrucions::settle::Settle;

use anchor_lang::prelude::*;

declare_id!("8K8ELAz6Q5uvNwQz1iYqn8BnLU1Lf1LFghCTSnBvHAF6");

pub mod errors;
pub mod event;
pub mod instrucions;
pub mod state;
pub mod utils;

use instrucions::*;

#[program]
pub mod ss_flow {
    use self::{
        add_token_a::AddTokenA, initialize_pool::InitializePool, refund_token_b::RefundTokenB,
    };

    use super::*;

    pub fn initialize_pool(
        ctx: Context<InitializePool>,
        amount: u64,
        proportion: u64,
    ) -> Result<()> {
        instrucions::initialize_pool::handler(ctx, amount, proportion)?;
        Ok(())
    }

    pub fn add_token_a(ctx: Context<AddTokenA>, amount: u64) -> Result<()> {
        instrucions::add_token_a::handler(ctx, amount)?;
        Ok(())
    }

    pub fn refund_token_b(ctx: Context<RefundTokenB>, amount: u64) -> Result<()> {
        instrucions::refund_token_b::handler(ctx, amount)?;
        Ok(())
    }

    pub fn settle(
        ctx: Context<Settle>,
        precent: u64,
        liquidity_amount: u128,
        token_max_b: u64,
    ) -> Result<()> {
        instrucions::settle::handler(ctx, precent, liquidity_amount, token_max_b)?;
        Ok(())
    }
}
