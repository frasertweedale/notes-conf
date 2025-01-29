# Evolving your APIs 

- Nicolas Frankel `@nicolas_frankel`
  - developer, developer advocate


- Your first API
  - Probably focused on RESTful semantics
    - But nobody really knows what that is
  - Did you forget about versioning?
  - Reverse proxy (industry standard practice)

- Instead of reverse proxy, **API gateway**
  - You shouldn't put your business logic in a reverse proxy
  - API gateway is more appropriate place
  - API gateway should support dynamic reconfiguration

- API gateways
  - Apache APISIX
  - Kong Gateway
  - Tyk
  - Gloo
  - Ambassador
  - Gravitee

- Apache APISIX demo

- Versioning the API
  - Path-based, e.g. `example.org/hello/v1/*`
  - Query-based (possible, but never seen in the wild)
  - Custom or content negotiation headers
  - Migrating from unversioned to versioned: use HTTP 
    Moved Permanently status (no breaking changes for
    client changes)
    - Note: you just double your number of requests, and
      probably your cloud bill as well!
  - Users: monitor your consumption!

- Make users register?
  - Then you can contact them.
  - Developers don't like to register.
  - "Incentive" (big stick): Status 429 Too Many Requests

- Canary releases
  - Weighted routes

- Deprecate your end points
  - IETF `Deprecation` header draft
    - https://datatracker.ietf.org/doc/html/draft-ietf-httpapi-deprecation-header
    - `Sunset` header: timestamp or Bool
    - `Link` header points to new resource

## References:

- https://bit.ly/evolve-apis
