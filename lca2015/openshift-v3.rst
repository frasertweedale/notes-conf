Connection Containers: Building a PaaS with Docker and Kubernetes

- Katie Miller @codemiller
- Steve Pousty @TheSteveO

Use cases
---------

- developer has awesome idea; writes code
- what does it take to get that app running somewhere

Deployment
----------

- "want and see"; ask for hardware, get it, rack & stack,
  and bah! still env issues
- insufficient resources? go back to 1
- black ops; "production" but not the kind you tell everyone about
- "friends in high places"

Aims
----

- freedom to choose best tool for job, without config headaches
- fast, easy, reproducible deployments
- scales pieces of solution independently according to demand
- end dev vs ops battles; automated deployment pipeline / CD
- security and ability to respond quickly to issues

PaaS
====

- dev focuses on application code; the rest is managed
- OpenShift
  - Origin is free software; Apache 2; manage yourself
  - Enterprise; custom private solutions by Red Hat
  - Online; public instance; probably second or third biggest public
    PaaS; >1M apps


Reasons to rebuild
------------------

- act on lessons from past three years
- new tech to build on, including Docker

The new stack
-------------

- Container host (RHEL/Atomic)
- Container system (Docker)


Atomic
======

- Driven be Red Hat
- New infrastructure (virt, containers) and change in thinking
- RPM-OSTree
  - atomic updates of system, with rollback
  - all the system utilities you need, and nothing else
- Containers are first-class citizens
  - If you want Postfix or Apache, bring them in as containers
  - No shared dependencies
- Cockpit: mgmt interface
- Wins: fast boot, container mgmt and security


Docker
======

- A *container* is a running instance of an *image*
  - Images are the new idea Docker brought
- Based on linux containers
- Images are layered in a unionfs, and layers are shared

Container operations
--------------------

- Instantiate container from image with ``docker run``
- List containers with ``docker ps``
- Rejoin containers with ``docker attach``
- Diff result of running commands in a container against the
  original image; export new images.
- Containers on same host can be *linked* to each other

Docker: pros and cons
---------------------

Pros:

- app portability
- fast boot
- efficient resource usage
- BYO bits; no reliance on OS lib versions

Cons:

- host-centric solution
- no higher-level provisioning (similar to VM)
- no built-in usage tracking and reporting


Kubernetes
==========

- Greek word for "helmsman" or "pilot"
- "a system for managing containerised applications across multiple
  hosts"
- Declarative; declare design endstate and Kube makes it happen
- open source proj by Google
- pre-prod beta
- GitHub: GoogleCloudPlatform/kubernetes


Terminology and architecture
----------------------------

- pod: colo group of Docker containers that share IP and storage
  volumes

- service: provides a single, stable name for a set of pods and acts
  as basic load balancer

- replication controller: manages the lifecycle of pods and ensures
  specified number are running

- label: used to organise and select groups of objects


Components
----------

- cluster: compute resources on top of which containers are built

- node: Docker host running ``kubelet`` (node agent) and *proxy*
  services (fka *minion*)

- master: hosts cluster-level control services including API server,
  scheduler and replication controller manager

- etcd: distributed KV store used to persist Kubernetes system state


Wins
----

- runtime and operational mgmt of containers

- manage related containers as a unit

- container communication across hosts (unlike ``docker link``)

- HA and scalability through automated deployment and monitoring of
  pods and their replicas, across hosts


Rebuilding OpenShift
====================

Design goal:

- Kube provides container runtime

- OpenShift provides DevOps and application environment


Concepts
--------

- application: one or more pods linked together by services;
  distinct, interconnected components

- config: collection of objs describing combination of pods,
  services, replication controllers, environment vars and other
  components

- template: parameterised version of config for reuse

- build config: obj defining source code URI, authn for change
  notifications (webhooks) and build type (*source-to-image* or
  *docker-builder*)

- deployment: image and settings for it: replication controller,
  trigger policies and deployment strategy

- project: namespaces around some of these to support team
  environments


Features
--------

- build, manage and deliver application descriptions at scale
- turn source code into new deployable components
- support for common workflows, application lifecycles and team
  environments
  - integration of CI/CD flows into Kube; trigger on code, app or
    image changes
  - support for projects and multi-user images


Wins
----

- single artifact that contains dependency chain for **reproducible
  deployments**
- share common tech stacks and patterns for rolling out changes
- efficient mgmt of thousands of apps, auto-scaling components
  individually and updating en masse
- easily provision new resources at scale and subdivide them for teams
- responsive, change-aware platform supports fault-tolerant,
  automated and repeatable


Conclusion
==========

- Rough roadmap
  - more work on developer experience
  - first release between may..august

- It's the Linux story again; building solution from best of breed
  parts
  - We are also using OpenVSwitch and Fabric

- Your world as sysadmin or dev is looking bright

- openshift.github.org
