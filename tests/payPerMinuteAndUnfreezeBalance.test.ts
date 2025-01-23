import * as anchor from "@coral-xyz/anchor";
import { getAssociatedTokenAddress } from "@solana/spl-token";
import { PublicKey } from "@solana/web3.js";
import { getTokenBalance } from "./utils/helpers";
import { initSetup } from "./utils/setup";

let setup: Awaited<ReturnType<typeof initSetup>>;

beforeAll(async () => {
  setup = await initSetup();
});

test("pay per minute and unfreeze balance", async () => {
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

  let tx2: string | null = null;
  try {
    tx2 = await program.methods
      .freezeBalance()
      .accounts({
        user: user.publicKey,
      })
      .signers([master])
      .rpc();
  } catch (error) {
    console.log(`Error: ${error}`);
  }

  const userInfoWithFrozenBalance = await program.account.userInfo.fetch(
    userInfoAddress
  );

  expect(tx2).toBeTruthy();
  expect(userInfoWithFrozenBalance.isBalanceFrozen).toBeTruthy();

  let tx3: string | null = null;
  try {
    tx3 = await program.methods
      .payPerMinuteAndUnfreezeBalance(new anchor.BN(3_000_000))
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

  expect(tx3).toBeTruthy();
  expect(await getTokenBalance(connection, vault)).toEqual(new anchor.BN(0));
});
