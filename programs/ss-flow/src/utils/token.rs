use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

/// `transfer_from_pool_to_user` is a helper function that transfers tokens from the pool to the user.
pub fn transfer_from_pool_to_user<'info>(
    pool: &Account<'info, Pool>,
    pool_vault: &Account<'info, TokenAccount>,
    user_token_account: &Account<'info, TokenAccount>,
    token_program: &Program<'info, Token>,
    amount: u64,
) -> Result<()> {
    token::transfer(
        CpiContext::new_with_signer(
            token_program.to_account_info().clone(),
            Transfer {
                from: pool_vault.to_account_info().clone(),
                to: user_token_account.to_account_info().clone(),
                authority: pool.to_account_info().clone(),
            },
            &[&pool.seeds()],
        ),
        amount,
    )
}
