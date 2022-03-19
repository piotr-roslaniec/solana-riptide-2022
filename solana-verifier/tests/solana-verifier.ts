import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SolanaVerifier } from "../target/types/solana_verifier";

describe("solana-verifier", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.SolanaVerifier as Program<SolanaVerifier>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
