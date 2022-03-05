import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Proj } from "../target/types/proj";

describe("proj", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.Proj as Program<Proj>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
