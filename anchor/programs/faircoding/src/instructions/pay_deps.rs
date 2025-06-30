use anchor_lang::{
    prelude::{AccountInfo, AccountLoader},
    Key,
};

use crate::state::{Debt, Repo};

// For a 1bps taker fee, set taker_fee to 100, so taker_fee/FEES_SCALE_FACTOR = 10e-4
// TODO: settle on fair price (max. 3$)
pub const BASE_REPO_REWARD: u64 = 10_000;

pub fn compute_deps_fees(repo: &AccountLoader<Repo>, deps: &[AccountInfo]) -> Vec<Debt> {
    let mut debts: Vec<Debt> = Vec::new();
    for dependency in deps {
        // TODO: computation based on depth
        debts.push(Debt {
            repo_key: repo.key(),
            dep_key: dependency.key(),
            amount: 1,
        });
    }
    debts
}
