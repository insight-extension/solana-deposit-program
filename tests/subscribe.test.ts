import * as anchor from "@coral-xyz/anchor";
import { getAssociatedTokenAddress } from "@solana/spl-token";
import { PublicKey } from "@solana/web3.js";
import { getTokenBalance } from "./utils/helpers";
import { initSetup } from "./utils/setup";

let setup: Awaited<ReturnType<typeof initSetup>>;

beforeAll(async () => {
  setup = await initSetup();
});

test("subscribe", async () => {
  const { program, connection, user, master, usdcMint, TOKEN_PROGRAM } = setup;

  const [userInfoAddress] = PublicKey.findProgramAddressSync(
    [Buffer.from("user_info"), user.publicKey.toBuffer()],
    program.programId
  );
  const vault = await getAssociatedTokenAddress(
    usdcMint,
    userInfoAddress,
    true,
    TOKEN_PROGRAM
  );

  let tx1: string | null = null;
  try {
    tx1 = await program.methods
      .deposit(new anchor.BN(3_000_000))
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

  expect(tx1).toBeTruthy();
  expect(await getTokenBalance(connection, vault)).toEqual(
    new anchor.BN(3_000_000)
  );

  const subscribeAmount = new anchor.BN(1_000_000);
  const durationInSeconds = new anchor.BN(60 * 60 * 24 * 7);

  let tx2: string | null = null;
  try {
    tx2 = await program.methods
      .subscribe(subscribeAmount, durationInSeconds)
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

  expect(tx2).toBeTruthy();

  const vaultBalanceAfter = await getTokenBalance(connection, vault);
  expect(vaultBalanceAfter.toNumber()).toEqual(2_000_000);

  const userInfo = await program.account.userInfo.fetch(userInfoAddress);
  const now = Math.floor(Date.now() / 1000);
  expect(userInfo.subscriptionEndsAt.toNumber()).toBeGreaterThan(now);

  expect(userInfo.subscriptionEndsAt.toNumber()).toBeGreaterThan(
    now + 6 * 24 * 60 * 60
  );
  expect(userInfo.subscriptionEndsAt.toNumber()).toBeLessThan(
    now + 8 * 24 * 60 * 60
  );
});
