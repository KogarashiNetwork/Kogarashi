# Circuits Spec
This describes the circuit used in pallet.

## Confidential Transfer
This circuit is implemented according to [`Zether`](https://crypto.stanford.edu/~buenz/papers/zether.pdf) p14 **Transfer transaction**.

### Additive Homomorphic
Alice has balance $a$ and public key $b$.  
She generates the randomness $r$ and computes encrypted balance $(g^r, g^a * b^r)$.  
When Bob transfers $c$ to Alice, he generates the randomness $r'$ and computes encrypted transfer amount $(g^{r'}, g^c * b^{r'})$.  
The sum of encrypted balance and transfer amount is folloing.
$$
(g^{r + r'}, g^{a + c} * b^{r + r'})
$$

## Circom
The circuits are described by `circom` language.

- [Installation](https://docs.circom.io/getting-started/installation/)
- [Implementation](https://github.com/iden3/circom)
