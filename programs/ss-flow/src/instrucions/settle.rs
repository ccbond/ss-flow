use crate::state::pool::Pool;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};
use whirlpool_cpi::{self, state::*};

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
        payer = pool.payer,
        space = 8+ Pool::LEN
    )]
    pub pool: Box<Account<'info, Pool>>,

    /// `position` will be initialized by the `Position` string、pool address and authority nft mint.
    #[account(
            init,
            seeds=[b"Position".as_ref(), pool.key().as_ref(), authority_nft_mint.key().as_ref()],
            payer = payer,
            bump,
            space = 8 + Position::LEN)]
    pub position: Account<'info, Position>,

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
        constraint = authority_nft_ata.owner == receiver.key(),
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
        position_authority: ctx.accounts.payer.to_account_info(),
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
        payer: ctx.accounts.payer.key(),
        pool: pool.key(),
        token_a: pool.mint_a,
        amount,
    });

    Ok(())
}
