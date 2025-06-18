use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

pub mod error;
pub mod util;

declare_id!("FstCVaLZ9oFU4rQ4NfMhGoLpYPQMaNzcM81jkJZoUwdB");

#[program]
pub mod faircoding {
    use super::*;
    use crate::util::fill_from_str;
    use anchor_spl::token_interface;

    pub fn create_user(ctx: Context<CreateUserAccount>, github_id: String) -> Result<()> {
        let user = &mut ctx.accounts.user;
        user.github_id = github_id;
        Ok(())
    }

    pub fn create_repo_mint(
        ctx: Context<CreateRepoMint>,
        repo_id: String,
        name: String,
        uri: String,
        version: String,
        dependencies: Vec<String>,
    ) -> Result<()> {
        // let repo = ctx.accounts.repo.load_init();
        let mut repo = ctx.accounts.repo.load_init()?;
        repo.mint = ctx.accounts.mint.key();
        repo.name = fill_from_str(&name)?;
        repo.uri = fill_from_str(&uri)?;
        repo.repo_id = fill_from_str(&repo_id)?;
        repo.version = fill_from_str(&version)?;
        let name_bytes = dependencies
            .iter()
            .map(|s| fill_from_str(s))
            .collect::<Result<Vec<[u8; 32]>>>()
            .unwrap();
        let mut deps_ = [[0u8; 32]; 9895];
        deps_[..name_bytes.len()].copy_from_slice(&name_bytes);
        repo.dependencies = deps_;

        let decimals = 0;
        // Initialize the mint account using token interface
        let cpi_accounts = token_interface::InitializeMint {
            mint: ctx.accounts.mint.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        token_interface::initialize_mint(
            cpi_ctx,
            decimals,
            &ctx.accounts.owner.key(),
            Some(&ctx.accounts.owner.key()),
        )?;

        Ok(())
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

#[account(zero_copy)]
#[derive(Debug, InitSpace)]
pub struct Repo {
    pub mint: Pubkey,     // 32 bytes
    pub name: [u8; 32],   // 4 + 32 bytes
    pub symbol: [u8; 32], // 4 + 8 bytes
    pub uri: [u8; 200],
    pub repo_id: [u8; 16],
    pub version: [u8; 32],
    pub dependencies: [[u8; 32]; 9895],
    pub authority: Pubkey, // 32 bytes
    pub bump: u8,
}

#[derive(Accounts)]
pub struct CreateRepoMint<'info> {
    #[account(mut)]
    /// The owner of the token and custom data (doesn't need to sign for account creation)
    /// CHECK: This account is validated by being set as the authority
    pub owner: UncheckedAccount<'info>,
    /// The payer who funds the account creation (must sign and pay)
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        mint::decimals = 0,
        mint::authority = owner,
        mint::token_program = token_program,
    )]
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        init,
        payer = payer,
        space = 8 + Repo::INIT_SPACE,
        seeds = [b"repo", owner.key().as_ref()],
        bump)]
    pub repo: AccountLoader<'info, Repo>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct CreateRepoToken<'info> {
    #[account(mut)]
    owner: Signer<'info>,
    #[account(mut)]
    payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = owner,
        associated_token::token_program = token_program,
    )]
    pub token_account: InterfaceAccount<'info, TokenAccount>,
    pub mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
