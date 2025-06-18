use std::collections::HashMap;

use crate::error::ErrorCode;
use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};
use bytemuck::Zeroable;

pub mod error;
pub mod util;

declare_id!("FstCVaLZ9oFU4rQ4NfMhGoLpYPQMaNzcM81jkJZoUwdB");

#[program]
pub mod faircoding {
    use super::*;
    use crate::{error::FairCodingError, util::fill_from_str};
    use anchor_spl::token_interface;

    pub fn create_user(ctx: Context<CreateUserAccount>, github_id: String) -> Result<()> {
        let user = &mut ctx.accounts.user;
        user.github_id = github_id;
        Ok(())
    }

    pub fn create_repo(
        ctx: Context<CreateRepoToken>,
        name: String,
        uri: String,
        dependencies: HashMap<String, bool>,
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
            .map(|s| DataEntry {
                key: fill_from_str(s.0).unwrap(),
                rewarded: if *s.1 { 1 } else { 0 },
            })
            .collect::<Vec<DataEntry>>();
        let mut deps_ = [DataEntry::default(); 36_632];
        deps_[..name_bytes.len()].copy_from_slice(&name_bytes);
        // repo.dependencies = deps_;

        Ok(())
    }

    pub fn get_dependency(ctx: Context<GetDependency>, amount: u64) -> Result<()> {
        /// Whether install or update a repo as a dependency of our own
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
}

// For a 1bps taker fee, set taker_fee to 100, so taker_fee/FEES_SCALE_FACTOR = 10e-4
pub const FEES_SCALE_FACTOR: i128 = 1_000_000;

// #[derive(Accounts)]
#[account()]
#[derive(Debug, InitSpace)]
pub struct User {
    #[max_len(32)]
    pub github_id: String,
}

#[derive(Accounts)]
pub struct CreateUserAccount<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = 8 + User::INIT_SPACE,
        seeds = [b"user", owner.key().as_ref()],
        bump)]
    pub user: Account<'info, User>,
    pub system_program: Program<'info, System>,
}

#[derive(Debug, Default, InitSpace)]
#[zero_copy]
pub struct DataEntry {
    pub key: [u8; 32],
    pub rewarded: u8,
}

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
    pub dependencies: [DataEntry; 36_632],
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
    pub fn find(&self, key: Pubkey) -> Option<&DataEntry> {
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

#[derive(Accounts)]
#[instruction(repo_id: String, version: String)]
pub struct CreateRepoToken<'info> {
    /// The owner of the token and custom data (doesn't need to sign for account creation)
    /// CHECK: This account is validated by being set as the authority
    #[account(mut)]
    pub owner: UncheckedAccount<'info>,
    /// The payer who funds the account creation (must sign and pay)
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        has_one = owner,
        zero,
        /* payer = payer,
        space = 8 + Repo::INIT_SPACE, */
        seeds = [b"repo", owner.key().as_ref(), repo_id.as_ref(), version.as_ref()],
        bump)]
    pub repo: AccountLoader<'info, Repo>,
    #[account(
        init,
        payer = payer,
        mint::decimals = 0,
        mint::authority = owner,
        mint::token_program = token_program,
        seeds = [b"mint", owner.key().as_ref(), repo_id.as_ref(), version.as_ref()],
        bump
    )]
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        init,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = owner,
        associated_token::token_program = token_program,
    )]
    pub token: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

// TODO:
// [x] join the two accounts into only one
// get to know various the constraints to apply to the accounts
// seeds the repo_id + version (coming fro isntruction parameter) to generate token/mint PDA
// get inspiration from dev.to & solana playground
//
#[derive(Accounts)]
pub struct GetDependency<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub receiver: Signer<'info>,
    #[account(mut)]
    pub repo: AccountLoader<'info, Repo>,
    #[account(
        mut,
        constraint = receiver.key() == dep_repo.load()?.owner.key(),
        seeds = [b"repo", receiver.key().as_ref(), dep_repo.load()?.repo_id.as_ref(), dep_repo.load()?.version.as_ref()],
        bump
    )]
    pub dep_repo: AccountLoader<'info, Repo>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}
