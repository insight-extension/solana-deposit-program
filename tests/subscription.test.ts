import * as anchor from "@coral-xyz/anchor";
import { airdropIfRequired } from "@solana-developers/helpers";
import { getAssociatedTokenAddress } from "@solana/spl-token";
import { Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import { getTokenBalance } from "./utils/helpers";
import { initSetup } from "./utils/setup";

let setup: Awaited<ReturnType<typeof initSetup>>;

beforeAll(async () => {
  setup = await initSetup();
});

test("subscription", async () => {
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
      .subscribe(new anchor.BN(6_000_000))
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

  // Find addresses for userInfo and vault accounts
  const [userInfoAddress] = PublicKey.findProgramAddressSync(
    [Buffer.from("user_subscription_info"), user.publicKey.toBuffer()],
    program.programId
  );

  const vaultAddress = await getAssociatedTokenAddress(
    usdcMint,
    userInfoAddress,
    true,
    TOKEN_PROGRAM
  );

  const userInfo = await program.account.userInfo.fetch(userInfoAddress);
  expect(userInfo.availableBalance.toNumber()).toEqual(1_000_000);
  // Check vault balance and masterWallet balance
  const vaultBalance = await connection.getTokenAccountBalance(vaultAddress);
  expect(new anchor.BN(vaultBalance.value.amount)).toEqual(
    new anchor.BN(1_000_000)
  );
  expect(await getTokenBalance(connection, masterWalletUsdcAccount)).toEqual(
    new anchor.BN(5_000_000)
  );
  const currentTimestamp = new Date().getTime() / 1000;
  expect(userInfo.expiration.toNumber()).toBeGreaterThan(currentTimestamp);
});
