use crate::error::FairCodingError;
use anchor_lang::prelude::*;

pub fn fill_from_str<const N: usize>(name: &str) -> Result<[u8; N]> {
    let name_bytes = name.as_bytes();
    // // TODO: declare errors
    require!(name_bytes.len() <= N, FairCodingError::AllocationError);
    let mut name_ = [0u8; N];
    name_[..name_bytes.len()].copy_from_slice(name_bytes);
    Ok(name_)
}
