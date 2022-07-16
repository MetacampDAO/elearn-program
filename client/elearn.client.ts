import * as anchor from '@project-serum/anchor';
import { BN, Idl, Program, AnchorProvider } from '@project-serum/anchor';
import { Connection, Keypair, PublicKey, SystemProgram } from '@solana/web3.js';
import { Elearn } from '../target/types/elearn';
import { AccountUtils, toBN, isKp } from './common';

export class TradehausClient extends AccountUtils {
  // @ts-ignore
  wallet: anchor.Wallet;
  provider!: anchor.Provider;
  tradehausProgram!: anchor.Program<Tradehaus>;

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
    this.setTradehausProgram(idl, programId);
  }

  setProvider() {
    this.provider = new AnchorProvider(
      this.conn,
      this.wallet,
      AnchorProvider.defaultOptions()
    );
    anchor.setProvider(this.provider);
  }

  setTradehausProgram(idl?: Idl, programId?: PublicKey) {
    //instantiating program depends on the environment
    if (idl && programId) {
      //means running in prod
      this.tradehausProgram = new anchor.Program<Tradehaus>(
        idl as any,
        programId,
        this.provider
      );
    } else {
      //means running inside test suite
      // @ts-ignore
      this.tradehausProgram = anchor.workspace.Tradehaus as Program<Tradehaus>;
    }
  }

  // --------------------------------------- fetch deserialized accounts

  // --------------------------------------- find PDA addresses

  // --------------------------------------- find all PDA addresses

  // --------------------------------------- elearn ixs

}