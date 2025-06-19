use crate::error::ErrorCode;
use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

#[account()]
#[derive(Debug, InitSpace)]
pub struct User {
    pub owner: Pubkey,
    #[max_len(32)]
    pub github_id: String,
    pub spending_limit: u64,
    pub total_spent: u64,
}
