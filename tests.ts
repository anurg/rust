import * as anchor from '@coral-xyz/anchor'
import {Program} from '@coral-xyz/anchor'
import {Keypair, PublicKey} from '@solana/web3.js'
import {Journalapp} from '../target/types/journalapp'
import { SolanaProvider } from '@/components/solana/solana-provider'
import JournalappFeature from '@/components/journalapp/journalapp-feature'
import { JournalappIDL } from '@project/anchor'
import { assert } from 'chai';
import crypto from "crypto";

describe('journalapp', () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)
  // const payer = provider.wallet as anchor.Wallet
  const bob = anchor.web3.Keypair.generate();
  const bob_title = "Hello World!";
  const bob_message = "Hello, This is bobs journal entry."
  const program = anchor.workspace.Journalapp as Program<Journalapp>

  // const journalappKeypair = Keypair.generate()

  it('Initialize Journalapp', async () => {
    await airdrop(provider.connection, bob.publicKey);
    const balance = await provider.connection.getBalance(bob.publicKey);
    console.log(balance);
    const [jounalAddress,journalbump] = getJournalAddress(bob_title,bob.publicKey,program.programId);
    console.log(`jounalAddress: ${jounalAddress}`);
    const tx = await program.methods
      .createJournalEntry(bob_title,bob_message)
      .accounts({
        owner: bob.publicKey,
      })
      .signers([bob])
      .rpc({ commitment: "confirmed" });
      console.log(`The transaction id is : ${tx}`);
     await checkJournal(program,jounalAddress,bob.publicKey,bob_title,bob_message,journalbump);
  })

  async function airdrop(connection:any,address:any,amount=1000_000_000) {
    await connection.confirmTransaction( await connection.requestAirdrop(address,amount),"confirmed");
  }

  function getJournalAddress(title: string, owner: PublicKey, programID: PublicKey) {
    return PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode(title),
        owner.toBuffer()
      ], programID);
  }
  async function checkJournal(
    program: anchor.Program<Journalapp>,
    journal: PublicKey,
    owner?: PublicKey,
    title?: string,
    message?: string,
    bump?: number,
  ) {
    let journalData = await program.account.journalEntryState.fetch(journal);
  
    if (owner) {
      assert.strictEqual(journalData.owner.toString(), owner.toString())
  
    }
    if (title) {
      assert.strictEqual(journalData.title, title);
    }
    if (message) {
      assert.strictEqual(journalData.message, message);
    }
   
    // if (bump) {
    //   assert.strictEqual(journalData.bump.toString(), bump.toString())
    // }
  }

  function stringToUtf8ByteArray(inputString: string): Uint8Array {
    const encoder = new TextEncoder();
    return encoder.encode(inputString);
  }
  
  // Function to pad a byte array with zeroes to a specified length
  function padByteArrayWithZeroes(byteArray: Uint8Array, length: number): Uint8Array {
    if (byteArray.length >= length) {
      return byteArray;
    }
  
    const paddedArray = new Uint8Array(length);
    paddedArray.set(byteArray, 0);
    return paddedArray;
  }
  // it('Increment Journalapp', async () => {
  //   await program.methods.increment().accounts({ journalapp: journalappKeypair.publicKey }).rpc()

  //   const currentCount = await program.account.journalapp.fetch(journalappKeypair.publicKey)

  //   expect(currentCount.count).toEqual(1)
  // })

  // it('Increment Journalapp Again', async () => {
  //   await program.methods.increment().accounts({ journalapp: journalappKeypair.publicKey }).rpc()

  //   const currentCount = await program.account.journalapp.fetch(journalappKeypair.publicKey)

  //   expect(currentCount.count).toEqual(2)
  // })

  // it('Decrement Journalapp', async () => {
  //   await program.methods.decrement().accounts({ journalapp: journalappKeypair.publicKey }).rpc()

  //   const currentCount = await program.account.journalapp.fetch(journalappKeypair.publicKey)

  //   expect(currentCount.count).toEqual(1)
  // })

  // it('Set journalapp value', async () => {
  //   await program.methods.set(42).accounts({ journalapp: journalappKeypair.publicKey }).rpc()

  //   const currentCount = await program.account.journalapp.fetch(journalappKeypair.publicKey)

  //   expect(currentCount.count).toEqual(42)
  // })

  // it('Set close the journalapp account', async () => {
  //   await program.methods
  //     .close()
  //     .accounts({
  //       payer: payer.publicKey,
  //       journalapp: journalappKeypair.publicKey,
  //     })
  //     .rpc()

  //   // The account should no longer exist, returning null.
  //   const userAccount = await program.account.journalapp.fetchNullable(journalappKeypair.publicKey)
  //   expect(userAccount).toBeNull()
  // })
})
