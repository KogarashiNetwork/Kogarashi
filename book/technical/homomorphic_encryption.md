# Homomorphic Encryption

In previous two sections, we enable to check whether `computation` was done correctly through polynomials equation and these polynomials are generated with valid process with following equations.

- [QAP](qap.md)
$$ L(x) * R(x) - O(x) = Z(x) * T(x) $$

- [Polynomial Commitment](polynomial_commitment.md)
$$ L(x) + R(x) * X^{d+1} + O(x) * X^{2d+1} = F(x) $$
$$ F(x) = k_0 + k_1X + k_2X^2 + ... + k_{3d}X^{3d} $$
$$ (\acute a_0,...,\acute a_{3d}, \acute b_0,...,\acute b_{3d}) = (a_0 * k_0,...,a_{3d} * k_{3d}, b_0 * k_0,...,b_{3d} * k_{3d}) $$
$$ (\acute a_0,...,\acute a_{3d}) = α(\acute b_0,...,\acute b_{3d}) $$

Lastly, we would like to send these information with zero knowledge. To do so, we are going to use `Homomorphic Encryption`.

## Abstract

The `Homomorphic Encryption` achieves above equations evaluation without revealing any information. The difference between homomorphic encryption and normal encryption is that the `Homomorphic Encryption` can do add, sub and mul remaining encrypted. With this, the verifier doesn't know any information about these polynomials but able to check the relation between them.

## Details

Specifically, if the encryption supports `additive` and `multiplicative`, we would call it `Full Homomorphic Encryption` but it takes so much cost to calculate or ciphertext is too big to transfer through the internet. In this case, we deal the encryption which supports multiple additive and one time multiplicative. We realize these with `elliptic curve` and `pairing`.

## Elliptic Curve

The `elliptic curve` is the equation look like following.

$$ y^2 = x^3 + ax + b $$

## Pairing

The `pairing` is the mapping which takes two elliptic curve points and map to the element of finitie field as following.

$$ f: G * G -> F_p $$

There are some types of pairing functions but it's complicated so describing the relationship with simple model. Let's denote the two generators of elliptic as `G1`, `G2` and, each scalar as `a`, `b` and generator of finite field as `g`. We can express the relactionship between these two elliptic curve points and finite field element as following.

$$ f: aG1 * bG2 -> g^{ab} $$
