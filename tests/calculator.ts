import * as anchor from "@coral-xyz/anchor";
import { Program, web3, BN } from "@coral-xyz/anchor";
import { Calculator } from "../target/types/calculator";
import { assert } from "chai";


describe("Solana Calculator Program:", () => {
  // Configure the client to use the local cluster.

  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);

  const program = anchor.workspace.Calculator as Program<Calculator>;
  const owner = provider.wallet.publicKey
  const calculator = web3.PublicKey.findProgramAddressSync(
    [Buffer.from('Calculator'), owner.toBuffer()],
    program.programId
  )[0]

  it("Is initialized!", async () => {
    const tx = await program.methods.initCalculator().rpc();
    console.log("Your transaction signature", tx);
  })


  // Add your tests here.
  it('Does some operations', async () => {
    const add2 = await program.methods
      .doOperation({ add : true  }, new BN(5))
      .accounts({ owner, calculator })
      .instruction()


    const mul3 = await program.methods
      .doOperation({ mul: true }, new BN(4))
      .accounts({ owner, calculator })
      .instruction()


    const sub1 = await program.methods
      .doOperation({ sub: true }, new BN(2))
      .accounts({ owner, calculator })
      .instruction()

    const tx = new web3.Transaction()
    tx.add(add2, mul3, sub1)
    await provider.sendAndConfirm(tx)
  })


  // Make sure our calculator is secure
  it('Prevents fraudulent transactions', async () => {
    let hackerman = new web3.Keypair()

    let shouldFail = await program.methods
      .resetCalculator()
      .accounts({
        owner: hackerman.publicKey,
        calculator,
      })
      .instruction()

    let tx = new web3.Transaction()
    tx.add(shouldFail)
    await provider
      .sendAndConfirm(tx, [hackerman])
      .then(() => assert.ok(false)) // Error on success, we want a failure
      .catch(() => {})
  })
});
