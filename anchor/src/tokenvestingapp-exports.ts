// Here we export some useful types and functions for interacting with the Anchor program.
import { AnchorProvider, Program } from '@coral-xyz/anchor'
import { Cluster, PublicKey } from '@solana/web3.js'
import TokenvestingappIDL from '../target/idl/tokenvestingapp.json'
import type { Tokenvestingapp } from '../target/types/tokenvestingapp'

// Re-export the generated IDL and type
export { Tokenvestingapp, TokenvestingappIDL }

// The programId is imported from the program IDL.
export const TOKENVESTINGAPP_PROGRAM_ID = new PublicKey(TokenvestingappIDL.address)

// This is a helper function to get the Tokenvestingapp Anchor program.
export function getTokenvestingappProgram(provider: AnchorProvider, address?: PublicKey) {
  return new Program({ ...TokenvestingappIDL, address: address ? address.toBase58() : TokenvestingappIDL.address } as Tokenvestingapp, provider)
}

// This is a helper function to get the program ID for the Tokenvestingapp program depending on the cluster.
export function getTokenvestingappProgramId(cluster: Cluster) {
  switch (cluster) {
    case 'devnet':
    case 'testnet':
      // This is the program ID for the Tokenvestingapp program on devnet and testnet.
      return new PublicKey('coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF')
    case 'mainnet-beta':
    default:
      return TOKENVESTINGAPP_PROGRAM_ID
  }
}
