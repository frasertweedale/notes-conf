Encryption in OpenStack Swift
=============================

John Dickinson, Project Technical Lead, SwiftStack; @notmyname

- the crypto is the easy bit (just use a lib)

- doing it inside scalable, eventually consistent object store is
  the hard part.

- like S3, but open source and you can run it inside your own DC

- used all over the world, for all kinds of data

- RESTful HTTP API

  - ``https://swift.example.com/<version>/AUTH_<acct>/<container>/<object>``

  - PUT to write / overwrite
  - GET to read, DELETE to write, POST to put new metadata
  - POST accepts entire object; no deltas

- User -> Proxy server -> Storage nodes
  - if you need more of something, you can add more
  - best practice: proxy servers on different physical servers than
    storage nodes
  - medium-large scale deployment: load balancers in front of suite
    of proxy servers

- client-side crypto vs server-side crypto
  - what's the threat model?
  - for this feature, threat model is "what happens when hard drives
    leave the cluster?"
  - quite a few use cases for protecting on-premise data

- what we are going to encrypt
  - object data
  - object metadata
  - digests of data (md5; used for ETag)

- what we are not encrypting
  - account, container and object name
  - content type
  - fragment numbers

Cryptography:

- derived key
- key wrapping
- key encryption key (KEK)
- data/content encryption key (CEK)

- data from client arrives as plaintext (ignoring TLS)

- need to load "master key" on proxy
- derived key: HMAC(master, object name a/c/o)
- CEK is random
- KEK wraps CEK
- storage node receives encrypted data and wrapped CEK
- ETag now refers to *encrypted* data so audit/integrity process is
  unchanged
- fragmenting / erasure coding:
  - each fragment encrypted separately (with same random key)

Where does master key come from?

- initially (currently), stored in config file
- working on HSM stuff, etc

Storage node on-disk format:

- files on file system (xfs)
- ``mount/objects/partition/hash_suffix/hsah/timestamp.data``
- the metadata is tricky
  - kinds: name, content type, size, etag, user-defined metadata,
    creation time, internal system metadata, etc.
  - normally stored in xattr
  - ``X-Object-Meta-<Key>: <value>``
  - ``X-Object-SysMeta-<Key>: <value>``
    - example: "fragment 3 of 12"
  - system metadata must persist across POST
  - now: ``X-Object-Transient-SysMeta-<Key>: <encrypted-value>``
  - on POST, we strip off existing metadata and re-encrypt it under
    new namespace.
    - allows upgrade of non-encrypted object to begin using
      encryption
    - new random key on POST
    - keys *not* encrypted
- unmodified user experience, but moar security!

Etag matching:

- stored etag is HMAC(derived key, digest)
- when client requests with ``If-[None-]Match: <etag>``, recompute
  the HMAC and pass on to storage node.

Performance:

- negligible difference due to hardware support (AESNI)
- you should turn it on


Future work
-----------

- Key management systems for storing master key
- Per-account/container keys
  - instantaneous invalidation of all account/container objects by
    deleting their key ("secure delete")
- Key rotation
- BYO key (S3 has this feature)


Questions
---------

- Why encrypt etag / digest?
  - *why* can't we expose etag?
  - more flexibility to change algo?
  - can associate "this object is the same as that object"
- Why not FDE the disk
