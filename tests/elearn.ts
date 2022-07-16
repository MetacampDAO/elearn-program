import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Elearn } from "../target/types/elearn";

import {ElearnClient} from '../client/elearn.client';
import { assert } from "chai";

describe("elearn", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const ec = new ElearnClient(provider.connection, provider.wallet as any)

  it("Is initialized!", async () => {
    // Add your test here.
    const [managerPDA, managerBump] = await ec.findManagerPDA(provider.wallet.publicKey);
    await ec.initializeManager(
      provider.wallet.publicKey,
      managerPDA,
      managerBump
    );

    const managerPDAAcc = await ec.fetchManagerPDAAcc(managerPDA);

    assert.equal(managerPDAAcc.managerKey.toBase58(), provider.wallet.publicKey.toBase58());

    assert.ok(managerPDAAcc.batchCount.toNumber() == 0);
    assert.ok(managerPDAAcc.certificateCount.toNumber() == 0);
    assert.ok(managerPDAAcc.managerBump == managerBump);

    const correctPermsBit = 1 << 0 | 1 << 1 | 1 << 2 | 1 << 3;
    assert.ok(managerPDAAcc.permissionType == correctPermsBit);
  });
});
