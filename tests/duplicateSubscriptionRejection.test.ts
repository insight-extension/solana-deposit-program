import * as anchor from "@coral-xyz/anchor";
import { initSetup } from "./utils/setup";

let setup: Awaited<ReturnType<typeof initSetup>>;

beforeAll(async () => {
  setup = await initSetup();
});

test("duplicate subscription rejection", async () => {
  const {
    program,
    user,
    master,
    usdcMint,
    connection,
    masterWalletUsdcAccount,
    TOKEN_PROGRAM,
  } = setup;

  let tx: string | null = null;
  try {
    tx = await program.methods
      .subscribe(new anchor.BN(20_000_000))
      .accounts({
        user: user.publicKey,
        token: usdcMint,
        tokenProgram: TOKEN_PROGRAM,
      })
      .signers([user, master])
      .rpc();
  } catch (error) {
    console.log(`Error: ${error}`);
  }

  expect(tx).not.toBeNull();

  const masterBalance = await connection.getTokenAccountBalance(
    masterWalletUsdcAccount
  );
  expect(new anchor.BN(masterBalance.value.amount)).toEqual(
    new anchor.BN(20_000_000)
  );

  let txError: Error | null = null;
  try {
    await program.methods
      .subscribe(new anchor.BN(20_000_000))
      .accounts({
        user: user.publicKey,
        token: usdcMint,
        tokenProgram: TOKEN_PROGRAM,
      })
      .signers([user, master])
      .rpc();
  } catch (error) {
    txError = error;
  }

  expect(txError).not.toBeNull();
});
