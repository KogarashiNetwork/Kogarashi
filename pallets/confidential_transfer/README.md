# Confidential Transfer Pallet
Account-Based transfer with hiding transfer amount.

## Statement

Zero Knowledge Proof proves following statement.

$$
enc_{left} = g^tpk^r\ \land enc_{right} = g^r\ \land enc_t' = (g^{b'}pk'^r)\ \land \\ enc_{left}/ enc_{balleft} = g^{b'} (enc_{right}/enc_{balright})^{sk}\ \land t \in [0,Max]\ \land b \in [0,Max]
$$

## Test

```shell
$ cargo test
```

## Documentation

- [Tutorial](https://kogarashinetwork.github.io/Kogarashi/tutorial/confidential_transfer_pallet/)
- [Technical](https://kogarashinetwork.github.io/Kogarashi/constraints/confidential_transfer_constraints/)
