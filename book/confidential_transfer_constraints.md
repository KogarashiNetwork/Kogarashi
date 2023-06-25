# Confidential Transfer Constraints

This `Confidential Transfer Constraints` checks whether users transfer transaction was done correctly.

## Transaction Requirement

Specifically, constraints check following conditions.

1. The transfer amount is encrypted by exact sender and recipient public key
2. The transfer amount and sender remaining balance are in valid range (not negative)
3. The transfer amount and sender remaining balance are calculated correctly

On the premise that every balances are encrypted by homomorphic encryption with different key and we need to perform addition and subtraction without revealing actual value. If Alice transfer Bob crypto currency, sender and recipient need to encrypt same transfer amount with their key in order to add and subtract their account without decrypt. First constraints are that they encrypt same transfer amount with their keys. And the next, we need to clarify that Alice has enough balance and her transfer amount is valid. Second constraints are that Alice balance is more than transfer amount and the transfer amount is not negative. The user needs to generate the proof which proves that the transaction satisfies above condition.

## Transfer Scheme

We describe the confidential transfer scheme here. We assume the case that Alice wants to transfer `t` amount crypto currency to Bob.

We define the symbol for each parameters as following.

|Symbol|Description|
|---|---|
|sk|Alice private key|
|pk|Alice public key|
|pk'|Bob public key|
|t|Transfer amount|
|b|Alice remaining balance|
|enc_bal_left|Balance encrypted by Alice|
|enc_bal_right|Balance encrypted by Alice|
|enc_left|Transfer amount encrypted by Alice|
|enc_right|Transfer amount encrypted by Alice|
|enc_t'|Transfer amount encrypted by Bob|
|r|Randomness|
|g|Generator of elliptic curve point|

We perform this transfer scheme on `jubjub` curve and the constraints are following.

$$
enc_{left} = g^tpk^r\ \land enc_{right} = g^r\ \land enc_t' = (g^{b'}pk'^r)\ \land \\ enc_{left}/ enc_{balleft} = g^{b'} (enc_{right}/enc_{balright})^{sk}\ \land t \in [0,Max]\ \land b \in [0,Max]
$$

Users generate proof and attach it with transaction.

## Transaction Speed

In my local PC, it takes about 5 seconds to generate proof and 78 milli-seconds to be verified.
