use crate::event::InitializePoolEvent;
use crate::state::pool::Pool;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(Accounts)]
pub struct InitializeSSPool<'info> {
    /// `base` is used to initialize admin account.
    pub base: Signer<'info>,

    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(init,
        seeds = [
            b"flow_pool".as_ref()
        ],
        bump,
        payer = admin,
        space = 8+ Pool::LEN
    )]
    pub pool: Account<'info, Pool>,

    pub mint_a: Account<'info, Mint>,

    #[account(init,
        payer = admin,
        associated_token::mint = mint_a,
        associated_token::authority = pool
      )]
    pub vault_a: Account<'info, TokenAccount>,

    pub mint_b: Account<'info, Mint>,

    #[account(init,
        payer = admin,
        associated_token::mint = mint_b,
        associated_token::authority = pool
      )]
    pub vault_b: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<InitializeSSPool>, amount: u64, proportion: u64) -> Result<()> {
    let pool = &mut ctx.accounts.pool;

    pool.base = ctx.accounts.base.key();
    pool.admin = ctx.accounts.admin.key();

    pool.bump = vec![ctx.bumps.pool];

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
