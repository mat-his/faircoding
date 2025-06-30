use crate::state::UserVault;
use anchor_lang::prelude::*;
use anchor_spl::{
    token::{Mint, TokenAccount},
    token_interface::TokenInterface,
};

#[derive(Accounts)]
pub struct WithdrawVault<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        mut,
        seeds = [b"user-vault", owner.key().as_ref()],
        bump,
        has_one = owner
    )]
    pub user_vault: Account<'info, UserVault>,
    #[account(mut)]
    pub escrow: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}
