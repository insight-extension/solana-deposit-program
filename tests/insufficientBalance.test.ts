import * as anchor from "@coral-xyz/anchor";
import { initSetup } from "./utils/setup";

let setup: Awaited<ReturnType<typeof initSetup>>;

beforeAll(async () => {
  setup = await initSetup();
});

test("insufficient balance", async () => {
  const { program, user, master, usdcMint, TOKEN_PROGRAM } = setup;

  let tx: string | null = null;
  try {
    tx = await program.methods
      .depositToSubscriptionVault(new anchor.BN(3_000_000))
      .accounts({
        user: user.publicKey,
        token: usdcMint,
        tokenProgram: TOKEN_PROGRAM,
      })
      .signers([user])
      .rpc();
  } catch (e) {
    console.log(e);
  }

  expect(tx).not.toBeNull();

  let txError: Error | null = null;
  try {
    await program.methods
      .subscribeWithVault(new anchor.BN(4_000_000))
      .accounts({
        user: user.publicKey,
        token: usdcMint,
        tokenProgram: TOKEN_PROGRAM,
      })
      .signers([master])
      .rpc();
  } catch (error) {
    txError = error;
  }

  // Check if error is thrown and if it matches "Insufficient balance"
  expect(txError).not.toBeNull();
  expect(txError?.message).toContain("Insufficient balance");
});
