Programming the Cloud with Functions
====================================

Nick Mitchell, IBM TJ Watson Research Centre

(Chess timer talk)

*Programming with (the help of) functions, with(out) servers,
with (many) services.*

- the burden of maintaining and operating a service dwarfs the costs
  of building it

- need isolation, scalable logging, etc.

Q: guaranteed delivery is not realistic.  What are the guarantees?
A: differs between platforms e.g. at least once delivery, exactly
   one delivery, etc.

Q: How widely used is serverless today?
A: AMZN says (1 yr ago) Lambda gets >1bn hits per day.  Still way
   less than EC2 but rapid growth.

Q: 50ms.  Isn't that slow?
A: Yeah, but better than tens of seconds or more


What about...

- Kube
- Microservices
- Akka, streams, reactive
- unikernels
- Spark, X10

Can't possibly-isms:

- be efficient
- a joy to program
- express anything complex

Q: how different from PaaS?
A: - serverless pushes the level of abstraction further up the ladder.
   - You're still paying for your app a 2am when noone is using it

Q: what do you do about state?  can't keep anything hot in mem?
A: one of the main tenets of serverless is enforced statelessness


tech fatigue: 10 new APIs and services coming out every 10mins

how to be sane

- rich set of managed subsystems to deal with the day-to-day boring
  stuff (like an OS has)
- pay for what I use (don't pay for idle time)

performance

- fusing function (need to be same language)

Q: how to debug?
A: locally

Q: WebAssembly?
A: ...

Cold start problem

- room for improvement but way better than Kube.

Composer/Conductor (IBM)
