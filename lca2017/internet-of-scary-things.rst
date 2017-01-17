Internet of Scary Things
========================

Christopher Biggs

agenda:

- risks
- faults
- causes
- interlude: pointing and laughing
- buying
- deploying
- building
- the future

risks:

- botnets of compromised video cameras are tip of iceberg
- risk 1: unauthorised access to devices
  - blackmail, burglary, information disclosure
  - threat model: stalkers, celebrity, etc
  - randomware
- would you notice if you were "netflix and chill" and camera in
  smart TV turned on?
- it's a matter of record that NSA paid Amazon $600M to be a
  datacentre for... something.
- medical device tampering
- mass takeover of devices
  - DDoS
  - bitcoin mining
  - Spam
  - other compute theft
- firewalls don't (usually) prevent outbound conns
- the monsters are *inside the room*

faults:

- malicious devices
- "never ascribe to malice that which can be explained by
  stupidity"
  - there's plenty of stupidity to go around
- poor security
- laziness
  - leave debugger access in prod devices
  - default passwords
  - opaque user interfaces
- all this fail is *not* intrinsic to IoT.  We've faced this same
  problem before.  The enemy is *us*.
- fragmentation
  - everything is different
  - no reuse of services, interfaces, etc
  - fixable with right industry action
    - there is a glimmer of hope
- limited maintainability
  - if there's a bug, you probably won't know
  - if you know, there's probably no patch
  - if you want a patch, there's probably noone who cares

causes:

- short product cycles
- fragmentation
- security is hard
- average time to compromise for exposed device vulnerable to Mirai
  botnet: 70s.
- threat surfaces too large
  - there are better OSes than Linux for IoT
- market incentives

interlude:

- Mirai
  - several products, trivially compromised
  - root telnet access via uPNP-requested port
  - sellers did not understand product
- low-end broadband routers
  - root device by embedding shell script in your SSID
  - send UDP packet to enable telnet interface
- lousy mobile apps
  - overloading app names

buying (constructive advice):

- return or bin unacceptable quallity
- demand good security - apply evolutionary pressure to the market
- scan your network
  - you may not even realise your new toaster has WiFi
- port-scan devices
- look for stupid setup procedures
- favour "big 3" framework support
  - curation is beneficial
  - only one hole in your firewall
  - unified management interface
- demand support for open protocols
  - MQTT, etc
  - if vendor has thought about providing this kind of access,
    they're probably a cut above
- check if open source firmware or clients exist
  - repurpose devices to suit your own framework
  - Node-RED can control many devices also
  - HomeBridge gateway enables remote access
- check reviews or teardowns
  - handful of ppl affecting change by pointing and laughing
  - review gamification drives down quality
  - positive reviews often worthless

defensive architecture

- sandbox devices, put on separate network, MAC whitelisting, etc
- turn off or block uPnP
- plan for breaches
  - think about what would happen if a device went rogue
- monitor device behaviour
  - look for new/changed devices
  - learn what ports a device needs
- BYO cloud; use open-source hubs over crapware
- for surveilllance: Zoneminder, Motion, et al.

DIY:

- HomeKit API
  - cons: curated and NDA'd
  - pros: curated and NDA'd
  - at least provide a bridge
  - designed for composability

- Amazon Alexa and AWS IoT
  - excellent security built in
  - you (sort of) don't have to write a UI
  - standard protocols (built on MQTT)

- OCF uPnP and SDP profiles

- no-app setup

- support MQTT
  - this is the glue that holds IoT together
  - IRC for robots

- long-term support
  - make devices self-documenting
  - provide updates or GTFO
  - don't be lazy and stupid

- avoid "hack-in-a-box"
  - don't just ship miniaturised Unix PC
  - security awareness training
  - automate biuld pipeline for dev/prod/test fimwares

Light at end of tunnel:

- industry groups have realised we need to get serious
- standards
  - BITAG (google, Intel, MS, et al)
  - Open Connectivity Foundation
  - Schneier weighs in
    - wants stronger regulation
      - has potentially to go badly wrong
- google "android things" toolkit
  - new but promising
  - standard device profiles
- Apple Homekit
  - standard camera interface
- Amazon Alexa
  - very open (even BYO)
- OCF IoTivity
  - Discovery and Mgmt
  - multi-language support (but not the right langauges)
- resin.io
  - open source
  - Linux and Docker based
  - device management and update
  - more aimed at enterprise/industrial than consumer
- consumer IDS
  - "Fing" mobile app; scan network
    - about to ship hardware device you can install on network

Missing pieces

- network access policy framework
- initial network authentication
  - a standard is needed
- vuln alerting
- patch dist
  - there are some good approaches, but need common standard

Recap

- immature ind. brings confusion, faults, risks
- frameworks: here comes the cavalry.
