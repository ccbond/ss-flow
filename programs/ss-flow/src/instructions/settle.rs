use crate::event::SettleEvent;
use crate::state::{pool::Pool, position::Position};
use crate::Settle;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};
use whirlpool_cpi::{self, state::*};

pub fn handler(
    ctx: Context<Settle>,
    precent: u64,
    liquidity_amount: u128,
    token_max_b: u64,
) -> Result<()> {
    let pool = &mut ctx.accounts.pool;

    let token_max_a = pool.amount / 2;

    let cpi_program = ctx.accounts.whirlpool_program.to_account_info();

    // add liquidity
    let cpi_accounts = whirlpool_cpi::cpi::accounts::ModifyLiquidity {
        whirlpool: ctx.accounts.whirlpool.to_account_info(),
        token_program: ctx.accounts.token_program.to_account_info(),
        position_authority: ctx.accounts.owner.to_account_info(),
        position: ctx.accounts.position.to_account_info(),
        position_token_account: ctx.accounts.position_token_account.to_account_info(),
        token_owner_account_a: ctx.accounts.pool_token_a_vault.to_account_info(),
        token_owner_account_b: ctx.accounts.token_owner_account_b.to_account_info(),
        token_vault_a: ctx.accounts.token_vault_a.to_account_info(),
        token_vault_b: ctx.accounts.token_vault_b.to_account_info(),
        tick_array_lower: ctx.accounts.tick_array_lower.to_account_info(),
        tick_array_upper: ctx.accounts.tick_array_upper.to_account_info(),
    };

    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

    whirlpool_cpi::cpi::increase_liquidity(cpi_ctx, liquidity_amount, token_max_a, token_max_b)?;

    emit!(SettleEvent {
        admin: ctx.accounts.owner.key(),
        pool: pool.key(),
        token_a: pool.mint_a,
        token_b: pool.mint_b,
        amount_a: token_max_a,
        amount_b: token_max_b,
    });

    Ok(())
}
