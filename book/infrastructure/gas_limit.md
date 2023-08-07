# Gas Limit

In this section, we describe the potential problem that privacy preserving transactions project have.

## Transaction Scheme

When we realize the privacy preserving transactions with homomorphic encryption, the transaction sender transaction scheme will first calculate the encrypted value and second generate the proof which proves the validity of these relationship. First schmeme needs to perform homomorphic arithmetic, and second one needs elliptic curve arithmetic and polynomial evaluations. And the verifier need to verify the proof by performing the pairing and homomorphic arithmetic. Both side needs to perform the heavy workload computation. The more computation we perform, the more gas cost we need to pay. If the verify function exceeds the gas limit, we would be unable to realize the protocol. To make it practical, we optimize the workload.

## Account Base vs UTXO

When we generate the zero knowledge proof, the more complex data structure we need to prove the condition, the more computation we need. There are mainly two types of data blockchain structure. The `account base` is just key-value mapping data structure. It's easy to prove the condition. The `UTXO` uses the input and output transactions when it transfers the asset. It's complicated comparing with `account base`. However, the `UTXO` can prevent the double spending with data structure and it's hard to track the transaction history.

## EVM vs Wasm

When we verify the zero knowledge proof, the verify costs depend on efficiency of VM environment. If we perform the verify calculation efficiency, we would save the gas cost. When the Ethereum was launched, it's not designed for perform the zero knowledge functions so we have limitation of optimization once the blockchain is deployed. The Wasm is more efficient and we can customize and create new bytecodes. We have a lot of ways to optimize.

## Zero Knowledge Friendly

The data structure is the trade off between security and workload. We use the account base data structure because the gas limit is the main bottleneck. We are going to support Wasm because it's high performance and we can optimize the workload on blockchain. We don't have a plan to support EVM now so our blockchain doesn't have compatible with Ethereum contracts.
