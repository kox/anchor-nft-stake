use anchor_lang::prelude::*;
use anchor_spl::{metadata::{mpl_token_metadata::{accounts::Metadata, instructions::{FreezeDelegatedAccountCpi, FreezeDelegatedAccountCpiAccounts, ThawDelegatedAccountCpi, ThawDelegatedAccountCpiAccounts}}, MasterEditionAccount, MetadataAccount}, token::{revoke, Approve, Mint, Revoke, Token, TokenAccount}};
use anchor_spl::token::approve;

use crate::{Config, StakeAccount, UserAccount};


#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    pub mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = payer,

    )]
    pub mint_ata: Account<'info, TokenAccount>,

    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
        ],
        seeds::program = metadata_program.key(),
        bump,
    )]
    pub metadata: Account<'info, MetadataAccount>,

    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
            b"edition",
        ],
        seeds::program = metadata_program.key(),
        bump,
    )]
    pub edition: Account<'info, MasterEditionAccount>,

    pub config: Account<'info, Config>,

    #[account(
        mut,
        seeds = [b"user".as_ref(), payer.key().as_ref()],
        bump = user_account.bump,
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(
        mut,
        close= payer,
        seeds = [b"stake".as_ref(), mint.key().as_ref(), config.key().as_ref()],
        bump = stake_account.bump,
    )]
    pub stake_account: Account<'info, StakeAccount>,

    pub metadata_program: Program<'info, Metadata>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info,  System>,
}

impl<'info> Unstake<'info> {
    pub fn unstake(&mut self) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let delegate = &self.stake_account.to_account_info();
        let token_account = &self.mint_ata.to_account_info();
        let mint = &self.mint.to_account_info();
        let edition = &self.edition.to_account_info();
        let token_program = &self.token_program.to_account_info();
        let metadata_program = &self.metadata_program.to_account_info();
        let config = &self.config.to_account_info();
        
        let now = Clock::get()?.unix_timestamp as i64;

        let thawAccounts = ThawDelegatedAccountCpiAccounts {
            delegate, 
            token_account,
            edition,
            mint,
            token_program,
        };

        ThawDelegatedAccountCpi::new(
            metadata_program,
            thawAccounts
        ).invoke()?;

        let cpi_accounts = Revoke {
            source: self.mint_ata.to_account_info(),
            authority: self.stake_account.to_account_info(),
        };

        let signer_seeds: [&[&[u8]]; 1] = [&[
            b"stake", 
            &mint.key.as_ref(), 
            &config.key.as_ref(),
            &[self.stake_account.bump],
        ]];

        let cpi_context = CpiContext::new_with_signer(
            cpi_program,
            cpi_accounts,
            &signer_seeds
        );

        revoke(cpi_context)?;

        self.user_account.amount_staked -= 1;

        let days_to_secs = 86400;
        let time_elapsed = ((Clock::get()?.unix_timestamp - self.stake_account.last_update / 86400));

        self.user_account.points += time_elapsed as u32 * self.config.points_per_stake as u32;        
        
        Ok(())
    }
}