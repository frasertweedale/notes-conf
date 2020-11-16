WG cose: CBOR Object Signing and Encryption
===========================================

minutes: https://codimd.ietf.org/notes-ietf-109-cose

Charter
-------

- preference for CFRG algos
- non-preference for state-defined algo

Counter-signatures
------------------

- original spec RFC 8152 had incorrect description of
  countersignature properties.  Decision to write a separate doc to
  respecify countersignatures and obsolete the countersignature spec
  from RFC 8152:
  https://datatracker.ietf.org/doc/draft-ietf-cose-countersign/

- rfc8152bis splits COSE into two primary documents: structures, and
  algorithms.  countersignatures will be kept as a separate doc,
  probably?

CBOR certificates
-----------------

- https://datatracker.ietf.org/doc/html/draft-mattsson-cose-cbor-cert-compress-03

- "make CBOR wine from sour ASN.1 grapes"

- likely direction: CBOR encoding for large subset of X.509 (RFC
  5280) so it works with existing and future X.509 profile.

- make registrations so CBOR certs can be used with TLS

- Compressed X.509 certs, and native (CBOR) X.509 certs
  ``CertificateType`` field tells implementations whether to compute
  signature over X.509 DER structure, or native CBOR structre.

- A considerable number of special cases in DN encoding

  - omit outer array when single RDN

  - omit map structure when RDN consists of single CN attr

  - EUI-64 MAC address CNs seems to be a major driver here,
    compressing into 6 bytes.

- Extensions:
  - special encoding for common ones, otherwise a byte string
    carrying DER-encoded extension

- Typical size reduction is 16-50%
