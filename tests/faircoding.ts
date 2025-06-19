import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Faircoding } from "../target/types/faircoding";
import { assert } from "chai";

describe("faircoding", () => {
  // Use the cluster and the keypair from Anchor.toml
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  // See https://github.com/coral-xyz/anchor/issues/3122
  const payer = (provider.wallet as anchor.Wallet).payer;
  // const user = (provider.wallet as anchor.Wallet).payer;
  const user = anchor.web3.Keypair.generate();
  const someRandomGuy = anchor.web3.Keypair.generate();

  const program = anchor.workspace.faircoding as Program<Faircoding>;

  // We don't need to airdrop if we're using the local cluster
  // because the local cluster gives us 85 billion dollars worth of SOL
  before(async () => {
    const balance = await provider.connection.getBalance(payer.publicKey);
    const balanceInSOL = balance / anchor.web3.LAMPORTS_PER_SOL;
    const formattedBalance = new Intl.NumberFormat().format(balanceInSOL);
    console.log(`Balance: ${formattedBalance} SOL`);
  });

  it("Create User", async () => {
    const tx = await program.methods
      .createUser("gégé")
      .accounts({ owner: user.publicKey, payer: payer.publicKey })
      .signers([user])
      .rpc();
    console.log("Your transaction signature", tx);
    assert.ok(tx);
    // console.log("User Address", user.publicKey);
    const [userData, _] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("user"), user.publicKey.toBuffer()],
      program.programId
    );
    let account = await program.account.user.fetch(userData);
    console.log("User Account", account);
    assert.isTrue(account !== null);
    assert.equal(account.githubId, "gégé");
    assert.equal(account.owner.toString(), user.publicKey.toString()); // HACK: Pubkey doesn't match without stringify them
  });
});
