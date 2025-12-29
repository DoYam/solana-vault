import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaVault } from "../target/types/solana_vault";
import { expect } from "chai";
import { PublicKey, SystemProgram, LAMPORTS_PER_SOL } from "@solana/web3.js";

describe("solana-vault", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.solanaVault as Program<SolanaVault>;
  const provider = anchor.AnchorProvider.env();

  const owner = provider.wallet;
  const user = anchor.web3.Keypair.generate();

  let vaultPda: PublicKey;
  let vaultBump: number;

  before(async () => {
    [vaultPda, vaultBump] = await PublicKey.findProgramAddress(
      [Buffer.from("vault"), owner.publicKey.toBuffer()],
      program.programId
    );

    const airdropSignature = await provider.connection.requestAirdrop(
      user.publicKey,
      2 * LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(airdropSignature);
  });

  it("Initializes vault", async () => {
    const tx = await program.methods
      .initialize()
      .accounts({
        vault: vaultPda,
        owner: owner.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    console.log("Initialize transaction signature", tx);

    const vaultAccount = await program.account.vault.fetch(vaultPda);
    expect(vaultAccount.owner.toString()).to.equal(owner.publicKey.toString());
    expect(vaultAccount.totalDeposited.toNumber()).to.equal(0);
    expect(vaultAccount.totalWithdrawn.toNumber()).to.equal(0);
  });

  it("Deposits SOL to vault", async () => {
    const depositAmount = new anchor.BN(0.5 * LAMPORTS_PER_SOL);

    const tx = await program.methods
      .deposit(depositAmount)
      .accounts({
        vault: vaultPda,
        user: user.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([user])
      .rpc();

    console.log("Deposit transaction signature", tx);

    const vaultAccount = await program.account.vault.fetch(vaultPda);
    expect(vaultAccount.totalDeposited.toNumber()).to.equal(
      depositAmount.toNumber()
    );

    const vaultBalance = await provider.connection.getBalance(vaultPda);
    expect(vaultBalance).to.equal(depositAmount.toNumber());
  });

  it("Withdraws SOL from vault", async () => {
    const withdrawAmount = new anchor.BN(0.2 * LAMPORTS_PER_SOL);
    const recipient = anchor.web3.Keypair.generate();

    const initialRecipientBalance = await provider.connection.getBalance(
      recipient.publicKey
    );

    const tx = await program.methods
      .withdraw(withdrawAmount)
      .accounts({
        vault: vaultPda,
        owner: owner.publicKey,
        recipient: recipient.publicKey,
      })
      .rpc();

    console.log("Withdraw transaction signature", tx);

    const vaultAccount = await program.account.vault.fetch(vaultPda);
    expect(vaultAccount.totalWithdrawn.toNumber()).to.equal(
      withdrawAmount.toNumber()
    );

    const finalRecipientBalance = await provider.connection.getBalance(
      recipient.publicKey
    );
    expect(finalRecipientBalance - initialRecipientBalance).to.equal(
      withdrawAmount.toNumber()
    );
  });

  it("Prevents unauthorized withdrawal", async () => {
    const withdrawAmount = new anchor.BN(0.1 * LAMPORTS_PER_SOL);
    const unauthorizedUser = anchor.web3.Keypair.generate();

    try {
      await program.methods
        .withdraw(withdrawAmount)
        .accounts({
          vault: vaultPda,
          owner: unauthorizedUser.publicKey,
          recipient: unauthorizedUser.publicKey,
        })
        .signers([unauthorizedUser])
        .rpc();
      expect.fail("Should have thrown an error");
    } catch (err) {
      expect(err.toString()).to.include("Unauthorized");
    }
  });

  it("Prevents withdrawal with insufficient funds", async () => {
    const vaultBalance = await provider.connection.getBalance(vaultPda);
    const excessiveAmount = new anchor.BN(vaultBalance + 1);
    const recipient = anchor.web3.Keypair.generate();

    try {
      await program.methods
        .withdraw(excessiveAmount)
        .accounts({
          vault: vaultPda,
          owner: owner.publicKey,
          recipient: recipient.publicKey,
        })
        .rpc();
      expect.fail("Should have thrown an error");
    } catch (err) {
      expect(err.toString()).to.include("InsufficientFunds");
    }
  });
});
