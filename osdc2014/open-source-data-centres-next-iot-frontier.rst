Open Source and Data Centres: The Next Internet-of-Things Frontier
==================================================================

Keith Miller; Red Hat refugee.  NEXTDC data centres.

Data centres:

- Data centres are one of the last bastions of proprietary control
  software.
- SCADA and BMS (Building Management Systems) have not progressed in
  many years.
- Many based on obsolete OSen (Windows 98!  ME!  Some have just
  been "upgraded" to XP).
- Very old, slow-moving vendors.  Very expensive!
- Control and telemetry systems are usually very vendor specific
  and hard to customise.

Complex architectures:

- lots of old serial comms
- RS-232, RS-485, Modbus are very common
- Some proprietary protocols.  Token bus!
- Demanding requirements.

The solution:

- oBIX (Open Building Information Exchange)
- OASIS standard
- http://www.obix.org/
- really starting to get legs

NEXTDC oBIX involvement:

- Open source project derived from the C oBIX Tools (CoT) project
  (2009)
- have rewritten from scratch since mid 2013
- GPLv3+
- github:ONEDC/obix ???

oBIX standard:

- XML & web services-based mechanism for building control systems
- oBIX designed to instrument control system
- designed for responsive, real time access to embedded control
  systems
- vendor independent, rapidly growing interest
- tech deep dive: http://onedc.com/blog/

oBIX implementation:

- multilayer distributed (scalable) arch with tiny footprint
- delivering real time power monitoring and telemetry
- maintains privacy/security (customer exclusive services)
- enables remote access control (un/locking racks)
- receive real time data to enable real time decision making
- sync adapters filter and distribute the data in real time

Access control:

- remote locking with front and rear door powered by oBIX provides
  access, security and control
- virtual and physical access levels of users can be managed and
  controlled by administrators
- provide time-frame specific records of any action, virtual or
  physical, within the allocated data spaces
- watch processes provide inter-server messaging

Real-time monitoring and analytics:

- powered by oBIX real time and history facility
- interactive data vis for effective knowledge discovery
- configurable alerts and notifications

More info:

- oBIX committee: www.obix.org
- community: http://onedc.com/community/index.html
- Keith.Miller@NEXTDC.com


Follow-up
---------

Follow up sent to Keith Miller:

  I had a quick look at the OBIX v1.1 draft which states that
  authentication and encryption are left as a transport issue, and
  privileges and user management are left as a vendor implementation
  issue.

  Do you have any comments or can you point to any resources about
  existing or proposed ways to leverage PKI or integrate with
  identity management systems in oBIX or related systems.


Response indicates that they are settling on SAML authorisation for
most larger deployments, and 2FA.  No substantial use of X.509/PKI
at this time.
