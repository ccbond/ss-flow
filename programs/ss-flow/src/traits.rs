use anchor_lang::prelude::*;

pub trait ReceiveToken<'info> {
    fn receive_a(&self, amount: u64) -> Result<()>;
    fn receive_b(&self, amount: u64) -> Result<()>;
}
