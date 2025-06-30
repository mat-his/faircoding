use anchor_lang::prelude::*;
use anchor_spl::token_interface::TokenInterface;

use crate::state::{Repo, UserVault};
#[derive(Accounts)]
pub struct PayDeps<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub receiver: Signer<'info>,
    #[account(mut)]
    pub repo: AccountLoader<'info, Repo>,
    /* #[account(
        mut,
        constraint = receiver.key() == dep_repo.load()?.owner.key(),
        seeds = [b"repo", receiver.key().as_ref(), dep_repo.load()?.repo_id.as_ref(), dep_repo.load()?.version.as_ref()],
        bump
    )]
    pub dep: AccountLoader<'info, Repo>, */
    #[account(
        init_if_needed,
        payer = signer,
        space = 8 + UserVault::INIT_SPACE,
        seeds = [b"user-vault", repo.load()?.owner.as_ref()],
        bump
    )]
    pub signer_vault: Account<'info, UserVault>,
    #[account(
        init_if_needed,
        payer = signer,
        space = 8 + UserVault::INIT_SPACE,
        seeds = [b"user-vault", repo.load()?.owner.as_ref()],
        bump
    )]
    pub receiver_vault: Account<'info, UserVault>,
    #[account(mut)]
    pub escrow: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}
