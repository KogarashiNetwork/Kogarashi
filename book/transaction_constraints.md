# Transaction Constraints

In this section, we describe how to generate the proof for each transactions.

## Transfer

When we transfer the assets, as we described at [Hide Transfer Amount](infrastructure/hide_transfer_amount.md) section, at first we encrypt the value and the second we generate the proof which proves the validity of value. In terms of the transfer transactions, the constraints the transfer transactions need to satisfy are always same. We can modularize these constraints and generating the proof. The more details for the constraints, you can see it on [Transaction Constraints](constraints/confidential_transfer_constraints.md) section.

## Smart Contract Execution

When we execute the smart contract, the constraints for each transaction is not the same so we can't use same with as in confidential transfer so we generate constraints for each opcode because the opcode operation is always same. We generate proof for each opcode which proves that the opcode was performed correctly and put these together to one proof. It's the same approach with [`zkevm`](https://github.com/privacy-scaling-explorations/zkevm-circuits). The `Substrate` works on `wasm` so we are going to implement `zkwasm`. We also describe the details constraints in [Transaction Constraints](constraints/confidential_transfer_constraints.md) section.

## Summarize

In terms of transfer, the contraints are always same so we can modularize these constraints. In terms of smart contract execution, the constraints are different for each smart contract so we customize the compiler and output the constraints when it compiles the smart contracts. The developer provides the constraints for users and they can know the constraints when they generate the proof.
