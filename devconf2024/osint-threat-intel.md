# With Open Source Intelligence Data in Cyber Threat Intelligence for a Common Purpose

- Julia Kriesch

Agenda

- OSINT vs non-OSINT data
- Cyber threat intel and attribution
- Structured Threat Expression (STIX)
- Threat score calculation
- example of an energy cloud env
- edge intelligence
- federated learning
- homomorphic encryption
- future: central servers for the common purpose (Quantum Computing)


OSINT

- use data from the outside to identify relevant threats
- OSINT tools are designed based on the OSINT cycle

OSINT cycle

1. collection: scan sources and collect data
2. processing: normalise, aggregate, index, correlate
3. analysis: validate sources, exclude manipulated data
4. intelligence dissemination

OSINT data

- publicly available
- accessible without authz
- structured and unstructured

Non-OSINT data

- confidential data / not publicly available
- mostly within orgs
- personal data

Cyber Threat Intelligence (CTI)

- provides insights into potential cyber threads and attacks
- use available info to understand threats to org
- Goal: earlier detection of possible incidents and
  reinforcement/prep of environment

Cyber Threat Attribution

- where, why, how an attack happened
  - identification, what an attacker wanted to achieve (direction,
    timestamp, goals)
- used to prevent future attacks
- convert (un)structured data to intelligence
- High-level IOCs: tactics, techniques, malware
- Low-level IOCs: IP addrs, URLs, hash, domain names, timestamp
- 4 attribute groups: network addr, file hash, file name, other

Structured Threat Intelligence Expression (STIX)

- standardised format for CTI data (OASIS)
- machine and "human-readable" format
- usable for automation
- 4 cyber-threat use cases  
  - analyse cyber threats
  - specify indicators
  - manage response activities
  - share CTI
- OSINF with threat attrs is categorised as STIX Domain Objects (SDO)
- platform Trusted Automation Exchange of Intelligence Information
  (TAXII) can extract threat feeds based on STIX info
- one established platform using STIX and TAXII is MISP (Malware
  Information Sharing Platform)

STIX Domain Objects

- ThreatActor
- campaign (group of activities by threat actors)
- CourseOfAction (recommended next actions)
- ExploitTarget
- Incident
- Indicator
- Observable
- TTP (tactics, techniques, procedures)
- many more

Threat Score

- heuristic analysis to prioritise incidents
- included as custom attributes in the sent IcO within CTI
- process:
  - evaluate threat data
  - calculation of weighted mean / TS
  - math due to Gonzales
- 0..1 = low, 2..3 = medium, 3..4 = high, 5 = critical

*Shows example CTI architecture for an energy provider's cloud env*

Edge Intelligence

- usage of AI in corporation with Edge Computing is called *Edge
  Intelligent / Edge AI*
- main objective is CIA (confidentiality, integrity, availability)
- also covers info privacy applied by AI algos or malfunctions in
  software
- advantage with AI for automated detection of cyber risks
- crypto can be applied to guarantee confidentiality

Federated Learning for CTI

- online/incremental learning from changed and adopted data
  - useful for CTI and Edge Intelligence
- training is distributed via a neural network across multiple
  participants (CTI servers and devices)
- Hao et al. have introduced Private-Enhanced Federated Learning for
  Industrial Artificial Intelligent

Homomorphic Encryption

- all devices assigned to the same secret key and can read encrypted
  data without decrypting first
- AI requires "full homomorphic encryption" (not only adding, but
  also XOR and mult)
- protection of sensitive info, enhancing privacy
- secure voting by Cyber Security Professionals possible
- applying of encrypted data within own network

Idea: global central CTI server

- Sun et al. (2021) suggest OSINT publishing platforms
  - new found issues can be published and pushed (similar to CVE)
- New findings can be converted into machine-readable CTI records
  - colletion of many data
- Dalvi et al. (2023) propose using Quantum Computers as powerful
  and safe systems for solving complex problems for CTI
- **Benefit**: processing CTI data fast (incl. structuring) and
  providing data for pulling

Conclusion

- started with an idea for benefit of all in the enterprise area
- achieve unexpectedly multidiscipline topic!
