import { Connection, PublicKey } from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";

export const getTokenBalance = async (
  connection: Connection,
  tokenAccountAddress: PublicKey
): Promise<anchor.BN> => {
  const tokenBalance = await connection.getTokenAccountBalance(
    tokenAccountAddress
  );
  return new anchor.BN(tokenBalance.value.amount);
};
