CFRG: Crypto Forum Research Group
=================================

OPAQUE
------

``draft-irtf-cfrg-opaque``

Slides:
https://datatracker.ietf.org/meeting/109/materials/slides-109-cfrg-opaque-03

- compiler for translating a OPRF, hash function, memory hard
  function (MHF) and authenticated key exchange (AKE) into a strong,
  augmented PAKE

- 2 phases:

  1. registration: clients use password to register PK creds with
     serfver

  2. authn: clients use password to recovery public key creds from
     server and complete an AKE. 

CPace
-----

Abstract::

    This document describes CPace which is a protocol for two parties
    that share a low-entropy secret (password) to derive a strong shared
    key without disclosing the secret to offline dictionary attacks.
    This method was tailored for constrained devices, is compatible with
    any group of both prime- and non-prime order, and comes with a
    security proof providing composability guarantees.

- current work: security analysis, implementation optimisation

ristretto255 and decaf448
-------------------------

Abstract::

   This memo specifies two prime-order groups, ristretto255 and
   decaf448, suitable for safely implementing higher-level and complex
   cryptographic protocols.  The ristretto255 group can be implemented
   using Curve25519, allowing existing Curve25519 implementations to be
   reused and extended to provide a prime-order group.  Likewise, the
   decaf448 group can be implemented using edwards448.

- edwards curves don't give you a prime order group - only prime
  order group times a small cofactor

- leading to protocol-specific tweaks

- security analysis of abstract algorithm doesn't hold for the
  concrete implementations

- decaf and ristretto are an effort to resolve this problem

- construction (due to Mike Hamburg) of a prime order group

- uses a non-prime-order curve internally; no overhead

- canonical, non-malleable encoding of group elements

How?

- 3 families of curves: montgomery, edwards curves, jacobi quartic
  curves

- linking curves via isogenies

Status:

- test vectors have coverage of all edge cases

- no outstanding issues; read to go.


AEAD limits
-----------

https://datatracker.ietf.org/doc/html/draft-irtf-cfrg-aead-limits-01

Abstract::

   An Authenticated Encryption with Associated Data (AEAD) algorithm
   provides confidentiality and integrity.  Excessive use of the same
   key can give an attacker advantages in breaking these properties.
   This document provides simple guidance for users of common AEAD
   functions about how to limit the use of keys in order to bound the
   advantage given to an attacker.  It considers limits in both single-
   and multi-user settings.

Oblivious PRFs using Prime Order Curves
---------------------------------------

Slides:
https://datatracker.ietf.org/meeting/109/materials/slides-109-cfrg-ietf109-voprf-00

- Fundamental part of OPAQUE

- two-party 1-RTT protocol between server and client

- client sends input x, server has secret k.

- y = PRF(k, x)

- oblivious
  - client learns y, nothing about k.
  - server does not learn anything about x or y

- verifiable: VOPRF
  - client can verify y was computed using k
  - server commits to key k and gives proof
