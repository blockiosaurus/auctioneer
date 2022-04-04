import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Auctioneer } from "../target/types/auctioneer";

describe("auctioneer", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.Auctioneer as Program<Auctioneer>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
