# Infrastructure

Basically, all public blockchain state is public for everyone and it can be looked by unknown someone without any permission. To keep the privacy, the projects for example [`Zcash`](https://z.cash/) , [`Monero`](https://www.getmonero.org/) and so on realized the privacy preserving transfer. Now people can transfer crypto currency with private. However, the real world applications require more complicated functionalities and the blockchain should support various of use case. It was hard to realize the general purpose privacy preserving transactions but recent scaling and privacy technologies evolution allows us to make it practical.

To achieve general purpose privacy preserving transactions, there are mainly five problems to be addressed. `Hide transfer amount`, `Gas limit`, `Zero knowledge scheme` and `Contract constraint`, `Attack protection`. Firstly, we would like to define `what is the privacy` and describe the solution. Finally, we would like to describe `the solution for the attack`.

## Contents

The introduction contents are following.

1. [**What is Privacy**](what_is_privacy.md)  
    We define what privacy is before we discuss the protocol.
2. [**Hide Transfer Amount**](hide_transfer_amount.md)  
    We describe how to hide the transaction values.
3. [**Gas Limit**](gas_limit.md)  
    We describe how to save the workload avoid to exceed the gas limit.
4. [**Zero Knowledge Scheme**](zero_knowledge_scheme.md)  
    We compare the zero knowledge scheme and describe which is suitable for privacy preserving transactions.
5. [**Transaction Constraints**](../constraints/confidential_transfer_constraints.md)  
    We describe how user generates the transaction proof.

These sections are work in progress and we are going to add the experiment result.
