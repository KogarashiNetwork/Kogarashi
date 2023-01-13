# Transaction Constraints

In this section, we describe how to generate the proof for each transactions.

## Transfer

When we transfer the assets, as we described at [Hide Transfer Amount](1_2_hide_transfer_amount.md) section, at first we encrypt the value and the second we generate the proof which proves the validity of value. In terms of the transfer transactions, the constraints the transfer transactions need to satisfy are always same. We can modularize these constraints and generating the proof. The more details for the constraints, you can see it on [Transaction Constraints](2_0_transaction_constraints.md) section.

## Smart Contract Execution

When we execute the smart contract, the constraints for each transaction is not the same so we can't modularize the same way with transfer. To define the constraints, we customize the smart contract compiler. The smart contract compiler often input the smart contract language and, output the compiled codes and metadata. We can define the constraints when the developer compile the contract. We costomize this compiler and output the constraints information as metadata. The developers need to show the constraints to user the same way as to show contract ABI to user. The user inputs the transaction constraints and their arguments to generate the proof. We also describe the details constraints in [Transaction Constraints](2_0_transaction_constraints.md) section.

## Summerize

In terms of transfer, the contraints are always same so we can modularize these constraints. In terms of smart contract execution, the constraints are different for each smart contract so we customize the compiler and output the constraints when it compiles the smart contracts. The developer provides the constraints for users and they can know the constraints when they generate the proof.
