use anchor_client::{
    solana_client::rpc_client::RpcClient,
    solana_sdk::{
        commitment_config::CommitmentConfig, native_token::LAMPORTS_PER_SOL, signature::Keypair,
        signer::Signer, system_program,
    },
    Client, Cluster, Program,
};
use anchor_lang::prelude::*;
use faircoding_api::{create_repo, create_user, repo_id::RepoIdRepository, User};
use serde_json::json;
use std::rc::Rc;
use vercel_runtime::{run, Body, Error, Request, RequestPayloadExt, Response, StatusCode};

declare_program!("FstCVaLZ9oFU4rQ4NfMhGoLpYPQMaNzcM81jkJZoUwdB");

struct Lambda<'a> {
    program: Program<'a>,
}

impl Lambda {
    pub async fn handler(&self, _req: Request) -> Result<Response<Body>, Error> {
        match _req.json::<RepositoryEvent>() {
            Ok(event) => {
                if event.action == RepositoryAction::Created {
                    create_repo(self.program, event.repository);
                } else if event.action == RepositoryAction::Edited {
                } else if event.action == RepositoryAction::Transferred {
                } else if event.action == RepositoryAction::Renamed {
                } else if event.action == RepositoryAction::Deleted {
                } else {
                    Ok(Response::builder()
                        .status(StatusCode::OK)
                        .header("Content-Type", "application/json")
                        .body(json!({"message": "你好，世界"}).to_string().into())?)
                }
            }
            Ok(None) => Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .header("Content-Type", "application/json")
                .body(json!({
                  "message": "Payload is missing"
                }))?),
            Err(JsonPayloadError::Parsing(err)) => {
                if err.is_data() {
                    package_publishing_handler()
                }
                if err.is_syntax() {
                    Ok(Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .header("Content-Type", "application/json")
                        .body(json!({
                          "message": "JSON Syntax is incorrect"
                        }))?)
                }
                FairCodingApiError::EventParsing(err)
            }
            Err(e) => FairCodingApiError::EventParsing(e),
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

enum RepositoryAction {
    Archived,
    Created,
    Deleted,
    Edited,
    Privatized,
    Publicized,
    Renamed,
    Transferred,
    Unarchived,
}

struct RepositoryEvent {
    pub action: RepositoryAction,
    // entreprise: Option<String>,
    // installation: Option<String>,
    // organisation: Option<String>,
    pub repository: RepoIdRepository,
    pub sender: String,
}
