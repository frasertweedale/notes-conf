JSON Standards for the Open Web - Jamie Lennox
==============================================

- There are hundreds of standards and helper libs etc for web APIs
  and formats

  - I'm only going to talk about a handful

  - There are competitors for all the standards I'm going to talk
    about

- SOAP
  - enterprisey
  - spins in all of the tricky things you can do with XML
  - WSDL described the interface
    - what funcs are calls, params and their types, errors that
      could be thrown.
  - can autogen WSDL file from service implementation
  - can autogen client lib from WSDL file
  - if it works, why should we care what protocol looks like?
  - there are lots of good tools for working with XML that don't
    exist for JSON

- jsonrpc ~ xmlrpm
- jsonpath ~ xmlpath
- jsont ~ xslt
- JsonML = JSON serialisation of XML data

What we want

- statelessness
- discoverability

jsonhome:

- api description
- theoretically you could rearrange all the endpoint and a
  well-behaved client would keep working

uritemplate:

- pip install uritemplate

jsonschema:

- a way to define a contract
- a strict representation of what can be sent to an API and what
  could be received back
- REST purists will tell you the MIME types should tell you
  everything you need to know, but in practice it's not enough


