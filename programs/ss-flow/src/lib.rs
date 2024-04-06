use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};
use state::{pool::Pool, position::Position};
use whirlpool_cpi::state::Position as WPosition;
use whirlpool_cpi::state::{OpenPositionBumps, TickArray, Whirlpool};

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

#[derive(Accounts)]
pub struct AddTokenA<'info> {
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

#[derive(Accounts)]
pub struct Settle<'info> {
    pub whirlpool_program: Program<'info, whirlpool_cpi::program::Whirlpool>,

    #[account(mut)]
    pub whirlpool: Account<'info, Whirlpool>,

    #[account(mut)]
    pub funder: Signer<'info>,

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(mut,
        seeds = [
            b"flow_pool".as_ref()
        ],
        bump,
    )]
    pub pool: Box<Account<'info, Pool>>,

    /// `position` will be initialized by the `Position` string、pool address and authority nft mint.
    #[account(
            init,
            seeds=[b"Position".as_ref(), pool.key().as_ref(), authority_nft_mint.key().as_ref()],
            payer = owner,
            bump,
            space = 8 + Position::LEN)]
    pub position: Account<'info, WPosition>,

    /// `authority_nft_mint` is only one withdraw permit of position.
    #[account(
        constraint = authority_nft_mint.decimals == 0,
        constraint = authority_nft_mint.supply == 1,
        constraint = authority_nft_mint.mint_authority.is_none(),
        constraint = authority_nft_mint.freeze_authority.is_none()
    )]
    pub authority_nft_mint: Box<Account<'info, Mint>>,

    /// `authority_nft_token` is the token of authority nft mint.
    #[account(
        constraint = authority_nft_ata.owner == owner.key(),
        constraint = authority_nft_ata.mint == authority_nft_mint.key(),
        constraint = authority_nft_ata.amount == 1
    )]
    pub authority_nft_ata: Account<'info, TokenAccount>,

    /// CHECK: init by whirlpool
    #[account(mut)]
    pub position_token_account: UncheckedAccount<'info>,

    #[account(
        mut,
        constraint = pool_token_a_vault.mint == pool.mint_a,
        constraint = pool_token_a_vault.owner == pool.key()
    )]
    pub pool_token_a_vault: Box<Account<'info, TokenAccount>>,

    #[account(mut, constraint = token_owner_account_b.mint == whirlpool.token_mint_b)]
    pub token_owner_account_b: Box<Account<'info, TokenAccount>>,

    #[account(mut, constraint = token_vault_a.key() == whirlpool.token_vault_a)]
    pub token_vault_a: Box<Account<'info, TokenAccount>>,

    #[account(mut, constraint = token_vault_b.key() == whirlpool.token_vault_b)]
    pub token_vault_b: Box<Account<'info, TokenAccount>>,

    #[account(mut, has_one = whirlpool)]
    pub tick_array_lower: AccountLoader<'info, TickArray>,

    #[account(mut, has_one = whirlpool)]
    pub tick_array_upper: AccountLoader<'info, TickArray>,

    pub token_a_ata: Box<Account<'info, TokenAccount>>,

    pub token_a_vault: Box<Account<'info, TokenAccount>>,

    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>,
}
