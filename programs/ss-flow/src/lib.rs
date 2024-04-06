use anchor_lang::prelude::*;

declare_id!("8K8ELAz6Q5uvNwQz1iYqn8BnLU1Lf1LFghCTSnBvHAF6");

pub mod instrucions;
pub mod state;

use instrucions::*;

#[program]
pub mod ss_flow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
