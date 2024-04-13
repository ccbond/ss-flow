import * as anchor from "@coral-xyz/anchor";
import { BN, Program } from "@coral-xyz/anchor";
import { SSFlow } from "../idls/ss_flow";
import { getOrCreateATA, TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID } from "./utils";

describe("ss-flow", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider();

  const program = anchor.workspace.SsFlow as Program<SSFlow>;

  it("initializePool", async () => {
    // Add your test here.
    const amount = new BN(10);
    const proportion = new BN(20);

    const baseKeyPair = anchor.web3.Keypair.generate();
    const poolSeed = anchor.utils.bytes.utf8.encode("flow_pool");

    const mintA = new anchor.web3.PublicKey("");
    const mintB = new anchor.web3.PublicKey("");

    const [poolKey] = await anchor.web3.PublicKey.findProgramAddress([poolSeed], program.programId);

    const valutAToken = await getOrCreateATA({ provider: provider, mint: mintA, owner: poolKey });
    const valutBToken = await getOrCreateATA({ provider: provider, mint: mintB, owner: poolKey });

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

    console.log("Your transaction signature", tx);
  });

  it("")
});
