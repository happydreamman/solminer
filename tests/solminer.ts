import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Solminer } from "../target/types/solminer";
import { expect, assert, use as chaiUse } from 'chai';
import chaiAsPromised from 'chai-as-promised';
import { User } from "./user";
import {
  initializeProgram,
  deposit,
  compound,
  unstake,
  claimReferral,
  initBlacklist
} from "./instructions";

describe("sol2x", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  
  const admin = new User();
  const user1 = new User();
  const user2 = new User();
  const user3 = new User();

  const user1Invests = [];

  it("Setup", async () => {
    await admin.init(provider);
    await user1.init(provider);
    await user2.init(provider);
    await user3.init(provider);
  });

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await initializeProgram(admin);
    const tx1 = await initBlacklist(admin);
    console.log("Your transaction signature", tx);
    console.log("Your transaction signature", tx1);
  });

  it("User1 deposit 10 SOL!", async () => {
    // Add your test here.
    user1Invests.push(await deposit(user1, 10, user2.publicKey));
  });
  
  it("User1 compound first investment", async () => {
    const tx = await compound(user1, user1Invests[0]);
    console.log("Your transaction signature", tx);
  });
  
  it("User1 unstake first investment before 60 days", async () => {
    const tx = await unstake(user1, user1Invests[0]);
    console.log("Your transaction signature", tx);
  });

  it("User2 claim referral", async () => {
    const tx = await claimReferral(user2);
    console.log("Your transaction signature", tx);
  });
});
