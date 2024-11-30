import * as anchor from "@coral-xyz/anchor";
import { getTokenBalance } from "./utils/helpers";
import { initSetup } from "./utils/setup";

let setup: Awaited<ReturnType<typeof initSetup>>;

beforeAll(async () => {
  setup = await initSetup();
});

test("pay per time", async () => {
  const {
    program,
    user,
    master,
    usdcMint,
    TOKEN_PROGRAM,
    connection,
    masterWalletUsdcAccount,
  } = setup;

  let tx: string | null = null;
  try {
    tx = await program.methods
      .depositToTimedVault(new anchor.BN(1_000_000))
      .accounts({
        user: user.publicKey,
        token: usdcMint,
        tokenProgram: TOKEN_PROGRAM,
      })
      .signers([user])
      .rpc();
  } catch (error) {
    console.log(`Error: ${error}`);
  }

  expect(tx).not.toBeNull();

  let tx2: string | null = null;
  try {
    tx2 = await program.methods
      .payPerTime(new anchor.BN(1_000_000))
      .accounts({
        user: user.publicKey,
        token: usdcMint,
        tokenProgram: TOKEN_PROGRAM,
      })
      .signers([master])
      .rpc();
  } catch (error) {
    console.log(`Error: ${error}`);
  }

  expect(tx2).not.toBeNull();
  expect(await getTokenBalance(connection, masterWalletUsdcAccount)).toEqual(
    new anchor.BN(1_000_000)
  );
});
