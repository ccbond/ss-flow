use crate::event::RefundTokenBEvent;
use crate::state::pool::Pool;
use crate::utils::token::transfer;
use anchor_lang::context::Context;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(Accounts)]
pub struct RefundTokenB<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut,
        seeds = [
            b"flow_pool".as_ref()
        ],
        bump,
    )]
    pub pool: Account<'info, Pool>,

    #[account(
        mut,
        associated_token::authority = payer.key(),
        associated_token::mint = pool.mint_a
    )]
    pub token_a_ata: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        constraint = pool_token_a_vault.mint == pool.mint_a,
        constraint = pool_token_a_vault.owner == pool.key()
    )]
    pub pool_token_a_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::authority = payer.key(),
        associated_token::mint = pool.mint_b
    )]
    pub token_b_ata: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        constraint = pool_token_b_vault.mint == pool.mint_b,
        constraint = pool_token_b_vault.owner == pool.key()
    )]
    pub pool_token_b_vault: Box<Account<'info, TokenAccount>>,

    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<RefundTokenBEvent>, amount: u64) -> Result<()> {
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