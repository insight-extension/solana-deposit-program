import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { DepositProgram } from "../../target/types/deposit_program";
import { airdropIfRequired } from "@solana-developers/helpers";
import { Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";
import {
  createAssociatedTokenAccount,
  createMint,
  getAccount,
  getAssociatedTokenAddress,
  mintTo,
  TOKEN_2022_PROGRAM_ID,
} from "@solana/spl-token";
import "dotenv/config";

const TOKEN_PROGRAM = TOKEN_2022_PROGRAM_ID;

export const initSetup = async () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const connection = provider.connection;
  const program = anchor.workspace.DepositProgram as Program<DepositProgram>;

  const master = Keypair.fromSecretKey(
    Uint8Array.from(JSON.parse(process.env["PRIVATE_KEY"] ?? ""))
  );

  //await airdropIfRequired(
  //  connection,
  //  master.publicKey,
  //  5 * LAMPORTS_PER_SOL,
  //  5 * LAMPORTS_PER_SOL
  //);

  const user = Keypair.generate();

  // Airdrop SOL to user and master wallet
  await Promise.all([
    airdropIfRequired(
      connection,
      user.publicKey,
      5 * LAMPORTS_PER_SOL,
      5 * LAMPORTS_PER_SOL
    ),
    airdropIfRequired(
      connection,
      master.publicKey,
      5 * LAMPORTS_PER_SOL,
      5 * LAMPORTS_PER_SOL
    ),
  ]);

  const usdcMint = await createMint(
    connection,
    user,
    user.publicKey,
    null,
    6,
    Keypair.generate(),
    null,
    TOKEN_PROGRAM
  );

  const userUsdcAccount = await getAssociatedTokenAddress(
    usdcMint,
    user.publicKey,
    false,
    TOKEN_PROGRAM
  );

  try {
    await getAccount(connection, userUsdcAccount, null, TOKEN_PROGRAM);
  } catch {
    await createAssociatedTokenAccount(
      connection,
      user,
      usdcMint,
      user.publicKey,
      null,
      TOKEN_PROGRAM
    );
  }

  const masterWalletUsdcAccount = await getAssociatedTokenAddress(
    usdcMint,
    master.publicKey,
    false,
    TOKEN_PROGRAM
  );

  try {
    await getAccount(connection, masterWalletUsdcAccount, null, TOKEN_PROGRAM);
  } catch {
    await createAssociatedTokenAccount(
      connection,
      user,
      usdcMint,
      master.publicKey,
      null,
      TOKEN_PROGRAM
    );
  }

  const userUsdcBalance = new anchor.BN(100_000_000);
  await mintTo(
    connection,
    user,
    usdcMint,
    userUsdcAccount,
    user.publicKey,
    userUsdcBalance.toNumber(),
    [],
    null,
    TOKEN_PROGRAM
  );

  return {
    provider,
    connection,
    program,
    user,
    master,
    usdcMint,
    userUsdcAccount,
    masterWalletUsdcAccount,
    userUsdcBalance,
    TOKEN_PROGRAM,
  };
};
