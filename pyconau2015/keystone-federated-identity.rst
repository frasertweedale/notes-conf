Practical Federated Identity - Jamie Lennox
===========================================

- Federated identity over last 18mths has been one of big
  advancements in Keystone and OpenStack in general

- A live walkthrough setting up horizon and keystone using websso.

- In keystone, SAML is the most mature option
  - Implemented first
  - Used in lots of enterprises

- In openstack, Keystone is Service Provider, but it doesn't have
  web frontend

- At high level
  - Browsers go to horizon
  - Get directed to keystone
  - Keystone punts to IdP; user logs into IdP
  - Keystone receives token from IdP and passes to Horizon

- Ipsilon for IdP
  - Universal adapter of Identity Providers
  - You don't have to update your crusty old IdP to provide SSO

- Demo
  - V3 auth required for federation
  - github.com/jamielennox/pyconau2015-ansible
    - does rely on some non-public nightly repos
      - for latest version of ipsilon
  - FreeIPA and Ipsilon!
  - Normal horizon login

Steps

1. modify horizion config to enable SAML SSO

2. ``ipsilon-client-install``

3. set up apache to forward you to IdP (Ipsilon provides a default
   config file) (uses ``mod_auth_mellon``).

4. Tell keystone that redirect-url is a valid horizon host.
   ``trusted_dashboard = ...``

5. Configure keystone to know who the identity providers are.
   - ``openstack identity provider create --remote-id URL ipsilon``
   - ``openstack federation protocol create --identity-provider
       ipsilon --mapping ipsilon-mapping saml2``

6. Tell keystone how to map saml assertions into
   keystone tokens

  - map user into a group
  - assign roles to group
  - Create groups and roles
    - ``openstack group create ipsilon-engineering``
    - ``openstack role add --group ipsilon-engineering --projcet pycon member``
  - Mapping file is a json spec w/ local and remote sections
    - if conditions in remote part are met, then the local section
      will be applied
    - interpolation in local section
    - ``openstack mapping create --rules mapping.json ipsilon-mapping``

Takeaways:

- it's not trivial but it's not that hard
- once it's set up, the hardest thing you'll have to deal with is
  the mapping file
  - keep mappings simple and obvious and take care
- websso is configured differently from CLI federation
