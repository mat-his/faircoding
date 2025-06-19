use anchor_lang::prelude::*;
use anchor_spl::token_interface::TokenInterface;

use crate::state::Repo;

#[derive(Accounts)]
pub struct GetDependency<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub receiver: Signer<'info>,
    #[account(mut)]
    pub repo: AccountLoader<'info, Repo>,
    #[account(
        mut,
        constraint = receiver.key() == dep_repo.load()?.owner.key(),
        seeds = [b"repo", receiver.key().as_ref(), dep_repo.load()?.repo_id.as_ref(), dep_repo.load()?.version.as_ref()],
        bump
    )]
    pub dep_repo: AccountLoader<'info, Repo>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}
