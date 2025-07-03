use anchor_client::solana_sdk::signature::Keypair;

#[derive(Serialize)]
pub struct User {
    pub signer: Keypair,
    pub github_id: String,
}
