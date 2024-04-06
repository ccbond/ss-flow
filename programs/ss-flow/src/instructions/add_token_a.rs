use crate::event::AddTokenAEvent;
use crate::state::pool::Pool;
use crate::utils::token::{transfer, transfer_from_pool_to_user};
use crate::AddTokenA;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};

pub fn handler(ctx: Context<AddTokenA>, amount: u64) -> Result<()> {
    let withdraw_amount = ctx.accounts.pool.proportion * amount;

    transfer_from_pool_to_user(
        &mut ctx.accounts.pool,
        &ctx.accounts.pool_token_b_vault,
        &ctx.accounts.token_b_ata,
        &ctx.accounts.token_program,
        withdraw_amount,
    )?;

    ctx.accounts.receive_a(amount)?;
    ctx.accounts.pool.amount += amount;

    emit!(AddTokenAEvent {
        payer: ctx.accounts.payer.key(),
        pool: ctx.accounts.pool.key(),
        token_a: ctx.accounts.pool.mint_a,
        amount,
    });

    Ok(())
}

impl<'info> AddTokenA<'info> {
    fn receive_a(&self, amount: u64) -> Result<()> {
        transfer(
            &self.payer.to_account_info(),
            &self.token_a_ata,
            &self.pool_token_a_vault,
            &self.token_program,
            amount,
            None,
        )?;
        Ok(())
    }
}
