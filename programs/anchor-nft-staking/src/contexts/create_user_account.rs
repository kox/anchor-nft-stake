use anchor_lang::prelude::*;

use crate::state::UserAccount;

#[derive(Accounts)]
pub struct CreateUserAccount<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        seeds = [b"user".as_ref(), payer.key().as_ref()],
        bump,
        space = UserAccount.INIT_SPACE,
    )]
    pub user_account: Account<'info, UserAccount>,

    pub system_program: Program<'info, System>,
}

impl<'info> CreateUserAccount<'info> {
    pub fn create_user_account(
        &mut self,
        bumps: &CreateUserAccountBumps
    ) -> Result<()> {
        self.user_account.set_inner(UserAccount{
            points: 0,
            amount_staked: 0,
            bump: bumps.user_account,
        });

        Ok(())
    }
}