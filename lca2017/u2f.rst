U 2 can U2F
===========

Rob N (Fastmail)

slides: robn.io/u2f-lca-2017

- FreeOTP namedrop/screenshot

- 2FA assumes you are the only one with access to your 2nd factor
  - SMS is not secure
  - TOTP is safer (but typically depends on security of phone)
  - all these methods have some kind of vulnerability
    - no authentication of where a TOTP code came from
    - token valid for 30s?  That's long enough for attacker to
      intercept and log in.

- U2F is newer 2FA protocol that solves some of these problems
  - open standard, defined by FIDO Alliance
  - multiple manufacturers
  - USB, NFC, bluetooth
  - multiple server and host impls
    - not hard to implement

- device actively involved in authentication process
  - browser talks to it over USB
  - PK crypto

- one device, multiple sites
  - no tracking between services

- cloning protection

- device properties (can use for policy decisions)

- devices
  - U2F zero
    - open hardware design by Conor Patrick
    - u2fzero.com
    - ~$9 from amazon

  - Tomu (tomu.im)
    - open hardware certified
    - made by Tim Ansell
    - still needs software to be written

- u2f registration
  - plug in device, press button
  - device send public key to server (via browser)

- login
  - enter username and password
  - press button; you're done
  - under the hood there's a lot of stuff going on!
  - device signs challenge from server

- phishing avoidance
  - browser gives origin to device, device signs origin
  - server validates sig, checks that origin is valid
    - origin not valid -> MITM

- application-specific keys
  - avoid tracking across services
  - server also sends add_id, handle
  - device looks up key by (app_id, handle)
  - sign (challenge, origin, app_id)
  - on web, app_id and origin are the same

- cloning protection
  - device sends (and signs over) a per-app counter
  - server tracks counter; freak out if it sees lower-than-current
    serial

- attestation
  - attestation cert describes type of device
  - same cert used on all of a single device (or prod. batch)
  - make policy decisions based on attestation cert
  - attestation cert is just an X.509 cert

- implementation details
  - serverland
    - libs exist for most popular languages
    - but more to do for new/less commonly used languages
    - framework integration, etc
  - browserland
    - javascript
    - browser support is currently OK
      - Chromium via u2f-api.js
      - FF via extension (native Real Soon Now)
      - Microsoft "looking at it"
      - Apple silent
        - Safari extension available (not very stable)
      - Mobile
        - android + chrome + gauth + nfc
        - the way forward is probably Bluetooth profile
  - other
    - libu2f-host
    - ^ used by pam-u2f

- Bonus prize round!

Questions:

- can I use it for SSH, etc (i.e. like smart card)
  - you need server support, and the ability to transport challenge
    and response
  - someone is working on SSH support
    - there is an I-D for a u2f method

- do you think it will be used more by individual services, or just
  at major authn providers e.g. openid-connect.

- anything inherent in u2f that necessitates js in browser?
  - you can do away with the libs, but you have to invoke the
    behaviour somehow

- myGov integration?
  - lulz
  - write to MP

- what algo(s) used?
  - ecdsa nistp256

- where does it get entropy from?
  - ??? magic internal RNG ???

- what do you do when you break or lose your key?
  - same problem as existing 2FA solutions?
  - fastmail recommendation: get two keys; register both devices;
    keep one on person; one in safe deposit box.

- anything fancy in the attestation cert?
