# Enforcing privacy rights against big tech and big surveillance

- Dan Shearer

- How an embedded db project called LumoSQL became **EnforceRights**

- Sense of privacy develops from ~3yo.  There are things we do not
  want to share with other people (or most other people).
  - cf anonymity: act (or not) without revealing identity.  related
    but not the same as privacy.
  - cf security: state of being free from danger or threat

- Why do we care about privacy?
  - Is privacy a right?  Yes and no.  In AU law, we do not have a
    constitutional or general statutory right to privacy.
  - AU has signed, ratified and promoted the seven core UN rights
    instruments, but none are part of AU law.
  - Your rights depends on which state you are in and your standing
    in or relationship to the state.
  - So do you have a right?  Less than you might think.  And this is
    a problem.  Most people feel, until it's too late, that they
    have a right to privacy.

- "metadata" is at the core of a quality privacy regime, but
  unfortunatley AU High Court decided that metadata is not PII but
  more about how a service is delivered.

## What is the current situation?

- If I'm in AU, what is the framework available to me?
  - Federal laws, state laws, ongoing Privacy Act review, increasing
    data breach penalties
  - There is no tort of privacy in AU.  Therefore no remedy for
    breach of privacy.
  - Remediation: law and courts are slow.  Not entirely bad, but the
    fact is they can't keep up with the developments.

- Data industry (selling of personal data) is galloping away.
  - High growth industry in AU and Asia.
  - If you're in AU, they're coming for us.

- Even in the "shining light" of EU, the situation is far from
  comfortable.

- There will be a review of AU mandatory metadata retention regime.

- Personal epiphany moment:
  - in the beginning of GDPR doc (2015), it doesn't just talk about
    right to privacy; it is contextualised in a framework of
    fundamental rights (e.g. freedom of thought/expression/religion;
    right to remedy, right to fair trial, culture and language).
  - the rights are interrelated.

- GDPR and similar laws put privacy rights at the heart of
  everything that happens online.
  - "Adequacy" - a concept/framework of privacy law compatibility
  - GDPR started a "race to the top"

- *While the mandatory data retention regime provides critical
  assistance to LE and intelligence services, the regime lacks
  transparency and adquate safeguards.* - Mark Dreyfus (AG) Feb '23.

- AU is currently in favour of and conducting mass surveillance.

## What actions can be taken?

- What can the individual do?
  - We have to think about our technology choices.
  - Actions that everyone can take in their daily lives, to some
    extent.
  - 3 questions:
    - Is the source code available?
    - Does it have end-to-end security?
    - Which countries define the product?
  - 3 imperatives
    - Choose hardware/software carefully
    - Choose cloud suppliers carefully
    - Mix up sources of supply
- Engage with politicians, policy development.  Vote.

## What if we started from the bottom up?

- Tor, Signal, etc are great efforts but many important applications
  and program infrastructure are lacking in privacy controls.

- How do we give user-level privacy control in SQL[ite]?
  - *Attribute-based encryption*
  - Can be fine grained e.g. bank can read the data, but not write.
    The permissions are in the data.
  - Ambitious to the point of hubristic, but what if every record in
    every database had these user privacy controls.
