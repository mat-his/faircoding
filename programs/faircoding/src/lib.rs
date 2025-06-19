use accounts_ix::*;
use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use error::FairCodingError;
use state::Dependency;
use util::fill_from_str;

pub mod accounts_ix;
pub mod error;
// pub mod instructions;
pub mod state;
pub mod util;

declare_id!("FstCVaLZ9oFU4rQ4NfMhGoLpYPQMaNzcM81jkJZoUwdB");

#[program]
pub mod faircoding {
    use super::*;

    /// Create User
    pub fn create_user(ctx: Context<CreateUserAccount>, github_id: String) -> Result<()> {
        let user = &mut ctx.accounts.user;
        user.owner = ctx.accounts.owner.key();
        user.github_id = github_id;
        Ok(())
    }

    /// Create Repo
    ///
    /// Create Repo owned by User and declare all its (Repo) dependencies
    pub fn create_repo(
        ctx: Context<CreateRepoToken>,
        name: String,
        uri: String,
        dependencies: Vec<String>,
        repo_id: String,
        version: String,
    ) -> Result<()> {
        // let repo = ctx.accounts.repo.load_init();
        let mut repo = ctx.accounts.repo.load_init()?;
        repo.mint = ctx.accounts.mint.key();
        repo.token = ctx.accounts.token.key();
        repo.name = fill_from_str(&name)?;
        repo.uri = fill_from_str(&uri)?;
        repo.repo_id = fill_from_str(&repo_id)?;
        repo.version = fill_from_str(&version)?;
        let name_bytes = dependencies
            .iter()
            .map(|s| Dependency {
                key: fill_from_str(s).unwrap(),
                rewarded: 0,
            })
            .collect::<Vec<Dependency>>();
        let mut deps_ = [Dependency::default(); 36_632];
        deps_[..name_bytes.len()].copy_from_slice(&name_bytes);

        Ok(())
    }

    /// Get Dependency
    ///
    /// Whether install or update a repo as a dependency of our own
    pub fn get_dependency(ctx: Context<GetDependency>, amount: u64) -> Result<()> {
        let dep_repo = ctx.accounts.dep_repo.to_account_info();
        let mut repo = ctx.accounts.repo.load_mut()?;

        // check if dep already exist
        if repo.is_rewarded(dep_repo.key()) {
            return Ok(());
        }
        // add new dep
        if repo.find(dep_repo.key()).is_none() {
            repo.insert(dep_repo.key().to_bytes())?;
        }
        // transfer fixed amount
        let res = transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.signer.to_account_info(),
                    to: dep_repo,
                },
            ),
            amount,
        );
        // if success, mark the dep as "rewarded"
        match res {
            Ok(_) => repo.validate(ctx.accounts.dep_repo.key().to_bytes()),
            Err(_) => Err(FairCodingError::RewardError.into()),
        }
    }

    /* pub fn repo_withdraw(ctx: Context<GetDependency>, amount: u64) -> Result<()> {
        // TODO: implement
        Ok(())
    } */
}

// For a 1bps taker fee, set taker_fee to 100, so taker_fee/FEES_SCALE_FACTOR = 10e-4
pub const FEES_SCALE_FACTOR: i128 = 1_000_000;
