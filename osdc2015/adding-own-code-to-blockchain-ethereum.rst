Ethereum and Dapps
==================

Scott Bragg @faulteh

Blockchain + Code = Amazeballs

Tinkering with blockchains and distributed systems.

Blockchains today

- Bitcoin
- alt-cryptocurrencies
- Ripple
- Ethereum

Tangent: distributed systems

- it has been proven that asynchronous sytsems cannot be guaranteed
  to have consistency
- for safety and liveness you need a synchronous system
- consensus
- 2-party consensus e.g. two-phase commit
  - consensus can block on a single node failure
- 3-party consensus
  - netsplits are a problem
- Paxos
  - Widely used: Google (Chubby), Zookeeper
- Raft
  - fault tolerance and performance on par with Paxos
  - easy to understand.  cluster elects leader; server votes once
    per term; followers can call election if heartbeat fails

Tangent: Bitcoin sidechains, private and public blockchains

- Sidechains
  - Rusty Russell is part of Bitstream which is working on Alpha, a
    sidechain protocol for Bitcoin
  - reassign reserved NOOP into new opcode, e.g. SIDECHAIN_VERIFY
    via a BIP
  - BIP = Bitcoin Improvement Process (like PEP)
  - On Bitcoin mainnet, only 4 - 5 specific standard scripts are
    allowed to run (i.e. ones that send transactions)
  - On Bitcoin testnet, more opcodes are available; this is where
    possible future improvements are developed
  - The Bitcoin VM is not turing complete

Public and private blockchains

- Bitcoin and Ethereum are MIT / GPL respectively
- could you trust a blockchain that implemented proprietary / secret
  crypto?
- business has expressed interest in developing blockchain tech for
  own uses.
  - What use is a private blockchain to open source?  It spreads
    adoption.
  - Private chains can be connected later via sidechains

Interacting with the blockchain

- Stack-based VM
- OP_RETURN can push arbitrary data (40 bytes) to be included in
  transaction.
  - This is how people are tracking things other than Bitcoin in the
    Bitcoin blockchain.
  - coinsecrets.org lists stuff in OP_RETURN

Ethereum Frontier

- Decentralised Software Platform
- a blockchain with a turing complete VM
- AGPL
- Live beta
- Blocks minted every 15secs
- Aim to achieve 1s blocks in future
- Proof of Work
  - Aim to move to Proof of Stake

Components:

- Terminology: Dapps = *Distributed apps*
- Ethereum
  - Blockchain and EtherVM for running contracts (your code)
- Whisper
  - Secure messaging system for users and Dapps
- Swarm (planned)
  - Blockchain-linked asset storage for Dapps
- Mist
  - Dapp enabled browser

Tangent: IPFS (Interplanetary file system)

- Written in Go
- Store some files, version them with a hash, explore the FS
- Snapshots

Some ethereum apps

- Prediction markets (augur.net)
- Crowdfunding
- Organisational transparency (boardroom.to)
- Full list: http://dapps.ethercasts.com

Tools

- eth (C++)
- geth (Go)
- ethereum-haskell

Writing contracts

- Solidity (native language)
- Serpent (Python dialect)
- Compile to EtherVM bytecode

Writing apps

- Embark
- Meteor
