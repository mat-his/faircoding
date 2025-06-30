fn initialize_repo() -> Repo {
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
            rewarded: [0u8; 32],
        })
        .collect::<Vec<Dependency>>();
    let mut deps_ = [Dependency::default(); 36_632];
    deps_[..name_bytes.len()].copy_from_slice(&name_bytes);
    repo
}

fn mint_repo() {}

/// Add Dependency
///
/// install a repo as a dependency of our own
// TODO: WRONG !
pub fn add_dependency(ctx: Context<AddDependency>, depth: u32) -> Result<()> {
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

    let cpi_ctx = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        anchor_lang::system_program::Transfer {
            from: ctx.accounts.signer.to_account_info(),
            to: ctx.accounts.escrow.to_account_info(),
        },
    );
    let payment_amount = BASE_REPO_REWARD / 2u64.pow(depth);
    let res = anchor_lang::system_program::transfer(cpi_ctx, payment_amount);
    // if success, mark the dep as "rewarded"
    match res {
        Ok(_) => {
            repo.validate(ctx.accounts.dep_repo.key().to_bytes(), [1u8; 32]);
            ctx.accounts.user_vault.lamports += payment_amount;
            Ok(())
        }
        Err(_) => Err(FairCodingError::RewardError.into()),
    }
}

/// Pay Royalties for specified repo
///
/// Actually deposing money to Repo's owner vault account
fn reward_repo() {}

fn estimate_repo_fees() {}
