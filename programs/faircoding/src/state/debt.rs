use anchor_lang::prelude::Pubkey;

use anchor_lang::prelude::*;
#[derive(Debug, Default, InitSpace, AnchorSerialize, AnchorDeserialize)]
#[zero_copy]
pub struct Debt {
    pub repo_key: Pubkey,
    pub dep_key: Pubkey,
    pub amount: u64,
}
