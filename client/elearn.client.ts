import * as anchor from '@project-serum/anchor';
import { BN, Idl, Program, AnchorProvider } from '@project-serum/anchor';
import { Connection, Keypair, PublicKey, SystemProgram } from '@solana/web3.js';
import { Elearn } from '../target/types/elearn';
import { AccountUtils, toBN, isKp } from './common';

export class ElearnClient extends AccountUtils {
  // @ts-ignore
  wallet: anchor.Wallet;
  provider!: anchor.Provider;
  elearnProgram!: anchor.Program<Elearn>;

  constructor(
    conn: Connection,
    // @ts-ignore
    wallet: anchor.Wallet,
    idl?: Idl,
    programId?: PublicKey
  ) {
    super(conn);
    this.wallet = wallet;
    this.setProvider();
    this.setElearnProgram(idl, programId);
  }

  setProvider() {
    this.provider = new AnchorProvider(
      this.conn,
      this.wallet,
      AnchorProvider.defaultOptions()
    );
    anchor.setProvider(this.provider);
  }

  setElearnProgram(idl?: Idl, programId?: PublicKey) {
    //instantiating program depends on the environment
    if (idl && programId) {
      //means running in prod
      this.elearnProgram = new anchor.Program<Elearn>(
        idl as any,
        programId,
        this.provider
      );
    } else {
      //means running inside test suite
      // @ts-ignore
      this.elearnProgram = anchor.workspace.Elearn as Program<Elearn>;
    }
  }

  // --------------------------------------- fetch deserialized accounts

  async fetchManagerProofAcc(managerPDA: PublicKey) {
    return this.elearnProgram.account.manager.fetch(managerPDA);
  }

  // --------------------------------------- find PDA adsdresses

  async findManagerProofPDA(managerKey: PublicKey) {
    return await PublicKey.findProgramAddress(
      [Buffer.from(anchor.utils.bytes.utf8.encode("manager-seed")), managerKey.toBytes()],
      this.elearnProgram.programId
    )
  }

  // --------------------------------------- find all PDA addresses

  // --------------------------------------- elearn ixs

  async initializeManager (
    master: PublicKey| Keypair,
    managerProof: PublicKey,
    managerBump: number,
  ) {
    const signers  = [];
    if (isKp(master)) signers.push(<Keypair>master)

    const masterPk = isKp(master)? (<Keypair>master).publicKey: master;
    const txSig = await this.elearnProgram.methods.initializeManager(
      managerBump
    ).accounts({
      master: masterPk as any,
      managerProof,
      systemProgram: SystemProgram.programId
    }).signers(signers)
    .rpc();

    return { txSig };
  }

  async addManager (
    admin: PublicKey| Keypair,
    adminProof: PublicKey,
    managerKey: PublicKey,
    managerProof: PublicKey,
    managerBump: number, 
  ) {
    const signers  = [];
    if (isKp(admin)) signers.push(<Keypair>admin)

    const adminPk = isKp(admin)? (<Keypair>admin).publicKey: admin;
    const txSig = await this.elearnProgram.methods.addManager(managerBump)
      .accounts({
        admin: adminPk as any,
        adminProof,
        managerKey,
        managerProof,
        systemProgram: SystemProgram.programId
      }).signers(signers)
      .rpc();

      return { txSig };
  }

}