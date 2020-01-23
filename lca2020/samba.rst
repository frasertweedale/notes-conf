Samba 2020: Why are we still in the 1980s for authentication?
=============================================================

Andrew Bartlett, Catalyst

Samba 4.11 released:

- Samba AD for 300k user scale
- GnuTLS for cryptography
- SMB1 disabled by default
- LanMan and plaintext authn deprecated
- CI tested on OpenSUSE, Fedora, RHEL, Debian, Ubuntu
- Python 3

1980s authn:

- LanMan now disabled by default
- NTLM
  - from 1990s
  - still used extensively in MS-CHAPv2 (for Wi-Fi authn)
- NTLMv2
  - HMAC-MD5
  - primary risk is offline brute force
  - still user/pass challenge/response
- Kerberos
  - first deployed in 1986 (v5 in 1993)
  - decouples login (get ticket) from submitting the ticket
  - Smart card support (PKINIT)

- KDC on every device?
  - Apple did it
  - fileserver is a KDC for itself only

- MS focus is on Windows Hello / Hello For Business
  - hello = log into computer directly to cloud
  - competing with Chromebooks and Apple cloud-based login
  - unlock device with per-device PIN
  - enrolment procedure into AD via ADFS
  - so... not an easy add-on for Samba

What can we do?

- Smart Cards could be easier
  - record each key on the user entry
  - drop the requirement for CA infra

- SSH wrapping?
  - could we somehow forward over ssh *and* inherit the authn?
  - or add SSHv2 as SPNEGO option?

- NT hash-free Samba AD?
  - quite a few bits of samba use the MD4 hash
    - e.g. password history

- Safer NTLM for Samba?
  - can't negotiate NTLM versions with clients easily
  - 2nd factor in NTLMv2 resp, encrypted with password?

- U2F?
  - the only phys 2FA system that is simple to set up
  - could we give Samba "API keys" as password?

- Are we headed ot pure-web world?
  - does any of this matter?

