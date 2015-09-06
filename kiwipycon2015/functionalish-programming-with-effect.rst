Functionalish programming with Effect
=====================================

..
  Lots of code exmaples in this talk;
  recommend viewing slides / video

- Robert Collins <rbtcollins@hp.com>
- @rbtcollins

Chris Armstrong @radix

- side effects make reasoning about (Python) programs at scale hard

How to test ``print``?

- monkeypatching
- subprocess
- IO redirection (move file descriptors around)

Haskell

- Monads

Free monads

- create a language to express actions
  - "actions" could be IO, state manip, name lookup

Interpreter:

.. code:: python

  @sync_performer
  def real_print(dispatcher, print_):
    print(print_.line)
    sys.stdout.flush()

  real_interpreter = ComposedDispatcher([
    TypeDispatcher({ Print: real_print, }),
    base_dispatcher
  ])

- use separate interpreters for test and prod
  - avoid IO, disk ops, etc. during test

.. code:: python

  def test_print(self):
    outputs = []

    @sync_performer
    def perform_test(dispatcher, print_):
      outputs.append(print_.line)

- no more monkeypatching
- no more subprocess
- no more IO redirection
- need to test prod interpreter once and only once
- we used the production API!

Testing API:

- ``effect.testing.SequenceDispatcher``

Tips:

- use ``namedtuple`` for intents (get ``__eq__`` for free)
- hyptothesis for randomised property-based testing

``effect.do`` for a more pythonic API

- generator-based coroutines
- makes loops easier


Questions
---------

Where is it used?

- Otther (a rackspace OpenStack / Heat thing)
- Could solve some problems in Pip
