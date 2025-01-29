- OpenShift CI: ephemeral clusters on many cloud providers

Optimise cloud spend:

- easy wins
  - don't run unnecessary worklodas
  - minimise worker nodes
  - machine type optimisations
    - AMD CPus, machinetypes, alt arches

- lower cost cluster profiles
  - single node runs
  - hypershift
  - hive

- spot instances for ephemeral cluster
  - up to 50-90% reduction

- data transfer optimisations


Background story

- in past, openshift ci policy was permissive
- financial burden associated with incoming users who were utilizing
  our cloud accounts was significant and growing

- solution: out team needs solutions for accurate cost attribution,
  and transition to a stricter policy

The solution:

- before: costs attributed to Openshift org
- now: costs attributed to entities that use the resources
- changes to existing infra:
  - use user-provided cloud accounts
- user migration campaign
- impact on users and their reactions


Leverage existing concepts: cluster profiles

- existing concept
- use to configure jobs
- mix of content created by us and the user
  - secrets, env vars, leases, **self-service credentials**

Migration

- users outside of Openshift org were using our cluster profiles
- migration steps
  - prep a cloud account
  - provide credentials using Vault
  - test new cluster profile (rehearsal tests)
  - migrate al lremaining tests
  - optionally make cluster profile private

Reality

- not every happy
  - i don't have time
  - how much will it cost me?
  - nope this is not official
  - my product is crucial
  - ignoring as long as possible
  - some people happy / no problem
  - ask to extend the timeframe

Results

- nearly everyone migrated in the required timeframe
- due to adoption of clutser profiles, fewer clients are competing
  for resources on main profiles

What we did correctly

- long transition period (60 days)
- tailored communication
- possibility to apply for timeboxed eception
- target cases from most expensive to least
- provided help on forum channel

What could we improve?

- more clear communication
- initial director message should be broad
- better tracking of exceptions


Cluster pools

- collection of pre-created clusters on a given cloud account that
  can be claimed and used by CI jobs
- we ues Hive
- Same problem as before: financial burden -> transition to stricter
  policy.  Another migration campaign

Main takeaways

- migration was far less controversial
- communication was better, but still not ideal
- received a high number of exceptions
- we improved tracking of exceptions
  - by trying to set the same end date

Migration results

- Does not necessarily change the company bill!  But our org is happier
- Cost attribution eliminated additional costs on the org, 10-15%
- perf gains due to less competition
  - shorter waits for cloud resources
  - slightly reduced test flakiness

Can we do more?

- with help of analytics, we could have cost attribution at team,
  manager or even developer level.

- Pros:
  - dev can received tailored info about their test run costs
  - users can be prompted if test is perma-failing
  - for other orgs: better budget planning per team, release or both
  - easier elimination of abandoned tests

- Cons:
  - dev hesitant to test expensive workflows when made aware of costs
  - might be a race to artificially elminiate cost by
    cutting expensive runs

- It is still being discussed
