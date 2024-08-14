use anchor_lang::prelude::*;

mod contexts;
mod state;
mod errors;

pub use state::*;
pub use errors::*;

declare_id!("9YwKpEDB1tNWaoYqFiQJ6YeAiYpwndKK7erTXJhKv3Nq");

#[program]
pub mod anchor_nft_staking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
