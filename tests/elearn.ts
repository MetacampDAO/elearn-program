import * as anchor from "@project-serum/anchor";

import {ElearnClient} from '../client/elearn.client';
import chai, { assert, expect } from 'chai';
import chaiAsPromised from 'chai-as-promised';

chai.use(chaiAsPromised);

describe("elearn", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const ec = new ElearnClient(provider.connection, provider.wallet as any)

  const manager1 = anchor.web3.Keypair.generate(); // can create cert + batch
  const manager2 = anchor.web3.Keypair.generate(); // can create cert only


  it("Airdrops to manager1", async () => {
    // airdrop to manager1
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(manager1.publicKey, 1000000000),
      "confirmed"
    );
    // airdrop to manager2
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(manager2.publicKey, 1000000000),
      "confirmed"
    );
  })

  it("Initialise manager", async () => {
    const [managerPDA, managerBump] = await ec.findManagerProofPDA(provider.wallet.publicKey);
    await ec.initializeManager(
      provider.wallet.publicKey,
      managerPDA,
      managerBump
    );

    const managerPDAAcc = await ec.fetchManagerProofAcc(managerPDA);

    assert.equal(managerPDAAcc.managerKey.toBase58(), provider.wallet.publicKey.toBase58());

    assert.ok(managerPDAAcc.batchCount.toNumber() == 0);
    assert.ok(managerPDAAcc.certificateCount.toNumber() == 0);
    assert.ok(managerPDAAcc.managerBump == managerBump);

    const correctPermsBit = 1 << 0 | 1 << 1 | 1 << 2 | 1 << 3;
    assert.ok(managerPDAAcc.permissionType == correctPermsBit);
  });

  it ("Adds manager", async () => {
    const [adminPDA, _] = await ec.findManagerProofPDA(provider.wallet.publicKey);
    const [manager1PDA, manager1Bump] = await ec.findManagerProofPDA(manager1.publicKey);
    const [manager2PDA, manager2Bump] = await ec.findManagerProofPDA(manager2.publicKey);

    await ec.addManager(
      provider.wallet.publicKey,
      adminPDA,
      manager1.publicKey,
      manager1PDA,
      manager1Bump
    );

    await ec.addManager(
      provider.wallet.publicKey,
      adminPDA,
      manager2.publicKey,
      manager2PDA,
      manager2Bump
    );

    const manager1ProofAcc = await ec.fetchManagerProofAcc(manager1PDA);

    assert.equal(manager1ProofAcc.managerKey.toBase58(), manager1.publicKey.toBase58());

    assert.ok(manager1ProofAcc.batchCount.toNumber() == 0);
    assert.ok(manager1ProofAcc.certificateCount.toNumber() == 0);
    assert.ok(manager1ProofAcc.managerBump == manager1Bump);

    const correctPermsBit = 1 << 0 | 1 << 1 | 1 << 2;
    assert.ok(manager1ProofAcc.permissionType == correctPermsBit);
  })

  it ("Fails to add manager if no perms", async () => {
    const tempManager = anchor.web3.Keypair.generate();
    const [manager1PDA, _] = await ec.findManagerProofPDA(manager1.publicKey);
    const [managerPDA, managerBump] = await ec.findManagerProofPDA(tempManager.publicKey);
    await expect(ec.addManager(
      manager1,
      manager1PDA,
      tempManager.publicKey,
      managerPDA,
      managerBump
    )).to.be.rejectedWith("AnchorError occurred. Error Code: WrongPermission. Error Number: 6003. Error Message: wrong permission type.")
  })

  it ("Modifies permissions", async () => {
    const [adminPDA, _adminBump] = await ec.findManagerProofPDA(provider.wallet.publicKey);
    const [manager2PDA, _manager2Bump] = await ec.findManagerProofPDA(manager2.publicKey);

    await ec.modifyManager(
      provider.wallet.publicKey,
      adminPDA,
      manager2.publicKey,
      manager2PDA,
      1 << 0 | 1 << 1
    )

    const manager2ProofAcc = await ec.fetchManagerProofAcc(manager2PDA);

    const correctPermsBit = 1 << 0 | 1 << 1;
    assert.ok(manager2ProofAcc.permissionType == correctPermsBit);
  })

  it ("Fails to modify perms given invalid perms", async () => {
    const [adminPDA, _adminBump] = await ec.findManagerProofPDA(provider.wallet.publicKey);
    const [manager2PDA, _manager2Bump] = await ec.findManagerProofPDA(manager2.publicKey);

    await expect(ec.modifyManager(
      provider.wallet.publicKey,
      adminPDA,
      manager2.publicKey,
      manager2PDA,
      1 << 4
    )).to.be.rejectedWith("AnchorError occurred. Error Code: InvalidPermission. Error Number: 6002. Error Message: invalid permission passed.")
  })

  it ("Fails to modify perms when not authorised", async () => {
    const [manager1PDA, _manager1Bump] = await ec.findManagerProofPDA(manager1.publicKey);
    const [manager2PDA, _manager2Bump] = await ec.findManagerProofPDA(manager2.publicKey);

    await expect(ec.modifyManager(
      manager1,
      manager1PDA,
      manager2.publicKey,
      manager2PDA,
      1 << 0
    )).to.be.rejectedWith("AnchorError occurred. Error Code: WrongPermission. Error Number: 6003. Error Message: wrong permission type.")
  })

  it ("Creates 2 new batches", async () => {
    const [manager1PDA, _manager1Bump] = await ec.findManagerProofPDA(manager1.publicKey);
    const [manager1batch0PDA, manager1batch0Bump] = await ec.findNewBatchPDA(manager1.publicKey);
    await ec.createBatch(
      manager1,
      manager1PDA,
      "BACKEND DEVELOPMENT WITH RUST (BATCH 0)",
    );
    
    const manager1batch0Acc = await ec.fetchBatchAcc(manager1batch0PDA);
    let manager1ProofAcc = await ec.fetchManagerProofAcc(manager1PDA);

    assert.equal(manager1batch0Acc.managerKey.toBase58(), manager1.publicKey.toBase58())
    assert.ok(Number(manager1batch0Acc.certificateCount) == 0)
    assert.ok(Number(manager1batch0Acc.batchNum) == 0)
    assert.equal(manager1batch0Acc.batchName, "BACKEND DEVELOPMENT WITH RUST (BATCH 0)")
    assert.ok(Number(manager1batch0Acc.batchBump) == manager1batch0Bump)

    assert.ok(Number(manager1ProofAcc.batchCount) == 1)

    const [manager1batch1PDA, manager1batch1Bump] = await ec.findNewBatchPDA(manager1.publicKey);
    await ec.createBatch(
      manager1,
      manager1PDA,
      "BACKEND DEVELOPMENT WITH RUST (BATCH 1)",
    );
    
    const manager1batch1Acc = await ec.fetchBatchAcc(manager1batch1PDA);
    manager1ProofAcc = await ec.fetchManagerProofAcc(manager1PDA);

    assert.equal(manager1batch1Acc.managerKey.toBase58(), manager1.publicKey.toBase58())
    assert.ok(Number(manager1batch1Acc.certificateCount) == 0)
    assert.ok(Number(manager1batch1Acc.batchNum) == 1)
    assert.equal(manager1batch1Acc.batchName, "BACKEND DEVELOPMENT WITH RUST (BATCH 1)")
    assert.ok(Number(manager1batch1Acc.batchBump) == manager1batch1Bump)

    assert.ok(Number(manager1ProofAcc.batchCount) == 2)
  })

  it ("Fails to create batch when not authorized", async () => {
    const [manager2PDA, _manager2Bump] = await ec.findManagerProofPDA(manager2.publicKey);
    const [manager2batch0PDA, manager2batch0Bump] = await ec.findNewBatchPDA(manager2.publicKey);
    await expect(ec.createBatch(
      manager2,
      manager2PDA,
      "BACKEND DEVELOPMENT WITH RUST (BATCH 0)",
    )).to.be.rejectedWith("AnchorError occurred. Error Code: WrongPermission. Error Number: 6003. Error Message: wrong permission type.")
  })
  
});
