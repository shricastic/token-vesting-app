import * as anchor from '@coral-xyz/anchor'
import {Program} from '@coral-xyz/anchor'
import {Keypair} from '@solana/web3.js'
import {Tokenvestingapp} from '../target/types/tokenvestingapp'

describe('tokenvestingapp', () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)
  const payer = provider.wallet as anchor.Wallet

  const program = anchor.workspace.Tokenvestingapp as Program<Tokenvestingapp>

  const tokenvestingappKeypair = Keypair.generate()

  it('Initialize Tokenvestingapp', async () => {
    await program.methods
      .initialize()
      .accounts({
        tokenvestingapp: tokenvestingappKeypair.publicKey,
        payer: payer.publicKey,
      })
      .signers([tokenvestingappKeypair])
      .rpc()

    const currentCount = await program.account.tokenvestingapp.fetch(tokenvestingappKeypair.publicKey)

    expect(currentCount.count).toEqual(0)
  })

  it('Increment Tokenvestingapp', async () => {
    await program.methods.increment().accounts({ tokenvestingapp: tokenvestingappKeypair.publicKey }).rpc()

    const currentCount = await program.account.tokenvestingapp.fetch(tokenvestingappKeypair.publicKey)

    expect(currentCount.count).toEqual(1)
  })

  it('Increment Tokenvestingapp Again', async () => {
    await program.methods.increment().accounts({ tokenvestingapp: tokenvestingappKeypair.publicKey }).rpc()

    const currentCount = await program.account.tokenvestingapp.fetch(tokenvestingappKeypair.publicKey)

    expect(currentCount.count).toEqual(2)
  })

  it('Decrement Tokenvestingapp', async () => {
    await program.methods.decrement().accounts({ tokenvestingapp: tokenvestingappKeypair.publicKey }).rpc()

    const currentCount = await program.account.tokenvestingapp.fetch(tokenvestingappKeypair.publicKey)

    expect(currentCount.count).toEqual(1)
  })

  it('Set tokenvestingapp value', async () => {
    await program.methods.set(42).accounts({ tokenvestingapp: tokenvestingappKeypair.publicKey }).rpc()

    const currentCount = await program.account.tokenvestingapp.fetch(tokenvestingappKeypair.publicKey)

    expect(currentCount.count).toEqual(42)
  })

  it('Set close the tokenvestingapp account', async () => {
    await program.methods
      .close()
      .accounts({
        payer: payer.publicKey,
        tokenvestingapp: tokenvestingappKeypair.publicKey,
      })
      .rpc()

    // The account should no longer exist, returning null.
    const userAccount = await program.account.tokenvestingapp.fetchNullable(tokenvestingappKeypair.publicKey)
    expect(userAccount).toBeNull()
  })
})
