import * as anchor from "@coral-xyz/anchor";
import { initSetup } from "./utils/setup";

let setup: Awaited<ReturnType<typeof initSetup>>;

beforeAll(async () => {
  setup = await initSetup();
});

test("insufficient balance", async () => {
  const {
    program,
    user,
    master,
    usdcMint,
    userUsdcAccount,
    connection,
    TOKEN_PROGRAM,
  } = setup;

  let txError: Error | null = null;
  try {
    await program.methods
      .subscribe(new anchor.BN(4_000_000))
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

  // Check if error is thrown and if it matches "Insufficient balance"
  expect(txError).not.toBeNull();
  expect(txError?.message).toContain("Insufficient balance");

  // Verify user's token balance remains unchanged
  const tokenBalance = await connection.getTokenAccountBalance(userUsdcAccount);
  expect(new anchor.BN(tokenBalance.value.amount)).toEqual(
    setup.userUsdcBalance
  );
});
