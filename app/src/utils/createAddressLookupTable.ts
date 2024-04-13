import {
  AddressLookupTableProgram,
  Connection,
  PublicKey,
  TransactionMessage,
  VersionedTransaction,
} from '@solana/web3.js';
import { getAccountInfo } from "@solana/spl-token";

import { PhantomProvider } from '../types';
import { signAndSendTransaction } from '.';
import * as anchor from "@coral-xyz/anchor";
import { BN, Program } from "@coral-xyz/anchor";
import { SSFlow } from "../../idls/ss_flow";
import { getOrCreateATA, TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID } from "@saberhq/token-utils";
import { SolanaProvider } from "@saberhq/solana-contrib";

/**
 * 1. Creates an Address Lookup Table Instruction
 * 2. Signs and sends it in a transactionV0
 * 
 * @param   {String}      publicKey  a public key
 * @param   {Connection}  connection an RPC connection
 * @param   {String}  publicKey recent blockhash
 * @returns {[VersionedTransaction, String]} array of transaction
 *          signature and lookup table address
 */
const createAddressLookupTable = async (
  provider: PhantomProvider,
  publicKey: PublicKey,
  connection: Connection,
  blockhash: string
): Promise<[string, PublicKey]> => {

  // get current `slot`
  let slot = await connection.getSlot();

  const program = anchor.workspace.SsFlow as Program<SSFlow>;

  const amount = new BN(10);
  const proportion = new BN(20);

  const baseKeyPair = anchor.web3.Keypair.generate();
  const poolSeed = anchor.utils.bytes.utf8.encode("flow_pool");

  const mintA = new anchor.web3.PublicKey("");
  const mintB = new anchor.web3.PublicKey("");

  const [poolKey] = await anchor.web3.PublicKey.findProgramAddress([poolSeed], program.programId);


  const valutAToken = await getOrCreateAssociatedTokenAccount(connection, keypair, mintA, keypair.publicKey);

  getOrCreateATA({ provider: anchorProvider, mint: mintA, owner: poolKey });
  const valutBToken = await getOrCreateATA({ provider: anchorProvider, mint: mintB, owner: poolKey });

  const tx = await program.methods.initializePool(amount, proportion).accounts({
    base: baseKeyPair.publicKey,
    admin: provider.publicKey,
    pool: poolKey,
    mintA,
    vaultA: valutAToken.address,
    mintB,
    vaultB: valutBToken.address,
    tokenProgram: TOKEN_PROGRAM_ID,
    associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
    systemProgram: anchor.web3.SystemProgram.programId,
    rent: anchor.web3.SYSVAR_RENT_PUBKEY,
  });

  // create an Address Lookup Table
  const [lookupTableInst, lookupTableAddress] = AddressLookupTableProgram.createLookupTable({
    authority: publicKey,
    payer: publicKey,
    recentSlot: slot,
  });

  console.log('lookup table address:', lookupTableAddress.toBase58());

  // To create the Address Lookup Table on chain:
  // send the `lookupTableInst` instruction in a transaction
  const lookupMessage = new TransactionMessage({
    payerKey: publicKey,
    recentBlockhash: blockhash,
    instructions: [lookupTableInst],
  }).compileToV0Message();

  const lookupTransaction = new VersionedTransaction(lookupMessage);
  const lookupSignature = await signAndSendTransaction(provider, lookupTransaction);
  console.log('Sent transaction for lookup table:', lookupSignature);

  return [lookupSignature, lookupTableAddress]
};

export default createAddressLookupTable;
