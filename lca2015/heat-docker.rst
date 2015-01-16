Deploying to the cloud with golden images, Heat and Docker
==========================================================

Steve Baker <sbaker@redhat.com> @stevebake

Declarative vs Procedural Orchestration
---------------------------------------

- procedural describes steps to execute
- declarative describes desired state

Heat
----

- REST service for declarative orchestration of multi-tenant
  OpenStack cloud services
- multi-tenant = multiple users using same cloud infra
- consumes a number of other OpenStack APIs

Kubernetes
----------

- REST service for declarative orchestration of <del>multi-tenant
  OpenStack cloud services</del> containers
- No OpenStack container API
- Kubernetes/Docker are not multi-tenant APIs

Evolution of Heat software configuration
----------------------------------------

- boot-time config: user-data script + cfn-init metadata,
  cloud-init / cloud-config
- config/deployment resources: shell / puppet / ansible etc

Kubelet
-------

- Process a container manifest so the containers are launched
  according to how they are described

- A *pod* is a collection of containers

What you don't get with kubelet vs full kubernetes
--------------------------------------------------

- no service load balancing
- no scheduler - requires manual placement of pods

Heat template
-------------

- yaml?
- sections; parameter defns, resources, outputs (e.g. IP addrs of
  resultant instances)

Heat architecture
-----------------

- haproxy (optional)
- heat-api (scales horizontally)
- rabbitmq
- heat-engine (scales horizontally)
- mysql

Building the VM image
---------------------

- ``diskimage-builder`` from OOO
- uses ``heat-config-kubelet`` element from the ``heat-templates``
  directory
- currently Fedora only (lots of ``systemd``)
- includes ``tar`` file of docker images for import on boot

Images and Security
-------------------

  ...the code responsible for downloading images is shockingly
  insecure.  Users should only download images whose provenance is
  without question. At present, this does not include the "trusted"
  iamges hosted by Docker, Inc.

    Trevor Jaysecurityblog.redhat.com

Launching the stack
-------------------

- launch heat-standalone template
- heat launches VM with kubelet-embedded image
- heat builds data describing pods to create
- VM agent fetches data, writes out pod template files
- kubelet picks up files, creates containers
- VM agent monitors for container creation, signals Heat with
  results

Lifecycle of container stacks
-----------------------------

- image releases handled with heat stack-update
- container arch changes handled with heat stack-update
- other workflows handled procedurally (with zero or more
  stack-updates)

Evolution of Heat software configuration
----------------------------------------

- config fed to service running on host: docker / kubelet
- (future) config driven heat-provisioned cluster: kubernetes /
  etcd / mesos

Next steps
----------

- expose cAdvisor stats as deployment outputs
- bring up a full Kubernetes cluster with heat, define containers in
  heat template, use unmodified Atomic OS image
- encourage Kubernetes to declare stable interfaces for its
  components

Other container things in OpenStack
-----------------------------------

- Nova Docker driver: GitHub:stackforge/nova-docker
- Heat contrib docker API resource plugin
- OpenStack Magnum multi-tenant container API
  GitHub:stackforge/magnum
- Heat templates for Atomic-based Kubernetes cluster
