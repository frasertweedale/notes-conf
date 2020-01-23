Who cares about democracy?
==========================

"The sorry state of Australian election software and what you can do
about it."

Associate Prof. Vanessa Teague


- there are real techniques for providing end-to-end verifiable
  electronic elections (given some assumptions)
  - but the assumptions (about user behaviour) are too strong;
    the systems don't adequately defend against bugs and fraud

- NSW iVote falls far short
  - but don't worry, it'll prob only be used for at most 1/3 the
    votes

- unlike the Senate count

- unlike proposed Vic local gov all-electronic elections
  - which are about to be introduced unless everyone writes to their
    MLC *right now*


End-to-end verifiable elections

- the system should provide evidence that it did the right thing at
  every step
  - and the evidence should be public for verification

- voters verify their votes with code they control
  - proofs of honest vote recording for the voter

- public bulletin board where everyone can check that their vote is
  there

- series of mix servers so nobody knows which output vote is whose
  - proofs of honest mixing on the bulletin board

- decrypted votes
  - proofs of honest decryption on the bulletin board


Mixing and decrypting with proofs:

- includes a *mixnet* which makes a proof that
  - votes are *shuffled* without manipulation
  - votes are *decrypted* correctly

- a cryptographic equivalent of shaking a ballot box and tipping it
  out


SwissPost/Scytl: two errors in the math

- wasn't end-to-end verifiable, but it was supposed to have shuffle
  & decryption proofs

- there were 2 bugs; one in shffling, one in decryption proof
  (joint work with Sarah Jamie Lewis and Olivier Pereira)
  - it was possible to generate a "proof" that passed verification
    while tampering with votes


Chaum-Pedersen proof of equality of discrete logs

- public info: g, public key pk = g^x, ciphertext (C0, C1)
- claim: that (C0,C1) is a valid ElGamal encryption of 1
- fact to prove: ∃x. s.t. C_1=C_0^x and pk = g^x *without revealing x*
- proof:
  - pick random 'a'
  - set B_0 = g^a , B_1 = C_0^a
  - compute c = H(pk, C_1, B_0, B_1)
  - compute z = a + cx
  - proof <- (c,z)
- verification; check:
  - B_0 = g^z . (pk)^(-c)    and
  - B_1 = C_0^z . C_1^(-c)   and
  - c = H(pk, C_1, B_0, B_1)
- mistake: forgot to include C_0 in hash!

- implications:
  - a cheating mixer/decrypter can turn a valid vote into nonsense
    while providing false proof that it decrypted properly
    - needs to collute with cheating client in order to fit the fake
      proof into the mix

End-to-end verifiable elections; limitations and criticism:

- users need to do a lot of careful work to verify
  - verification requires expertise
- if you don't do it properly you can be tricked
- you can (usually) prove how you voted
  - but not always, and usually not in a polling-place system
- subtle bugs can undermine security
- hard to do for preferential voting
- summary: reasonable solutions in a polling place, but remote
  e-voting with adequate safeguards is an unsolved problem.

What does all this have to do with NSW iVote?

- main assumptions:
  - the **voters** verify the votes
  - Voters use some Scytl javascript to vote
  - to verify they use a different closed-source app from same
    company!
  - so you're trusting Scytl/NSWEC (or anyone who compromises them)

- no proofs of honest vote recording for the voter (secret bulletin
  board; only insiders can see what's there)

- single mix server, not a series
  - so you can see how inputs correspond to outputs

- The first bug in SwissPost's shuffle proof affected iVote too

- Swiss made the code available 6mths before election, for analysis
- NSWEC was already using it for early voting
  - "don't worry, Sctyl patched it during the election"

- NDA to see source was for 5 years (did not sign it!)
  - after election, was made available on better terms (45 days;
    signed it)

- Laws about election software really matter
  - Switzerland has really good laws
  - NSW has really bad laws
    - if you tell anyone about weaknesses you can go to jail!


Victoria

- Last year a bill allowing for universal, compulsory, unregulated
  internet voting for all local gov elections passed lower house.
  Call your MLC now to strike the clause!


Part II: Risk limiting audits
-----------------------------

- electronic counting of paper votes, Risk Limiting Audit provides
  rigorous statistical evidence that announced election outcome is
  right.

- if evidence isn't convincing, can fall back to full hand count

- code that was used in pilot in San Francisco:
  - github.com/pbstark/SHANGRLA
  - github.com/michelleblom/audit-irv-cp/tree/raire-branch
  - Any citizen can observe the audit, see the sampled ballots, and
    check the calculations themselves


AU senate count
---------------

- Senate estimates: 2016 JSCEM report recommended independent,
  impartial expert scrutineer be appointed for each state and
  territory.
  - "It was a recommendation only; wasn't mandated by legislation;
    wasn't done"



Why is it so bad?
-----------------

- Swiss internet voting regulations are really good; detailed,
  serious, firmly oriented around transparency, privacy,
  verification
  - they found out about their problems as soon as they released the
    source code, LONG BEFORE THE ELECTION

- California laws mandate audits, including RLAs

- AU sentate scrutiny rules say nothing useful about computerised
  scanning and counting

- So what can we do?
  - Mandate (in legislation) a meaningful statistical audit of the
    paper ballots against the digitised votes, with meaningful
    observation by scrutineers

Summary
-------

- verifiable e-voting in polling place is feasible
- over internet is an unsolved problem; we shouldn't be doing it
- senate count at present provides no evidence of accuracy
  - but would if rigorous statistical audit were mandated.

Speculation
------------

- it's not an open source project yet but...
- crowdsourced amendments to legislation with the chance to vote
  them up or down
- open input into parliamentary questions e.g. estimates, question
  time
- version for teenagers to practice debating what they choose
- Lack of secure Australian digital ID makes it hard.  Ideas welcome.
