# Zero Knowledge Scheme

In this section, we describe the zero knowledge scheme features.

## `SNARKs` vs `STARKs` vs `Bulletproofs`

We compare the three types of zero knowledge scheme `zk-SNARKs`, `zk-STARKs` and `Bulletproofs`. The `zk-SNARKs` is the most efficient. We can verify the proof with const or almost const time and generate proof process is also efficient. The proof size is also small. However, it's necessary to setup the parameters. We can save a lot of workload because of this but it would be the critical security issue. The `zk-STARKs` doesn't need to setup parameters and it has quantum tolerance. However, its proof size is far bigger than `zk-SNARKs` and the workload of verification process also far bigger than `zk-SNARKs`. The `Bulletproofs` doesn't need to setup parameters and its feature is in the middle between `zk-SNARKs` and `zk-STARKs` but it doesn't have quantum tolerance.

## Summarize

To summerize the above comparison, it would be as following table.

| Scheme | Trusted Setup | Prover Cost | Verifier Cost | Proof Size | Quantum Tolerance |
| ---- | ---- | ---- | ---- | ---- | ---- |
| zk-SNARKs | Necessary |Low|Low|Small| No |
| zk-STARKs | Unnecessary |Moderate|High|Large| Yes |
| Bulletproofs | Unnecessary |Low|Moderate|Moderate| No |

## Privacy Preserving Transactions Friendly

The `Bulletproofs` is mainly used for other privacy preserving transactions project for example [`Aztec`](https://aztec.network/), [`Zether`](https://crypto.stanford.edu/~buenz/papers/zether.pdf) and so on. That's because these projects are Ethereum smart contract base projects so if they use the `zk-SNARKs`, it's necessary to setup the parameters for each deploy smart contracts. It's really hard to collect enough parties to setup the parameters for each deploy. We use the `zk-SNARKs` because of its efficiency and the `plonk` allows us to setup parameters without depending on circuit. In other words, once we setup the parameters, we can reuse them when we prove the transactions.　
