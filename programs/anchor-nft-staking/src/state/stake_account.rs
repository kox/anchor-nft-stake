use anchor_lang::prelude::*;


#[account]
pub struct StakeAccount {
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub last_update: u32,
    pub bump: u8,
}

impl StakeAccount  {
    pub const INIT_SPACE: usize = 8 + 8*2 + 4 + 1;   
}
