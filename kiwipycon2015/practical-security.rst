Practical Security
==================

- Jeremy Stott
- @jsstott

Why is security so hard?

- Are there _really_ people out there that do that?
- But we are releasing it next week! (no time for security)
- Noone will attack _us_!
- We don't store anything important.
- We can't afford it yet.
- Framework X handles all the security things.

- As a developer, security is my responsibility.
  - or a tester, or anyone really

- Use HTTPS for your whole website
- If you don't know what you're doing, don't host it yourself
- Keep your private keys safe!

Cookies:

- Don't put secrets in cookies
- never store anything personal, or identifiable
- flags: httponly, secure (sent only in https req)

Authentication and authorisation

- not just "auth"; these are two different things
- authentication = who are you
- authorisation = are you allowed to do X?

User input

- do not trust your users
- SQL injection
- script injection, XSS
- escape user import
- always escape stored user-supplied content when writing out
- parameterised queries
- whitelist, don't blacklist
- make sure webserver runs with low privileges
- don't write your own sanitisation functions
