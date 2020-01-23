New Phone, Who Dis?: Human Authentication in the Digital Age
============================================================

yaakov, wisetech global

- The best way to find out who someone is is to ask them.
  - but sometimes people lie

identity documents

- describes the subject (just them, or a very small set of people)
- authentic; difficult/impossible to fake

           Personal identity / digital identity 
What        who you are           some account in db
how many?       one           zero to inf.
how to get?  at birth     sign up to any idp
how to prove?   attestation by 3rd party / 



What is a digital drivers' license (DDL)

- like real world one, but on phone
- NSW, SA have it, QLD will do regional trial
- some US states

Positives:

- one less card to carry
- limit access to different readers / verifiers
- wipe remotely if lost
- always up to date
- cryptographic proofs -> more trustworthy

Negatives

- no standard or federated system; everyone rolling their own
- no formal verification

What if...?

- device runs out of juice?
- screen is cracked?
- I have no cellular data?
- the RP (e.g. police) has any of these problems?

What about...?

- interstate or intl licenses?
- intl driver permit system?
- exchanging details e.g. at collision?
- abuse of tech for Police state monitoring?
- privacy implications of any given implementation?
  - turn phone into BLE beacon so the cops know who you are before
    they even step out of their patrol car?
- identify theft and forgeries?

DDL in NSW

- "evidence of the issue of a driver license" 
- you do not have to give your phone to the police!
  - they can look, maybe touch, but definitely not take.
- presenting DDL is exemption from other mobile-phone-while-driving
  laws
- the "Authority" that issues DDL may use and *release* info if for
  the issuance of DDL

Implementation

- Log into "service NSW" with existing / new Gov online services acct
- all kinds of licenses
  - working with children check, fishing license, DDL
- Photo, license number, all the usual stuff, QR code
- scan the QR code -> valid/not valid
  - verifier needs to be online, hits a server to validate
- it's NOT a JWS/JWT
  - section 1: "p":timestamp, "u":uuid, "c":1, "t":1, "s":a sha-384 hash?
  - section 2: base64url 2048-bit binary data
  - section 3: timestamp and license number

Does it meet the requirements?

- describes (only) the target ✓
- secure ?
  - lift a real QR off another license?
    or overlay fake details around your own QR code?
    license check is OK/Not OK; doesn't show license details
  - yes, you can fake it.  they updated the verifier app to show
    name, and to instruct user to also check photo matches bearer,
    etc.

Conclusion

- not a good system
- QLD will use a new ISO standard
- always use most stringent option available to verify a digital
  license
- evaluate digital license tech for yourself; it's different
  everywhere
