# Hide Transfer Amount

In this section, we describe how we hide the transfer amount.

## Homomorphic Encryption vs Commitment

We have two options to hide the transfer amount homomorphic encryption and commitment. The homomorphic encryption allows us to calculate the encrypted number. To combine the homomorphic encryption and zero knowledge proof, we can calculate the encrypted number without decrypting and check whether the calculation was done correctly by zero knowledge proof. And the other option is commitment. The commitment uses the one-way function for example hash function. It hides the original value. The user provides the part of original value and check whether original value satisfies the condition. We describe the famous commitment in [Pedersen Commitment](../technical/pedersen_commitment.md) section.

If we use the commitment scheme, we need to generate the randomness for each transaction and prove the validity with it. It's hard for users to deal every randomness and also we support the contract execution so it's not practical to generate randomness for each value. If we use the homomorphic encryption, we can realize it simpler way. We are going to describe how we hide the transfer amount and prove whether that value satisfies the condition in [Transaction Constraints](../constraints/confidential_transfer_constraints.md) section.

## Scheme

To summarize the two scheme difference, it would be following.

### Homomorphic Encryption

The homomorphic encryption can calculate the encrypted number. Let `Enc()` encrypt function and we can't know the input from output. We can get calculation result without revealing actual value.

$$ Enc(10) + Enc(5) = Enc(15) $$

The encrypted value doesn't expose any information so we need to attach the proof which proves the value satisfies the condition. If users try to transfer the asset, user need to prove that the user balance is more than transfer amount, transfer amout is not negative and so on.

### Commitment

The commitment can hide the number and prove that the value satisfies the condition. Let `Hash()` hash function. To make it hard to know the input from output, we generate the randomness. The hash function takes `amount` and `randomness` as argument. If user wants to send `3` asset and it's balance is `10`, user would generate the randomness and prove that following equation holds.

$$ Hash(3, 100) + Hash(7, 200) - Hash(10, 300) = 0 $$

The transfer amount user wants send and the amount after transfer is equal to current balance when it's added. The hashed value is called commitment. We describe this more detail with example in [Pedersen Commitment](../technical/pedersen_commitment.md) section.
