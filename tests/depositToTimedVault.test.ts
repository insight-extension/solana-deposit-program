import * as anchor from "@coral-xyz/anchor";
import { getAssociatedTokenAddress } from "@solana/spl-token";
import { PublicKey } from "@solana/web3.js";
import { getTokenBalance } from "./utils/helpers";
import { initSetup } from "./utils/setup";

let setup: Awaited<ReturnType<typeof initSetup>>;

beforeAll(async () => {
  setup = await initSetup();
});

test("deposit to timed vault", async () => {
  const { program, user, connection, usdcMint, TOKEN_PROGRAM } = setup;

  let tx: string | null = null;
  try {
    tx = await program.methods
      .depositToTimedVault(new anchor.BN(3_000_000))
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

  const [userInfoAddress] = PublicKey.findProgramAddressSync(
    [Buffer.from("user_timed_info"), user.publicKey.toBuffer()],
    program.programId
  );
  const vault = await getAssociatedTokenAddress(
    usdcMint,
    userInfoAddress,
    true,
    TOKEN_PROGRAM
  );

  const userInfo = await program.account.userTimedInfo.fetch(userInfoAddress);
  expect(userInfo.availableBalance.toNumber()).toEqual(3_000_000);
  expect(await getTokenBalance(connection, vault)).toEqual(
    new anchor.BN(3_000_000)
  );
});
