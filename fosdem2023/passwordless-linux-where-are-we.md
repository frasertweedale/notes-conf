Presenter: Alexander

- remember the traditional office environment?
  - first of all, it was in an office
  - network connectivity; connecting directly to IT infra
    

- now: all the infra is in the cloud, or not directly accessible.
  - users use applications; applications use the infrastructure
  - applications consuming OAuth etc

- browser is new mainframe
  - 2016: captive portals.  network access needs captive portal
    handling before login to desktop/laptop

  - 2023: OAuth 2.0 idp before login
    - login with OAuth 2.0 implies user browser interaction
    - still no browser view access prior to GDM login
      - security issues with untrusted content

- Demo
  - ssh into some machine.
  - associate FIDO2 token with user in keycloak
    - then log out of IdP to purge session
  - ssh login prompts to authenticate at URL
  - go elsewhere and do the token auth at IdP
  - login succeeded; kerb TGT automatically acquired
  - no password.  never had one!

- (Showed protocol flow slide)

- probably want to use phone; show QR code and phone can
  authenticate to IdP.

- Webauthn / FIDO2
  - OAuth 2.0 IdP
    - may already support webauthn/fido2 tokens
    - may already allow login to itself with webauthn
  - FreeIPA in f37
    - login with external idp authn

- More demos
  - `vlock`: webauthn / external IdP based unlocking
  - hardware device based unlock with phyiscal presence check

- webauthn / fido2
  - combine local FIDO2 and kerb
  - similar to OAuth 2.0 IdP integration
  - WIP

- Desktop integration
  - GDM login issues: UX, multiple authn methods, passkeys and
    remove device guidance
  - Other graphical environments
  - authentication state preservation

- Distribution integration
  - upstream project coordination
  - parallel efforts
  - we collectively need to focus on the polishing, as a community

- Q&A
  - Question about security & stealing kerb keys/tickets.  Alexander
    discussed gssproxy as a countermeasure, and privilege separation
    more generally.  Also credential cache storage implementations
    (e.g. KCM)

- Shock and awe the quasi-heckler question-askers.
