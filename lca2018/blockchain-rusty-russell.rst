Rusty Russell - Blockstream (ADL)

Future technological directions in bitcoin
==========================================

Won't be talking about layer-2 or hard forks (incompat. changes)

Bitcoin keyword primer:

- TXO: transaction output (amount + output script)
- Input Script (ScriptSig)
- TXID: hash of entire transaction
- UTXO set (unspent transaction set)
  - new transactions can spend (only) these
- Merkle tree with TXIDs at leaves

Proposed consensus changes (soft forks; backward compat change)

- You can make things that are currently legal, illegal
  - but not the other way around

- Segregated witness (already happened)
  - new Output Script
  - input looks "empty" to old nodes (because input script has been
    segreated/moved out of hash input)
  - helps: future changes, layer 2, hw wallets, bloat incentives,
    throughput

- segwit addresses: base32 + 30-bit BCH checksum
  - guaranteed to detect up to 4 errors
  - "similar" letter subst: detect up to 5 errors
  - otherwise 1 in 1e9 chance of error
  - helps: addr reliability, QR codes etc

- MAST (BIP 114)
  - Merkelised Abstract Syntax Tree
  - provide one script + proof it's in tree
  - helps: large input scripts

- MAST (BIP 116)
  - OP_MERKLETREEVERIFY does tree check on the top stack elems

- MAST (BIP 117)
  - execute stack leftovers

- MAST overview:
  - privacy for unused script branches
  - problem 1: MAST inputs are obvious

- Taproot
  - new pay-to-pubkey-hsah
  - users make key use base key & hash of script
  - can spend using key & sig like before
    - or revealing base key + script
  - helps: fungibility

- key aggregation
  - Shnorr-style signatures
  - multisig helps fungibility & size

- sig aggregation
  - theoretical "OP_CHECKAGGSIG"
  - actual sig checking done at end of tx
  - helps: input size, validation speed and coinjoin incentive

- batch sig validation
  - problem: lots of sigs per block
  - helps: initial block download validation

- atomic swaps

- scriptless scripts

- scripts are not provable
  - and hard to statically analyse

- delayed transactions
  - with emergency backup key covenant, for recovery if private
    key leaks

- confidential transactions
  - problem: amounts are public
  - nodes only need to know:
    - sum of input = sum of output + fee; and
    - that there's no overflow
  - "bulletproofs"

- UTXO commitments
  - problem: checking xns is impossible for lite nodes
  - prove UTXO, missing UTXO or do full block check

- TXO commitments
  - old TXOs committed into blocks
  - append-only block-order merkle tree of TX + "spent bit"
  - wallet proves output was in TXO tree
  - proof is sufficient to make TXO as spent
  - but, wallet would need to track TXO proofs

- Fraud proofs:
  - problem: lite nodes simply trust miners
  - what can you prove?
    - tx invalid/malformed + tx and merkle proof it's in block
    - tx can't spend output + above, and input tx
  - what could you prove with fraud proof?
    - that input tx does not exist
  - what could you not prove?
    - can't prove merkle hash is wrong ("data withholding attack")

Non-consensus protocol improvements

- TXO bitfields
  - nodes remember (spent) bitfield for each block
  - wallet's send merkel proof of tx's location in bit block
  - proofs are still big, but proof of TXO *position* is static

- Rolling UTXO set hashes
  - single 256-bit number
  - helps: full nodes, initial node bootstrap

- xn compressor
  - average 28% compression (in isolation)
  - helps: bandwidth, storage

- set reconciliation for tx relay
  - most bandwidth is just nodes announcing that they have an xn id
  - solution: announce to 1 or 2 peers and use set reconciliation

- block propagation takes ~40k
  - compact blocks high/lo bandwidth mode
- block template delta compression
  - average delta is 3K (fits in one packet)

- Dandelion
  - problem: sniffers look for tx origins
  - expose tx to single peer 90%* (stem)
  - 10% pass to all (fluff)
  - helps: fungibility

- Neutrino:
  - current nodes send bloom filter of inputs/outputs they are
    interested in
  - easy to detect what addresses :(
  - requires per-peer calcs on full nodes
  - neutrino inverts this...
  - full node sends block summary ~20k
    - lite nodes pulls interesting block
  - helps: lite node privacy

- NODE_NETWORK_LIMITED (BIP 159)
  - problem: pruned nodes don't advertise
  - helps: reduce bw for full nodes

- Peer encryption and auth (BIP 150/151)
  - problem: all traffic in clear, no easy way to assure comms to
    trusted nodes
  - helps: make passive analysis harder, make trusted nodes easier

- TYPE THEORY "simplicity"

Q&A:

- There are chain analysis companies who do this.
- I believe they are trying to sell the info to places with KYC
  requirements.
