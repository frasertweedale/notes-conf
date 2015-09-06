Microservices - is HTTP the only way?
=====================================

Ben Shaw

What is a microservice?

- app that reads some data, processes it, and outputs the result
- should operate in only one domain
- app's behaviour should be describable in one sentence

What is a microservice not?

- uses only one db table
- runs on a separate server
- is written in a different language
- communicates over http

Demo app - features

- users can publically post messages
- authn system required (signup/login)
- users can "follow"
- 141chr msgs
- Uwitter (Twitter + 1)

The monolith:

- Django + db
- tables: uweet, user_profile (incl. follow info), user (incl login
  details)
- urls:
  - uweets
    - /
    - /post
    - /u/<username>
  - follow
    - /(un)follow/<username>
  - /accounts/register
  - /accounts/log{in,out}

Uweeting microservice

- store and retrieve (all or by user) uweets
- "message" database: id, user_id, message, date_posted
- DB API:
  - post_message
  - search_messages
  - list_messages
  - ...
- HTTP API
  - obvious

Advantages of HTTP

- well supported
- de facto solution
- security and authentication built in
- no networking issues
  - firewall is probably open
  - proxying and load balancing is a "solved problem"
- synchronous

Disadvantages of HTTP

- some overhead with headers
- load balancing is not magic
- synchronous

Notification microservice:

- when user posts a uweet, notify all followers
- using a message queue
  - put stuff on MQ
  - workers will read messages from the queue
  - in this case, queue is message sink
  - used Kombu (AMQ lib) as transport
    - in theory this should allow cross-platform

Advantages of Message Sink

- message set and forget
- scaling up is magic
- asynchronous

Disadvantages of Message Sink

- need message broker that you can communicate with
  - firewalls etc
- having intermediary service (broker) adds level of complexity for
  debugging
- can give a false sense of security
  - app will receive confirmation that message entered queue, but no
    guarantee that there are consumers

Authentication microservice

- store new registrations and authenticate them with username and
  password

Two-way communication with MQ

- use separate queues for requests and responses
- need way to identify which message corresponding to which web
  request
  - "message_id" (UUID)

Advantages of MQ

- magical scaling

Disadvantages of MQ

- handle own security
- need terrible code that only works on single python process

Advantages of TCP

- low overhead
- load-balance with HAProxy
- synchronous

Disadvantages of TCP

- firewall
- DIY
  - ``socketserver`` module
  - seucrity
  - threading
- synchronous

UDP

- some kinds of notifications don't need a response
- somes messages don't matter if we lose them

Advantages of UDP

- less overhead than TCP
- easy to code
- faster than TCP
- asynchronous

Disadvantages of UDP

- unreliable
- DIY
  - ``socketserver`` module
  - security (maybe signing is OK)
  - threading

Other thoughts

- naturally tend toward façade pattern

Code sharing / duplication

- no cross-project imports
- lots of copy-pasta
- in real life you would notice this happening and extract lib

Is HTTP the only way?

- No
- But it's definitely the easiest
