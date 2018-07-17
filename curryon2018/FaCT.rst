Constant-time crypto programming with FaCT
==========================================

Sunjay Cauligi, UC San Diego

- example: padding length check
  - bad padding oracle
  - divulge length of padding

- it's dangerous to
  - return early
  - branch

- constant time code is hard
  - folding control flow into data
  - manually keep track of secret vs public

- this is what DSLs are for

- DSL for:
  - distinguish public vs private
  - prevent writing leaky code

FaCT

- ``.fact``
- spit out C header and obj file
- looks like C
  - conditionals, early return, etc.

- automatically transform code using "recipes"
  - remove branches
  - remove early return
  - pass current control flow / caller state as an extra parameter

- labels ensure no leakage
  - type system tracks control flow label
  - prevent secret expressions we can't transform
    - loop bounds
    - array indices
    - variable-time instructions
      - e.g. prevent direct division / modulus with secret data
  - recursive calls
    - need to determine that recursion bounds are in fact public
    - but not that much cryptographic code uses recursion
    - so FaCT does not have recursion

- Problem: secret ``if`` statements always perform both branches
  - this kind of program is disallowed

- FaCT is memory safe and has no undefined behaviour
  - every mem access generates a (Z3) constraint during type-checking
  - aware of secret-if semantics
  - ``ct-verif`` for verifying that FaCT produces constant-time code

How good is FaCT really?

- case studies: libsodium crypto_secretbox
  - FaCT beat the C reference impl
- OpenSSL AES-CBC_HMAC-SHA1
  - FaCT gets within 5-15% of OpenSSL C impl

- Usability results
  - user study as part of undergrad PL class
    - understanding what code is doing (C vs FaCT)
    - writing constant-time code (C + ct-verif vs. FaCT)
  - takeaway: yep!
  - the main hangup for students was just the syntax
    - once beyond that, FaCT was easier for the students

Future directions

- optimise transformations
- add other platforms (ARM, CT-WASM, ...)
  - each platform has some quirks, e.g. variable-time
    64-bit mul on ARM
- verify the FaCT compiler
  - so that we don't have to depend on external verification tool


Also

- bindings for Python, Haskell, etc


Questions:
---------

Isn't there a real danger that another optimiser in the pipeline
would undo the constant-time transformed code?

- Yes

- The reason you have to do bit-hacks etc is to trick the compiler
  into believing the code is alive.


Is not declaring public/secret correctly a risk?

- Yes.  You have to know what must be secret and label it properly.

