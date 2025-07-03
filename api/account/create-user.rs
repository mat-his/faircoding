use anchor_client::{
    solana_client::rpc_client::RpcClient,
    solana_sdk::{
        commitment_config::CommitmentConfig, native_token::LAMPORTS_PER_SOL, signature::Keypair,
        signer::Signer, system_program,
    },
    Client, Cluster, Program,
};
use anchor_lang::prelude::*;
use faircoding_api::{create_user, User};
use serde_json::json;
use std::rc::Rc;
use vercel_runtime::{run, Body, Error, Request, RequestPayloadExt, Response, StatusCode};

declare_program!("0000");

struct Lambda<'a> {
    program: Program<'a>,
}

impl Lambda {
    pub async fn handler(&self, _req: Request) -> Result<Response<Body>, Error> {
        match _req.json::<User>() {
            Ok(user) => {
                create_user(self.program, user);
                Ok(Response::builder()
                    .status(StatusCode::OK)
                    .header("Content-Type", "application/json")
                    .body(
                        json!({
                          "message": "你好，世界"
                        })
                        .to_string()
                        .into(),
                    )?)
            }
            Ok(None) => Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .header("Content-Type", "application/json")
                .body(
                    json!({
                      "message": "No user specified"
                    })
                    .to_string()
                    .into(),
                )?),
            Err(e) => AccountError::CreateUser,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let connection = RpcClient::new_with_commitment(
        "http://127.0.0.1:8899", // Local validator URL
        CommitmentConfig::confirmed(),
    );
    // TODO: get payer KeyPair
    // [SECTION mock]
    let payer = Keypair::new();
    let counter = Keypair::new();
    println!("Generated Keypairs:");
    println!("   Payer: {}", payer.pubkey());
    println!("   Counter: {}", counter.pubkey());

    println!("\nRequesting 1 SOL airdrop to payer");
    let airdrop_signature = connection.request_airdrop(&payer.pubkey(), LAMPORTS_PER_SOL)?;

    // Wait for airdrop confirmation
    while !connection.confirm_transaction(&airdrop_signature)? {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    println!("   Airdrop confirmed!");
    // [ENDSECTION mock]

    // Create program client
    let provider = Client::new_with_options(
        Cluster::Localnet,
        Rc::new(payer),
        CommitmentConfig::confirmed(),
    );
    let program = provider.program(example::ID)?;
    lambda = Lambda { program };
    run(|e| lambda.handler(e)).await
}

enum AccountError {
    CreateUser,
}
