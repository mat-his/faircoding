use accounts_ix::*;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::example_mocks::solana_signer::Signer;
use anchor_spl::token_2022::{MintTo, TransferChecked};
use anchor_spl::token_interface;
use error::FairCodingError;
use state::{Debt, Dependency};
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
    /// Init -> Mint
    pub fn create_repo(
        ctx: Context<CreateRepoToken>,
        name: String,
        uri: String,
        dependencies: Vec<String>, // list of PubKeys (existing Repos in the system)
        repo_id: String,
        version: String,
    ) -> Result<()> {
        let mut repo = ctx.accounts.repo_data.load_init()?;
        repo.mint = ctx.accounts.mint_account.key();
        repo.token = ctx.accounts.token_account.key();
        repo.name = fill_from_str(&name)?;
        repo.uri = fill_from_str(&uri)?;
        repo.repo_id = fill_from_str(&repo_id)?;
        repo.version = fill_from_str(&version)?;
        let name_bytes = dependencies
            .iter()
            .map(|s| Dependency {
                key: fill_from_str(s).unwrap(),
                rewarded: 0u8,
            })
            .collect::<Vec<Dependency>>();
        let mut deps_ = [Dependency::default(); 36_632];
        deps_[..name_bytes.len()].copy_from_slice(&name_bytes);
        repo.len = dependencies.len() as u16;

        // Create the MintTo struct with the accounts required for the CPI
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint_account.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.owner.to_account_info(),
        };

        // The program being invoked in the CPI
        let cpi_program = ctx.accounts.token_program.to_account_info();

        // Combine the accounts and program into a "CpiContext"
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);

        // Make CPI to mint_to instruction on the token program
        token_interface::mint_to(cpi_context, 1)
    }

    /// Pay one dependency
    ///
    /// Transfer is executed based on `Debt` declared by `compute_deps_fees`
    pub fn pay_deps(ctx: Context<PayDeps>, debt: Debt) -> Result<()> {
        // prepare transfer
        // TODO: transfer from vaults to vaults if available
        let cpi_ctx = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            anchor_lang::system_program::Transfer {
                from: ctx.accounts.signer.to_account_info(),
                to: ctx.accounts.escrow.to_account_info(),
            },
        );
        let res = anchor_lang::system_program::transfer(cpi_ctx, debt.amount);
        // if success, mark the dep as "rewarded" in target repo
        match res {
            Ok(_) => {
                ctx.accounts
                    .repo
                    .load_mut()?
                    .validate(debt.repo_key.to_bytes());
                ctx.accounts.receiver_vault.lamports += debt.amount;
                Ok(())
            }
            // else register debt on signer vault
            Err(_) => {
                ctx.accounts.signer_vault.insert(debt)
                // Err(FairCodingError::RewardError.into())
            }
        }
    }

    /// User withdraw lamports in his Vault
    ///
    /// Vault keeps the amount of lamports a user deserves from the Escrow Account
    /// Only the program have authority to edit vault and escrow data
    pub fn withdraw_vault(ctx: Context<WithdrawVault>) -> Result<()> {
        let user_vault = &mut ctx.accounts.user_vault;
        let amount = user_vault.lamports;

        require!(amount > 0, FairCodingError::NothingToWithdraw);

        // Transfer SOL from escrow to creator
        **ctx.accounts.escrow.try_borrow_mut_lamports()? -= amount;
        **ctx.accounts.owner.try_borrow_mut_lamports()? += amount;

        user_vault.lamports = 0;

        emit!(RoyaltyWithdrawn {
            user: ctx.accounts.owner.key(),
            amount,
        });
        Ok(())
    }
}

#[event]
pub struct RoyaltyWithdrawn {
    pub user: Pubkey,
    pub amount: u64,
}

