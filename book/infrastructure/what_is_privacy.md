# What is Privacy

Before we describe our protocol, we would like to define what the `privacy` exactly means.

## Confidential vs Anonymous

There are two types of privacy level `confidential` and `anonymous`. The difference between them is that how much information we hide. The `confidential` means it hides the input and output. The `anonymous` means it hide the users related the transaction.

## Transfer Example

If the protocol supports `confidential`, the users would be able to hide the input and output. When users send the transaction, the input and output are going to be `balance`, `transfer amount` and `after balance`. The function needs to know that the transfer satisfies the conditions for example, the amount is not negative, the balance is more than transfer amount and so on. The `confidential` transactions can verify these conditions without revealing input and ouput values. We use the homomorphic encryption to realize this feature. You can see it on [`Hide Transfer Amount`](hide_transfer_amount.md) section.

If the protocol supports `anonymous`, the users would be able to hide the users information related to transactions in addition to `confidential`. When users transfer the assets, the users information related to transactions are going to be `sender` and `recipient`. There are some ways to hide users information and we describe some of them in related tools. The typical tool to hide the `sender` is the [`Ring Signature`](https://en.wikipedia.org/wiki/Ring_signature#:~:text=In%20cryptography%2C%20a%20ring%20signature,a%20particular%20set%20of%20people.) and the `recipient` is the [Stealth Address](../technical/stealth_address.md).

## Summarize

To summarize the story, the `confidential` hides the transaction contents and the `anonymous` hides the transaction senders and recipients. We describe the contents and privacy level in table.

| Item | Confidential | Anonymous |
| ---- | ---- | ---- |
| Balance | ✅ | ✅ |
| Transfer Amount | ✅ | ✅ |
| Sender | - | ✅ |
| Recipient | - | ✅ |
