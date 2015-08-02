BitID

- open proto allowing simple and secure authn using PK crypto


Simple sign-up

- scan a QR code
- your ID is your public addr
- unique addr per site
- no passwords

How it works

- server provides cryptographic challenge url
  - diplayed as QR code
- users bitcoin client
  - generates unique addr based on the domain
  - post signed response to callback url

Live demo

Benefits

- out of band authn
- no 3rd party
- no storage of user-sensitive data on server
- resistant to brute-force or dict attacks

Warning

- you've got to protect your keys
- and make backups
- MitM
- no authn recovation in protocol

Potential uses

- web authn
- decentralised 2FA
- door access control (lockers, hotel, etc)

See:

- gihub.com/bitid/bitid
- demo at bitid.bitcoin.blue
- PyBitId

See also:

- BitAuth
- SQRL
