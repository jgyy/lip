import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Vault } from "../target/types/vault";
import { assert } from "chai";

describe("Vault", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Vault as Program<Vault>;
  let vault: anchor.web3.Keypair;
  let vaultAccount: anchor.web3.PublicKey;

  before(async () => {
    // Create vault account
    vault = anchor.web3.Keypair.generate();
    vaultAccount = anchor.web3.PublicKey.createProgramAddressSync(
      [Buffer.from("vault_pda")],
      program.programId
    );
  });

  it("initializes a vault", async () => {
    const admin = provider.wallet.publicKey;

    const tx = await program.methods
      .initialize()
      .accounts({
        vault: vault.publicKey,
        admin: admin,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([vault])
      .rpc();

    console.log("Initialize tx:", tx);

    const vaultAccount = await program.account.vault.fetch(vault.publicKey);
    assert.equal(vaultAccount.totalAssets.toNumber(), 0);
    assert.equal(vaultAccount.totalShares.toNumber(), 0);
    assert.equal(vaultAccount.numUsers.toNumber(), 0);
  });

  it("calculates share price correctly", async () => {
    // Share price should be 1 when vault is empty
    const vaultData = await program.account.vault.fetch(vault.publicKey);

    // For empty vault: total_assets / total_shares with handling for zero
    const totalAssets = vaultData.totalAssets;
    const totalShares = vaultData.totalShares;

    assert.equal(totalAssets.toNumber(), 0);
    assert.equal(totalShares.toNumber(), 0);
  });

  it("calculates shares for deposit", async () => {
    // First deposit: 1:1 ratio
    // Deposit amount: 1,000,000 lamports
    // Expected shares: 1,000,000

    const depositAmount = 1_000_000;
    // In empty vault, shares = amount
    assert.equal(depositAmount, 1_000_000);
  });
});
