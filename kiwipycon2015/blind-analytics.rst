Analysing data without seeing the data
======================================

Brian Thorne

- Data61 (NICTA / CSIRO merger)
- @thorneynz

- machine learning is getting smarter
- better data, more accurate data, more kinds of data
- a lot more data
- computation is getting cheaper

Here be dragons as well as unicorns

- insights might be wrong, or too revealing
- usually it's the raw data that gets leaked

Privacy

- as a user you care about privacy
- many organisations do too
- many organisations decide to share or sell information about
  customers
  - often use third parties
  - regulations are sometimes obeyed
- monetisation...

Yao's millionnaire's problem

- alice and bob are interested in knowing who is richer but don't
  want to reveal actual wealth
- can be extended to N participants

Insight from data without seeing the data

- analyses
  - K-means clustering
  - Principal component analysis (PCA)
  - linear regression
  - logistic regression
- two approaches
  - take algorithm to the data and share delta
  - compute on encrypted (not raw) data

Applications

- business to business
- medical records
  - medical research without revealing patient data

Secret sharing

- good for highly sensitive and important data

Secure Multiparty Communication

- been a research area for 30 years
- taking off in last few years

Differential privacy

- maximise accuracy of queries while minimising risk of information
  disclosure
- introduce noise
- gives you a knob between accuracy and privacy

Homomorphic encryption

Paillier Cryptosystem

- github.com/NICTA/python-paillier
- python-paillier.readthedocs.org
