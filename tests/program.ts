import * as anchor from "@project-serum/anchor";
import { Solminer } from "../target/types/solminer";

const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);
const program = anchor.workspace.Solminer as anchor.Program<Solminer>;

export const getProgram = () => {
  return program;
};
