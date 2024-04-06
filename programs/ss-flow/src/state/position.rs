use anchor_lang::prelude::*;

#[account]
pub struct Position {
    pub bump: u8,

    pub authority_nft_mint: Pubkey,

    pub pool: Pubkey,

    pub amount: u64,
}

impl Position {
    pub const LEN: usize = 8 + 32 * 2 + 1;
}
