# Zero Network: Privacy Preserving Transactions Blockchain based on Substrate

## Abstract

We describe the blockchain which supports privacy preserving transactions for both transfers and smart constract executions depending only on cryptgraphic hardness assumption. In this document, we describe how we realize privacy with only cryptgraphy instead `TEE`, `L2 solutions` and `trusted parties`, `optimistic assumptions`.

As a part of our protocol, we combine some cryptgraphic toools. We would like to intruduce these tools and compare these with other alternative choices, and we finally describe how we implement the privacy preserving transactions with them.

## Contents

Firstly we describe the problems we face to when we realize the privacy preserving blockchain, the difinition of privacy and how we address the problems in [Overview](1_0_overview.md). Finally, we describe the concrete constraints that the proof of transactions should satisfy in [Transaction Constraints](2_0_transaction_constraints.md). Addtionally, we add related research in [Related Tools](5_0_related_tools.md).

- [Zero Network](0_0_zero_network.md)
- [Overview](1_0_overview.md)
    - [What is Privacy](1_1_what_is_privacy.md)
    - [Hide Transfer Amount](1_2_hide_transfer_amount.md)
    - [Gas Limit](1_3_gas_limit.md)
    - [Zero Knowledge Scheme](1_4_zero_knowledge_scheme.md)
    - [Transaction Constraints](1_5_transaction_constraints.md)
- [Transaction Constraints](2_0_transaction_constraints.md)
- [Primitive](3_0_primitive.md)
    - [Crypto](3_1_crypto.md)
    - [Jubjub](3_2_jubjub.md)
    - [Bls12 381](3_3_bls12_381.md)
    - [ElGamal](3_4_elgamal.md)
    - [Pairing](3_5_pairing.md)
- [Pallet](4_0_pallet.md)
    - [Plonk](4_1_plonk.md)
    - [Encrypted Balance](4_2_encrypted_balance.md)
    - [Confidential Transfer](4_3_confidential_transfer.md)
- [Related Tools](5_0_related_tools.md)
    - [Stealth Address](5_1_stealth_address.md)
    - [Pedersen Commitment](5_2_pedersen_commitment.md)
    - [Non Interactive Zero Knowledge Proof](5_3_non_interactive_zero_knowlege_proof.md)
        - [QAP](5_3_1_qap.md)
        - [Polynomial Commitment](5_3_2_polynomial_commitment.md)
        - [Homomorphic Encryption](5_3_3_homomorphic_encryption.md)
- [Tutorial](6_0_tutorial.md)
- [Frequent Errors](7_0_frequent_errors.md)

## Reference
[`Crypto Note v 2.0`](https://github.com/monero-project/research-lab/blob/master/whitepaper/whitepaper.pdf)  
[`Additive homomorphic encryption which supports one-time multiplication`](https://github.com/herumi/mcl/blob/master/misc/she/she.pdf)  
[`Zether: Towards Privacy in a Smart Contract World`](https://crypto.stanford.edu/~buenz/papers/zether.pdf)  
[`Zerochain Book`](https://layerxcom.github.io/zerochain-book/)  
[`A specification for a ZK-EVM`](https://ethresear.ch/t/a-zk-evm-specification/11549)  
[`ZKPs for privacy-Preserving Smart Contracts and Transactions`](https://github.com/nucypher/Sunscreen_public/blob/master/zk%20thoughts.pdf)  
[`plonkup: A simplified polynomial protocol for lookup tables`](https://github.com/AztecProtocol/plonk-with-lookups/blob/master/PLONK-with-lookups.pdf)  
[`Pinocchio: Nearly Practical Verifiable Computation`](https://eprint.iacr.org/2013/279.pdf)  
