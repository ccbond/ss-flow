use crate::event::RefundTokenBEvent;
use crate::utils::token::{transfer, transfer_from_pool_to_user};
use crate::RefundTokenB;
use anchor_lang::context::Context;
use anchor_lang::prelude::*;

pub fn handler(ctx: Context<RefundTokenB>, amount: u64) -> Result<()> {
    let withdraw_amount = amount / ctx.accounts.pool.proportion;

    transfer_from_pool_to_user(
        &mut ctx.accounts.pool,
        &ctx.accounts.pool_token_a_vault,
        &ctx.accounts.token_a_ata,
        &ctx.accounts.token_program,
        withdraw_amount,
    )?;

    ctx.accounts.receive_b(amount)?;

    ctx.accounts.pool.amount += amount;

    emit!(RefundTokenBEvent {
        payer: ctx.accounts.payer.key(),
        pool: ctx.accounts.pool.key(),
        token_b: ctx.accounts.pool.mint_b,
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
