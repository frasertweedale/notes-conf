Get on the Bus!
===============

Lennart Poettering


- Don't roll your own IPC
- D-Bus is better
  - performance-wise
  - functionality-wise
  - portability-wise
- Don't reinvent this wheel

Why D-Bus?
----------

- Discoverabliity
  - Find services

- Introspection
  - What do the services offer / require

- Access control
  - Only privileged entities should have access to privileged
    operations
  - Per-service, per-method, even PolicyKit

- Universal programming langugae support
  - Although not all the binding are equally good

- Powerful debugging toolbox

- Monitoring / eavesdropping


Kdbus
^^^^^

- Kernel D-Bus

- New iteration of local IPC

- Better performance
  - Can exchange massive amounts of data efficiently

- Portable (mostly)

- Type safety
  - Rich type vocabulary
  - inclusing structs, variants, dicts

- O(1) access

- Natural alignment in memory

- FD passing

- Network support

- Container support

- Bus activation
  - Services don't have to run all the time
  - Activate on demand, when needed
  - Exit on idle

- Sandboxing (kdbus)

- Reliable multicasting

- Generic event loop support

- Global ordering guarantees

- Timestamping (kdbus)

- Priority queuing

- It's kind of the standard
  - on generic Linux, there's no IPC more applications, init
    systems, system srevices speak.

- Choice
  - for C: libdbus, gdbus, sd-bus


Concepts
--------

- Bus
  - User bus (session bus); system stuff
  - System bus; system stuff

- Service
  - A program that offers something on the bus

- Object Path
  - A reference to one specific objects that exists

- Interface
  - A set of methods, signals and properties

- Member
  - Generalisation of method, signal, property

- Comparison to URL:
  - http://SERVICE/OBJECTPATH/INTERFACEMEMBER
  - on a BUS


Examples
--------

- ``busctl``
  - tab completion
  - paging
