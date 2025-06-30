use anchor_lang::prelude::*;
#[derive(Debug, Default, InitSpace)]
#[zero_copy]
pub struct Dependency {
    pub key: [u8; 32],
    pub rewarded: u8,
}
