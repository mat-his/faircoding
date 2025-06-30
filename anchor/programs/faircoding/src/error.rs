use anchor_lang::{error_code, prelude::msg};

#[error_code]
pub enum FairCodingError {
    #[msg("Allocated size doesn't match expected size")]
    AllocationError,
    #[msg("Failed to reward repo")]
    RewardError,
    #[msg("Nothing to withdraw from the user vault")]
    NothingToWithdraw,
}
#[error_code]
pub enum ErrorCode {
    #[msg("The array is full.")]
    ArrayFull,
    #[msg("Key not found.")]
    KeyNotFound,
}
