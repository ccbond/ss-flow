use crate::state::pool::Pool;
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

pub fn transfer<'info>(
    authority: &AccountInfo<'info>,
    source_ata: &Account<'info, TokenAccount>,
    target_ata: &Account<'info, TokenAccount>,
    token_program: &Program<'info, Token>,
    amount: u64,
    signer_seeds: Option<&[&[&[u8]]]>,
) -> Result<()> {
    match signer_seeds {
        Some(signer_seeds) => token::transfer(
            CpiContext::new_with_signer(
                token_program.to_account_info().clone(),
                Transfer {
                    from: source_ata.to_account_info().clone(),
                    to: target_ata.to_account_info().clone(),
                    authority: authority.clone(),
                },
                signer_seeds,
            ),
            amount,
        ),
        None => token::transfer(
            CpiContext::new(
                token_program.to_account_info().clone(),
                Transfer {
                    from: source_ata.to_account_info().clone(),
                    to: target_ata.to_account_info().clone(),
                    authority: authority.clone(),
                },
            ),
            amount,
        ),
    }
}
