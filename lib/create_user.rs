use crate::models::User;
use anchor_client::{
    anchor_lang::system_program,
    solana_client::rpc_client::RpcClient,
    solana_sdk::{address_lookup_table, signature::Keypair, signer::Signer},
    Client, Program,
};
use faircoding::{accounts::CreateUserAccount, state::User};

pub async fn create_user(program: Program, user: User) -> Result<()> {
    let new_user = Keypair::new();
    // create user
    let signature = program
        .request()
        .signer(&user.signer.pubkey())
        .accounts(CreateUserAccount {
            owner: user.signer.pubkey(),
            user: new_user.pubkey(),
            payer: program.payer(),
            system_program: system_program::ID,
        })
        .args(faircoding::instruction::CreateUser {
            github_id: user.github_id,
        })
        .send()
        .await?;

    println!("Transaction confirmed: {}", signature);
    pda_key = Pubkey::find_program_address(
        &[b"user", &user.github_id, &user.signer.pubkey()],
        &program.id(),
    );
    let user_account: User = program.account::<User>(pda_key);
    println!("Github ID: {}", user_account.github_id);
    Ok(())
}
