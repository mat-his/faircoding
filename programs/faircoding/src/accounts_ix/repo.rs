use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::state::Repo;
#[derive(Accounts)]
#[instruction(repo_id: String, version: String)]
pub struct CreateRepoToken<'info> {
    /// The owner of the token and custom data (doesn't need to sign for account creation)
    /// CHECK: This account is validated by being set as the authority
    #[account(mut)]
    pub owner: UncheckedAccount<'info>,
    /// The payer who funds the account creation (must sign and pay)
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        has_one = owner,
        zero,
        /* payer = payer,
        space = 8 + Repo::INIT_SPACE, */
        seeds = [b"repo", owner.key().as_ref(), repo_id.as_ref(), version.as_ref()],
        bump)]
    pub repo_data: AccountLoader<'info, Repo>,
    #[account(
        init,
        payer = payer,
        mint::decimals = 0,
        mint::authority = owner,
        mint::token_program = token_program,
        seeds = [b"mint", owner.key().as_ref(), repo_id.as_ref(), version.as_ref()],
        bump
    )]
    pub mint_account: InterfaceAccount<'info, Mint>,
    #[account(
        init,
        payer = payer,
        associated_token::mint = mint_account,
        associated_token::authority = owner,
        associated_token::token_program = token_program,
    )]
    pub token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
