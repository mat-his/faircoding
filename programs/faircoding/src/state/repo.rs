use crate::{error::ErrorCode, state::Dependency};
use anchor_lang::prelude::*;

#[account(zero_copy)]
#[derive(Debug, InitSpace)]
pub struct Repo {
    pub owner: Pubkey, // 32 bytes
    pub mint: Pubkey,  // 32 bytes
    pub token: Pubkey,
    pub name: [u8; 32],   // 4 + 32 bytes
    pub symbol: [u8; 32], // 4 + 8 bytes
    pub uri: [u8; 200],
    pub repo_id: [u8; 16],
    pub version: [u8; 32],
    pub dependencies: [Dependency; 36_632],
    pub len: u16, // number of elements currently inserted
    pub bump: u8,
    _padding: [u8; 1],
}

impl Repo {
    pub fn insert(&mut self, value: [u8; 32]) -> Result<()> {
        let idx = self.len as usize;
        if idx >= self.dependencies.len() {
            return Err(ErrorCode::ArrayFull.into());
        }

        self.dependencies[idx].key = value;
        self.dependencies[idx].rewarded = 0;
        self.len += 1;
        Ok(())
    }
    pub fn validate(&mut self, key: [u8; 32]) -> Result<()> {
        for i in 0..self.len as usize {
            if self.dependencies[i].key == key {
                self.dependencies[i].rewarded = 1;
                return Ok(());
            }
        }
        Err(ErrorCode::KeyNotFound.into())
    }
    pub fn is_rewarded(&self, key: Pubkey) -> bool {
        let _key = key.to_bytes();
        for i in 0..self.len as usize {
            if self.dependencies[i].key == _key {
                return self.dependencies[i].rewarded == 1;
            }
        }
        false
    }
    pub fn find(&self, key: Pubkey) -> Option<&Dependency> {
        let _key = key.to_bytes();
        for i in 0..self.len as usize {
            if self.dependencies[i].key == _key {
                return Some(&self.dependencies[i]);
            }
        }
        None
    }
    pub fn find_index(&self, key: Pubkey) -> Option<usize> {
        let _key = key.to_bytes();
        for i in 0..self.len as usize {
            if self.dependencies[i].key == _key {
                return Some(i);
            }
        }
        None
    }
}
