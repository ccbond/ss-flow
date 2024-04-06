use anchor_lang::prelude::*;

// ! Event emitted
use crate::*;

/// Event emitted when initialize a pool.
#[event]
pub struct InitializePoolEvent {
    pub admin: Pubkey,
    pub pool: Pubkey,
    pub token_a: Pubkey,
    pub token_b: Pubkey,
    pub amount: u64,
    pub proportion: u64,
}

#[event]
pub struct AddTokenAEvent {
    pub payer: Pubkey,
    pub pool: Pubkey,
    pub token_a: Pubkey,
    pub amount: Pubkey,
}

#[event]
pub struct RefundTokenBEvent {
    pub payer: Pubkey,
    pub pool: Pubkey,
    pub token_b: Pubkey,
    pub amount: Pubkey,
}

/// Event emitted when initialize a position.
#[event]
pub struct WithdrawTokenAEvent {
    pub payer: Pubkey,
    pub pool: Pubkey,
    pub authority_nft_mint: Pubkey,
    pub locked_nft: Pubkey,
    pub remain_amount: u64,
}

/// Event emitted when withdraw from a pool.
#[event]
pub struct SettleEvent {
    pub pool: Pubkey,
    pub admin: Pubkey,
    pub token_a: Pubkey,
    pub token_b: Pubkey,
    pub amount_a: u64,
    pub amount_b: u64,
}
