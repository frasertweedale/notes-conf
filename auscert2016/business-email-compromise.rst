Donald Mac McCarthy, VP Operations, myNetWatchman

Social Engineering 2.0

Business email compronmise (BEC)

- phish or web scrape executive email addrs from web
  - e.g. "contact us" pages etc
- take time to understand victim relationships etc

- Web-accessible email is vulnerable
- You don't have to get breached; valid credentials are out there
  (because users don't choose good passwords)
- Triage compromised accounts
  - search for interesting keywords, map relationships
- When somebody gets compromised, check forwarding rulesets!
  - "Password was reset but the keep getting back in!"
  - Will survive password reset.

- Moving money (directly) no longer happens
  - Impersonate target, ask target's banker to move the money for
    them


Look-alike domains

- Emulate counterparties
- Use levenstein difference against common/important domains
  - 1 or 2 is a red flag
  - can extend higher than 2 and whitelist false positive

Email thread redirection

MITE: Man-in-the-Email

- spoof legit email
- set up miscreant email (low levenstein distance)
- set up forwarding rules
- establish credibility, then ask victim for money

MITE: double-duping

- usually for supply chain fraud
- set up spoofed supplier and spoofed customer
- insert themselves directly between real supplier and customer
  - forwarding rules etc

BEC vs malware

- device print matches victim
- malware has big problem with behavioural match
  - BEC doesn't b/c the victim is actually doing the operation

BES miscreants by region:

- 40% west africa
- 10% SE Asia
- N.A. 8%
- Eastern Europe 7%
- Oceania 6%

- It's >$3bn problem globally, >$1.5bn in US alone.
- That's ONLY what's reported; we don't know for sure how bit it is

Where is this headed?

- Threat currently constrained by inefficient / manual setup process
- Remove bottleneck: buy creds from Eastern Europe.
- How to recruit mules?
  - Dating sites

Removing mule bottleneck

- Prepaid cards

Mitigation:

- FIs may not be liable, but threat to rep and loss of key
  relationships is high.
- Bad Beneficiary patterns: China, HK, Malaysia, Poland
  - If you don't normally send money to HK... bank should notice
- Employee education and controls
  - the most important thing you can do
  - positive reinforcement e.g. reward for reporting
    (not expensive even in large orgs)
  - negative reinforcement (monitor; tie to raises, etc)
  - you don't have to spend $$$ on appliances
- Educate customers: payment instructions delievered via email must
  always be validated
- Levenstein comparison of email domains

Questions:

- Would 2FA or client certs prevent this issue?
  - also SPF, DKIM etc
    - useless against look-alike domains
  - pay attention to not just your accounts but customers' etc
