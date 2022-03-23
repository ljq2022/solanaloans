const anchor = require("@project-serum/anchor");
const chai = require("chai");
const expect = chai.expect;
const { SystemProgram } = anchor.web3;

describe("solanaloans", () => {
  // Configure the client to use the local cluster.
  // export RUST_LOG=solana_runtime::system_instruction_processor=trace,solana_runtime::message_processor=info,solana_bpf_loader=debug,solana_rbpf=debug
  // solana logs
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  it("Is able to initialize and create two loans for the same user.", async () => {
    console.log("🚀 Starting test...");

    const LAMPORTS_PER_SOL = 1000000000;
    const SOL_AMOUNT = 10;
    const NUM_LAMPORTS = SOL_AMOUNT * LAMPORTS_PER_SOL;

    const program = anchor.workspace.Solanaloans;
    const baseAccount = anchor.web3.Keypair.generate();

    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(
        baseAccount.publicKey,
        NUM_LAMPORTS
      ),
      "confirmed"
    );

    await program.rpc.initialize({
      accounts: {
        baseAccount: baseAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [baseAccount],
    });
    await program.rpc.createLoan({
      accounts: {
        baseAccount: baseAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
    });
    await program.rpc.createLoan({
      accounts: {
        baseAccount: baseAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
    });
    const balance = await program.account.baseAccount.getAccountInfo(
      baseAccount.publicKey
    );
    expect(balance.lamports.toString()).equal("6000000000");
  });
});
