use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;

#[account]
pub struct Pool {
    pub base: Pubkey,

    pub admin: Pubkey,

    pub amount: u64,

    pub mint_a: Pubkey,

    pub vault_a: Pubkey,

    pub mint_b: Pubkey,

    pub vault_b: Pubkey,

    pub position_count: u64,

    pub proportion: u64,
}

impl Pool {
    pub const LEN: usize = 32 * 6 + 8 * 3;
    pub fn seeds(&self) -> [&[u8]; 3] {
        [&b"Pool"[..], self.base.as_ref(), self.bump.as_ref()]
    }
}
