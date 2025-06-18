use anchor_lang::{error_code, prelude::msg};

#[error_code]
pub enum FairCodingError {
    #[msg("Allocated size doesn't match expected size")]
    AllocationError,
}
