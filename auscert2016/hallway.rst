- Sat with some infrastructure people from different AU universities
  for lunch.  One of them mentioned that they had moved from RHEL
  to Oracle Linux and lamented the lack of technologies like
  Satellite.  Nice to know that our techs are being perceived well.

- Good feedback on talk

- RSA cryptographer offered to analyse tang algo

- Oracle cryptographer proposed different algo:
  - encrypt secret with random sym key
  - DH for secure chan


  Client                      Server
  m                           B ∈ G
  k ∈ sym
  c <- Enc(k, m)
  c' <- Enc(b, k)

  E ∈ G
                c', e ->
                             k <- Enc-1(B, c')
                             k' <- e^B
                             c'' <- Enc(k', k)
                 <- c''
  k' <- b^E
  k <- Enc-1(k', c'')
  m <- Enc-1(k, c)

  Analysis:
    - attacker at server learns 'k', sufficient for
      successful offline attack of client.
    - eavesdropper can replay c' in new rec req to
      learn k; sufficient for offline attack of client.

- Question about applicability of MR to situations where
  one party *forced* to go to another in order to get access to
  secret data.  Applicability to law, contracts, succession?

  Example: govt agency forced to obtain cooperation of court to
  unseal data.

- Stephen Wilson, Lockstep

  Cool PKI solution for first responder credentials, US Govt grant.
  Wants sub-CAs and issuance of constrained signing certs to
  users/devices.  Credentials are policy OIDs

  Need to get in touch.  Bring in nkinder.

- QPS: ~90% of fraud squad work is currently cryptolocker et al.


Booths
======

Dell
----

Was surprised to see Dell in IdM space.  They bought Quest a few
years ago.


Cyberark
--------

Secret management and sudo integration.

Has a "request access" feature that puts ticket into ticket system
e.g. ServiceNow.  Interesting idea.
