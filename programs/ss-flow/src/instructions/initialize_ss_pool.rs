use crate::event::InitializePoolEvent;
use crate::state::pool::Pool;
use crate::InitializeSSPool;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};

pub fn handler(ctx: Context<InitializeSSPool>, amount: u64, proportion: u64) -> Result<()> {
    let pool = &mut ctx.accounts.pool;

    pool.base = ctx.accounts.base.key();
    pool.admin = ctx.accounts.admin.key();

    pool.bump = [ctx.bumps.pool];

    pool.amount = amount;
    pool.proportion = proportion;

    pool.mint_a = ctx.accounts.mint_a.key();
    pool.vault_a = ctx.accounts.vault_a.key();

    pool.mint_b = ctx.accounts.mint_b.key();
    pool.vault_b = ctx.accounts.vault_b.key();

    emit!(InitializePoolEvent {
        admin: pool.admin,
        pool: pool.key(),
        token_a: pool.mint_a,
        token_b: pool.mint_b,
        amount,
        proportion,
    });

    Ok(())
}
