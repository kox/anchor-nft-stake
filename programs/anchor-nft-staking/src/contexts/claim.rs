use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, metadata::{mpl_token_metadata::{accounts::Metadata, instructions::{FreezeDelegatedAccountCpi, FreezeDelegatedAccountCpiAccounts}}, MasterEditionAccount, MetadataAccount}, token::{Approve, Mint, Token, TokenAccount}};
use anchor_spl::token::approve;

use crate::{Config, StakeAccount, UserAccount};


#[derive(Accounts)]
pub struct Claim<'info> {
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

    pub config: Account<'info, Config>,

    #[account(
        mut,
        seeds = [b"user".as_ref(), payer.key().as_ref()],
        bump = user_account.bump,
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(
        seeds = [b"rewards".as_ref(), config.key().as_ref()],
        bump = config.rewards_bump,
    )]
    pub rewards_mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = rewards_mint,
        associated_token::authority = payer
    )]
    pub reward_ata: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info,  System>,
}

impl<'info> Claim<'info> {
    pub fn claim(&mut self) -> Result<()> {
// minto
    }   
}