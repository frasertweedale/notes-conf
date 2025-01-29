# Webauthn, Passkeys and You - the Future of Authentication

- William Brown, SUSE

- Passkeys
  - apple announced last year
  - media reportsk

- What are passkeys?  Noone really known.
  - There is no actual definition.  They're just trying to create a
    more approachable term for "all possible Webauthn
    authenticators"

- Demo.
  - Registering with yubikey.
  - Browser native UI.  
  - Login using phone.

- Parties:
  - RP (service/site)
  - Client (UA)
  - Authenticator (yubikey, touchid, ...)

- Device interaction (PIN, presence, fingerprint, etc)

- How did the phone authenticate me?
  - "Multi-device credential"
  - key is stored in cloud and sync'd to devices
  - phone connected to computer over bluetooth, to act as
    Authenticator

- Can they be phished?
  - No.  Data signed by authenticator includes origin.

- How many sites can a key work with?
  - Infinite*
  - Resident vs non-resident keys

- "Discoverable keys"
  - What if RP sends empty credential ID list?
  - Authenticator can search for a resident key matching the RP.
  - Some authenticators can store 0 resident keys; up to 32 is
    common.
  - CTAP 2.0 can **never delete a key**
  - You **MUST** assume finite and indestructible keys.

- Conflicting definitions of "passkeys"
  - all possible webauthn authenticators
  - synchronised credentials
  - resident keys (problematic!)

- How can I use them?
  - Spec is very prescriptive, but use case focused HOWTOs are few
    and far between.
  - Some libs have problematic doc examples 
  - Set uv=required usually
  - Set rk=required never
  - Libraries still evolving; review the options

- If you have strict security requirements, you'll need attestation
  (a topic for another talk)

- Conclusion: passkeys are an easy and secure way to authenticate
  - go forth and implement

## Questions

- Losing tokens - how can it be handled?
  - RPs should handle this by encouraging/forcing users to enrol
    multiple authenticators

- PII or user enumeration risk?
  - Maybe.  In most cases the risk to users from forcing Resident
    Keys is worse.  For niche or sensitive RPs it could be a risk
    for users.
  - RP countermeasure: return junk Credential IDs for any login
    request with unknown user.

- What *is* the use case for Resident Keys?
  - RKs are part of Apple's account sync model
  - autocomplete of user names?  "Conditional UI" (an opportunistic
    property).  Probes key in background and prompts autofill.
    - Useful for physical access

- Can/Will Passkeys save us from the proliferation of
  single-service, almost-always proprietary authenticator apps or
  tokens (e.g. for banks, MyGovID, etc)
  - Because of the resident key situation, apps will exist to absorb
    the overflow
  - 3rd party auth can be a problem/threat for highly
    regulated/sensitive services.  3rd party IdPs may not be
    acceptable.

- Future spec - what is DPK?
