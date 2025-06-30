use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

#[derive(Accounts)]
struct MintRepo<'info> {
    // The mint authority
    #[account(mut)]
    pub signer: Signer<'info>,
    // The mint account
    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,
    // The destination token account
    #[account(mut)]
    pub token_account: InterfaceAccount<'info, TokenAccount>,
    // The token program
    pub token_program: Interface<'info, TokenInterface>,
}
