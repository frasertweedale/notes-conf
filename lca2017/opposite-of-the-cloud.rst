The Opposite of the Cloud
=========================

Tom Eastman (koordinates)


- "Using the cloud" means that you're running critical parts of your
  infra on hardware you don't own

- the opposite: an appliance

- requirements
  - a bootable vm image
  - single step config (or as close as possible)
  - subsequent config via koordinates' website
  - simple firewall requirements

- this kinda sounds like an IoT device
  - something that runs inside your network that you don't control,
    that's configured by an external server that you don't control
  - spidey sense tingling
  - the opposite of the cloud - someone else outsources their
    computing/data onto your infra.

- what do IoT devices get wrong?
  - security patches, firmware updates, software updates, etc
  - don't treat the network as hostile
  - sloppy with HTTPS etc
  - difficult to remotely administer/maintain

- how do you make a secure IoT device?
  - no default/static creds
  - reduce attack surface as much as possible
    - limit what it does.  limit what it can do
  - secure all network comms (privacy *and* authentication)
  - ensure that it fails securely
  - make sure it's easy to update

- solution: don't treat applicances like appliances
  - what it actually needs to be: a tightly orchestrated linux
    server
  - the fact it's on someone else's network is happenstance
  - stick to a conservative architecture
    - I don't know how to build an applicance, but I know Debian
    - use a mainstream distro, minimally customised
    - Turn on unattended updates and trust the process

- solution components:
  - aspen: a customised debian VM image builder build with Packer
  - pando: orchtestration server/C&C network
  - hakea: a Django/REST microservice API in charge

- Saltstack C&C
  - solves two problems: config mgmt and remote execution
  - a config mgmt system capable of maintaining remote nodes in
    defined states
  - distributed remote execution system used to execute commands and
    query data on remote notes
  - salt minion lives on applicance, has persistent TCP connection
    to server (cf Ansible, which is push)

- OpenVPN as Internet transport
  - almost everything locked down; whitelisting

- All the koordinates business logic runs on Docker images
  - decouple platform (VM image) from application
  - decouple deployment of application to appliance, from deployment
    of applicance itself.

- Applicance authentication
  - security through self-destruct (short-lived VPN client certs)
  - if for whatever reason security updates cannot succeed, it will
    be off the network in 24hrs anyway
  - can monitor for lack of customer re-authentication and contact
    them before they contact us!

- Hashicorp Vault: handles PKI.

- Conclusion
  - DevOps principles apply beyond the cloud
