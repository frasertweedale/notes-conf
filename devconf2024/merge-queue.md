# The Merge Queue - the missing piece of CI/CD

- Charly Laurent, Mergify

A brief history of collaborative development

- `cp`
- manual versioning/snapshots
- SCCS (1970s), CVS (1980s), SVN (~2000), Git (2000s)
- ~95% of developers today use Git
- Emergence of PR/MR workflow

A brief history of CI/CD

- CI = automated integration test workflow
- CD = continuous deployment
  - popularised due to agile practices
- "Shift Left" = push testing/QE earlier in dev lifecycle

Build and automation servers

- Jenkins, circleci, travis, ...

The advent of DevOps

- CI/CD services everywhere, for every price
- run tests on machine in the same way as in cloud, thanks to
  containers
- Software is delivered automatically.  Now what?

Semantic conflicts:

- rename file
- change function
- change configuration
- change dependency
- change CI

Tips to avoid semantic conflicts

- use tests
- make small changes
- merge frequently

Solutions

- communication between developers
- rebase before push
- merge lock
- name a release engineer that handles merge
  - may not be the most suitable person; usually the PR author is best
    
Merge queues

- Uber paper: *Keeping master green at scale*
- test different combinations of merge requests at a time
- popularsed the concept of a merge queue
- Airbnb *Evergreen Architecture*
- *shopify-merge*
- Mergify
- GitLab *merge train*
- GitHub released merge queue feature in July 2023

Who is it for?

- dev team working on same codebase
- high number of changes
- robust CI in place (not flaky, possibly long-running)

*Continuous merge*

- integration test, merge, deployment

Bots that create PRs

- Dependabot + Renovate
- These PRs might not need approval

Speculative checks

- check different combinations of PRs at the same time
- efficiently identify problematic PRs are remove them from the queue

Priority managements

- e.g. hotfix, normal, bot PRs

Monorepos

- identify distinct subsets of repo
- multiple merge queues for different segments of the codebase
