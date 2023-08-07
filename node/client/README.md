# Kogarashi Cli

## Commands

|Command|Description|Sample Argument|
|---|---|---|
| list |Display default account list. This doesn't take an argument.|-|
| init |Generate user RedDSA wallet. It fails if the user already generated a wallet. This doesn't take an argument.|-|
| balance |Get account balance. It fails if the user hasn't generated a wallet and has no argument. This takes an optional argument and gets user balance if there is no argument.|Optinally, 'Alice' 'Bob' |
| fund |Fund authority account to user wallet. It fails if the user hasn't generated a wallet. This doesn't take an argument.|-|
| transfer |Transfer asset to another account. It fails if the user hasn't generated a wallet and has no argument. The first argument is the wallet owner's name which can be displayed with the `list` command and the second argument is the transfer amount. | 1: 'Alice', 2: 500  |
