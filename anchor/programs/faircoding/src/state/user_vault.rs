use anchor_lang::prelude::*;

use crate::error::ErrorCode::ArrayFull;
use crate::state::Debt;

#[derive(InitSpace)]
#[account]
pub struct UserVault {
    pub owner: Pubkey,
    pub lamports: u64,
    pub debts: [Debt; 36_632],
    pub bump: u8,
    pub len: u16,
}

impl UserVault {
    pub fn insert(&mut self, value: Debt) -> Result<()> {
        let idx = self.len as usize;
        if idx >= self.debts.len() {
            return Err(ArrayFull.into());
        }

        self.debts[idx] = value;
        self.len += 1;
        Ok(())
    }

    pub fn remove(&mut self, ndx: usize) -> Result<()> {
        self.debts[ndx] = Debt::default();
        Ok(())
    }
}
