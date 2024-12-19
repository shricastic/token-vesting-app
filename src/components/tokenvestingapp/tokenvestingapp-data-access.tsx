'use client'

import { getTokenvestingappProgram, getTokenvestingappProgramId } from '@project/anchor'
import { useConnection } from '@solana/wallet-adapter-react'
import { Cluster, Keypair, PublicKey } from '@solana/web3.js'
import { useMutation, useQuery } from '@tanstack/react-query'
import { useMemo } from 'react'
import toast from 'react-hot-toast'
import { useCluster } from '../cluster/cluster-data-access'
import { useAnchorProvider } from '../solana/solana-provider'
import { useTransactionToast } from '../ui/ui-layout'

export function useTokenvestingappProgram() {
  const { connection } = useConnection()
  const { cluster } = useCluster()
  const transactionToast = useTransactionToast()
  const provider = useAnchorProvider()
  const programId = useMemo(() => getTokenvestingappProgramId(cluster.network as Cluster), [cluster])
  const program = useMemo(() => getTokenvestingappProgram(provider, programId), [provider, programId])

  const accounts = useQuery({
    queryKey: ['tokenvestingapp', 'all', { cluster }],
    queryFn: () => program.account.tokenvestingapp.all(),
  })

  const getProgramAccount = useQuery({
    queryKey: ['get-program-account', { cluster }],
    queryFn: () => connection.getParsedAccountInfo(programId),
  })

  const initialize = useMutation({
    mutationKey: ['tokenvestingapp', 'initialize', { cluster }],
    mutationFn: (keypair: Keypair) =>
      program.methods.initialize().accounts({ tokenvestingapp: keypair.publicKey }).signers([keypair]).rpc(),
    onSuccess: (signature) => {
      transactionToast(signature)
      return accounts.refetch()
    },
    onError: () => toast.error('Failed to initialize account'),
  })

  return {
    program,
    programId,
    accounts,
    getProgramAccount,
    initialize,
  }
}

export function useTokenvestingappProgramAccount({ account }: { account: PublicKey }) {
  const { cluster } = useCluster()
  const transactionToast = useTransactionToast()
  const { program, accounts } = useTokenvestingappProgram()

  const accountQuery = useQuery({
    queryKey: ['tokenvestingapp', 'fetch', { cluster, account }],
    queryFn: () => program.account.tokenvestingapp.fetch(account),
  })

  const closeMutation = useMutation({
    mutationKey: ['tokenvestingapp', 'close', { cluster, account }],
    mutationFn: () => program.methods.close().accounts({ tokenvestingapp: account }).rpc(),
    onSuccess: (tx) => {
      transactionToast(tx)
      return accounts.refetch()
    },
  })

  const decrementMutation = useMutation({
    mutationKey: ['tokenvestingapp', 'decrement', { cluster, account }],
    mutationFn: () => program.methods.decrement().accounts({ tokenvestingapp: account }).rpc(),
    onSuccess: (tx) => {
      transactionToast(tx)
      return accountQuery.refetch()
    },
  })

  const incrementMutation = useMutation({
    mutationKey: ['tokenvestingapp', 'increment', { cluster, account }],
    mutationFn: () => program.methods.increment().accounts({ tokenvestingapp: account }).rpc(),
    onSuccess: (tx) => {
      transactionToast(tx)
      return accountQuery.refetch()
    },
  })

  const setMutation = useMutation({
    mutationKey: ['tokenvestingapp', 'set', { cluster, account }],
    mutationFn: (value: number) => program.methods.set(value).accounts({ tokenvestingapp: account }).rpc(),
    onSuccess: (tx) => {
      transactionToast(tx)
      return accountQuery.refetch()
    },
  })

  return {
    accountQuery,
    closeMutation,
    decrementMutation,
    incrementMutation,
    setMutation,
  }
}
