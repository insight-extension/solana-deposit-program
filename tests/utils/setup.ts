import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { DepositProgram } from "../../target/types/deposit_program";
import { airdropIfRequired } from "@solana-developers/helpers";
import { Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import {
  createAssociatedTokenAccount,
  createMint,
  getAccount,
  getAssociatedTokenAddress,
  mintTo,
  TOKEN_2022_PROGRAM_ID,
} from "@solana/spl-token";

const TOKEN_PROGRAM = TOKEN_2022_PROGRAM_ID;

export const initSetup = async () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const connection = provider.connection;
  const program = anchor.workspace.DepositProgram as Program<DepositProgram>;

  const masterWallet = new PublicKey(
    "71q6LEWUkPZhYChjAcZcuxVVyDqdEyjf95etzte2PzwK"
  );
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
      masterWallet,
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
    masterWallet,
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
      masterWallet,
      null,
      TOKEN_PROGRAM
    );
  }

  const userUsdcBalance = new anchor.BN(20_000_000);
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
    masterWallet,
    usdcMint,
    userUsdcAccount,
    masterWalletUsdcAccount,
    userUsdcBalance,
    TOKEN_PROGRAM,
  };
};
