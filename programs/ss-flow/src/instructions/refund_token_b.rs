use crate::event::RefundTokenBEvent;
use crate::state::pool::Pool;
use crate::utils::token::transfer;
use crate::RefundTokenB;
use anchor_lang::context::Context;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};

pub fn handler(ctx: Context<RefundTokenB>, amount: u64) -> Result<()> {
    let pool = ctx.accounts.pool;

    let withdraw_amount = amount / pool.proportion;

    pool.transfer_from_pool_to_user(
        pool,
        &ctx.accounts.pool_token_a_vault,
        &ctx.accounts.token_a_ata,
        &ctx.accounts.token_program,
        withdraw_amount,
    )?;

    ctx.accounts.receive_b(amount)?;

    pool.deposit_token_a_amount -= withdraw_amount;

    emit!(RefundTokenBEvent {
        payer: ctx.accounts.payer.key(),
        pool: pool.key(),
        token_b: pool.mint_b,
        amount,
    });

    Ok(())
}

impl<'info> RefundTokenB<'info> {
    fn receive_b(&self, amount: u64) -> Result<()> {
        transfer(
            &self.payer.to_account_info(),
            &self.token_b_ata,
            &self.pool_token_b_vault,
            &self.token_program,
            amount,
            None,
        )?;
        Ok(())
    }
}
