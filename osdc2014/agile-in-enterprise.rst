Agile in the Wonderland: Open Source in the Enterprise World
============================================================

Nick Moore

Standard Operating Environment:

- still often Windows XP, Word, Sharepoint etc
- BYO tools
  - Git remote on shared drive
  - Trello doesn't like like an issue tracker
  - portableapps.com
  - (Or shrink your windows partition, and then run it virtualised)

Internet:

- URL restriction
- limits on concurrent connections
- (probably unintentional) URL mangling
- You might want to buy a 4G modem

Hairball:

- "Two hairs unite.  Then they're joined by another.  And another.
  And another. Before long, where there was once nothing, this
  tangled, imprenetrable mass has begun to form"
  - Gordon MacKenzie, *Orbiting the Giant Hairball*
- The hairball is a fact you're stuck with.
- Don't try to ignore it or cut it in half - you can't do it.

The team:

- Manager, Proj Manager, BA, sysadmin, Sales/Marketing, Finance,
  Users
- Find your project a champion

Project management:

- Developers don't measure progress in the same way that managers
  do.
- Temper your progress reporting
- remember all these tasks: planning, reading, documenting, testing,
  selecting, prototyping
- run multiple phases in parallel

Strange requirements:

- sometimes there as a legacy requirement
- sometimes beacuse a competing product has it
- sometimes it's just been it Gartner / Infoworld / etc
- better to say Yes than No
- better to say No than not answer the question

Surviving success:

- Outgrowing requirements
- Ongoing maintenance
- Integration with existing systems
  - if you're lucky, someone will tell you before internal
    interfaces changes.
- Handover ...

The good news:

- large enterprise systems starting to open up
- ODBC -> SOAP -> oData
- SAP oData
- oData is RESTful, fairly horrible syntadx for querying
- can emit JSON

Open data:

- extract into local db
- extract into msg queue
- direct access from HTML5
- use SSIS or BizTalk
- Keep state in SQL server
- specific task-related apps
- enable more efficient workflows
- beware usability lock-in!
