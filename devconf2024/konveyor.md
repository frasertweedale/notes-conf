# Modernize Legacy Applications to Kubernetes with Konveyor

- Marek Aufart, Red Hat (OpenShift migrations team)

Outline:

- intro
- Konveyor application
- development
- conclusion

Konveyor is a CNCF sandbox project.

Migration and modernisation

- https://www.konveyor.io/modernization-report/
- Goals: increase security, reliability, scalability, automation,
  CI/CD
- main problem is the complexity of legacy applications
- 6R: retire, retain, **rehost, replatform, refactor**, repurchase
- Rehost: **Forklift** for VMs to kubevirt, **Crane** for containers
- virt-v2v lightning talk today 17:10, A113 (Rich Jones)
- Konveyor's focus: **replatform, refactor**

Konveyor:

> The ultimate Open Source toolkit for help orgs **safely** migrate
> and modernize their application portfolio to leverage Kube and
> cloud-native tech, providing differential value on each stage of
> the adoption process.

Methodology

- assess, rationalise, prepare, pilot, scale
- modernise at scale; unified experience
- https://github.com/konveyor/methodology
- roles: architect, project manager, migrator (dev)

Konveyor application

- k8s operator
- originated in Java-world tools Windup and Pathfinder

Koveyor Hub (central component)

Application inventory:

- portfolio of apps, manage deps, owners/stakeholders
- integration with SCM and bin repos (Git, svn, maven, ...)
- Secure credential storage
- extensible metadata

Application assessment:

- questionnaire-based tool for readiness and risk assessment

Application analysis

- source code and bin analysis
- estimate migration effort (according to numerous supported
  migrations paths)
- warnings, issue analysis and guidance for devs
- technology identification (e.g. what frameworks are used)
- dependency analysis
- delegates complexity analysis via LSP
- rule engine for detecting and advising on migration points
  - custom framework -> write custom rules
- `kantra` CLI
  - run application analysis locally
  - leverage podman to avoid complex installations
  - could be integrated in CI/CD pipline
  - leverages OpenRewrite to automate source code changes

Development

- add support for more languages
- enable Konveyor to retrieve info about apps directly from the
  platform in which they are running, e.g. deployment config,
  runtime config
- asset generation
  - generate all assets required t odeploy an app on k8s (and
    potentially other platforms in future)
  - provide opinionated best practies OOTB

Kai - Koveyor AI

- goal: automate source code changes as much as possible, even when
  apps use custom corp tech and frameworks.  Integrate in IDE.
- Use structured migration data stored in Koveyor to enhance
  commercial LLMs via prompt engineering.
- https://konveyor.io/blog/kai-deep-dive-2024
