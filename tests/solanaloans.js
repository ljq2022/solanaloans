const anchor = require("@project-serum/anchor");
const chai = require("chai");
const expect = chai.expect;
const { SystemProgram } = anchor.web3;

describe("solanaloans", () => {
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  it("Is able to initialize and create a loan for a user.", async () => {
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
    const programBalance = await program.account.baseAccount.getAccountInfo(
      baseAccount.publicKey
    );
    expect(programBalance.lamports.toString()).equal("8000000000");
  });
  it("Is able to initialize, create a loan for a user, and have them pay it back.", async () => {
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
    await program.rpc.payLoan({
      accounts: {
        baseAccount: baseAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
    });
    const programBalance = await program.account.baseAccount.getAccountInfo(
      baseAccount.publicKey
    );
    expect(programBalance.lamports.toString()).equal("11000000000");
  });
});
