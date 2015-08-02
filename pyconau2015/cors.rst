- krotscheck.github.com


Outline:

- history of same origin policy and XMLHttpRequest
- breaking browser sandbox
- CORS spec
- Example impl
- Deom

Same origin policy:

- netscape 2 (1995)
- script on one page allowed to access content on other pages iff
  "origin" is same
- origin = (scheme, domain, port)

Important exceptions:

- include scripts from other domains directly

XMLHttpRequest:

- nothing to do with xml
- IE5 (1999)
- lets you issue HTTP reqs directly from JS engine
- SOP was extended to include XMLHttpRequest

Hacks and workarounds

- HTTP proxies
  - server proxies request on behalf of client
  - processing overhead
  - what if 3rd party is restful and returns fully-qualified URIs
    - proxy has to rewrite all of the things

- document.domain
  - HTML DOM 2.0, Jan 2013
  - supposed to be read-only ; was never implemented read-only
  - trick scripts into thinking they're on a different origin

- JSONP
  - "JSON with Padding" (2006)
  - Manipulate DOM to issue GET request with callback
  - result returned as javascript to be direclty included in page
    and call callback

- Cross-document messaging
  - WHATWG 2008
  - Messaging between windows / iframes
  - ``.addEventListener`` ; ``.postMessage``

- WebSocket
  - RFC 6455; 2011

- CORS
  - Jan 2014
  - https://www.w3.org/TR/cors/
  - works in HTTP headers
  - preflight check
    - "mother, can I do this?"
  - request

Preflight request::

  Origin: https://myapp.example.com
  Access-Control-Request-Method: GET
  Access-Control-Request-Headers: accept, x-client

Preflight response::

  Vary: Origin
  Access-Control-Allow-Origin: https://myapp.example.com
  Access-Control-Allow-Methods: GET
  Access-Control-Allow-Headers: accept, x-client
  Access-Control-Allow-Credentials: true

Wildcard::

  Access-Control-Allow-Origin: *

CORS: server

- Apache 2: mod_headers
- Nginx: add_headers
- Python packages
  - django-cors-headers
  - wsgicors
  - Flask-Cors
  - tornado-cors
  - oslo_middleware (OpenStack)
