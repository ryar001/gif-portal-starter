import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Hello } from "../target/types/hello";

describe("hello", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.Hello as Program<Hello>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
