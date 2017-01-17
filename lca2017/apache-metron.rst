Nik Lam, Ned Shawa (Hortonworks)

- "3 Vs": volume, variety, velocity

- Metron features:
  - PCAP, NetFlow, IDS, Email archives, server logs, enrichment data
    - "enrichment data" -> GeoIP and other metadata
  - Stream processing
    - normalisation, enrichment, tagging (alerting) in RT
  - Indexing for visualisation (SIEM-like functionality)
  - Long-term storage in Data Vault
    - forensics; deep discovery

- reduce incident response time
  - everything in one place -> eliminate overhead of manual tasks
  - enriched data provides context, faster understanding of what
    you're dealing with
  - see also "War on Stealth Cyberattacks" on YouTube and Slideshare

- data vault analysis and machine learning
  - UEBA - exploration on large scale using big data analytics e.g.
    Hive, Pig, Spark, Zeppelin/Jupyter
  - build model of "normal" -> anomoly detection

- Apache NiFi
  - developed by NSA
  - "Niagara Files"
  - dataflow management tool for enhanced "situational awareness"

- Data provenance
  - NiFi traces all data from source
  - drag and drop to construct "data stories"; sources,
    "processors", sinks, etc.
