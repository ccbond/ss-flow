import { Transaction, SystemProgram, Connection, PublicKey } from '@solana/web3.js';

/**
 * Creates an arbitrary transfer transaction
 * @param   {String}      publicKey  a public key
 * @param   {Connection}  connection an RPC connection
 * @returns {Transaction}            a transaction
 */
const createTransferTransaction = async (publicKey: PublicKey, connection: Connection): Promise<Transaction> => {

  const program = anchor.workspace.SsFlow as Program<SSFlow>;
  const programId = new program.programId;
  console.log('programId', programId)

  const amount = new anchor.BN(1000);
  const from = new anchor.web3.PublicKey("BSmSnj7wNncYkaZqn3nwHiQgcPYWX3iKvc3mSVpJyRhH");
  const to_account = new anchor.web3.PublicKey("98kCzLGDquUNiE6g162rzgS46VM67xvVok9bP6kwrwvo");

  const instruction = await program.methods.transferSol(amount).accounts({
      from,
      to_account,
      systemProgram: anchor.web3.SystemProgram.programId,
  }).instruction()


  const transaction = new Transaction().add(
    instruction
  );
  transaction.feePayer = publicKey;

  const anyTransaction: any = transaction;
  anyTransaction.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;

  return transaction;
};

export default createTransferTransaction;
