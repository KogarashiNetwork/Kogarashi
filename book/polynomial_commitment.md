# Polynomial Commitment

In previous [section](qap.md), we enable to check whether `computation` was done correctly by the knowledge of polynomial which can be devided by minimal polynomial `Z(x)` as following.

$$ L(x) * R(x) - O(x) = Z(x) * T(x) $$

However, we can create equation easily because `Z(x)` is public information. Then we need to verify whether `L(x), R(x), O(x)` are created by using valid input. To do so, we are going to use `Polynomial Commitment`.

## Abstract
The `Polynomial Commitment` check whether the prover know polynomials `L(x), R(x), O(x)` and these comes from valid input.

## Details

To know polynomials `L(x), R(x), O(x)` means having knowledge of coefficients of them.

## Combination

First of all, we combine these polynomials to one in order to make check process easier. Let's say degree of polynomials `L(x), R(x), O(x)` as `d`, we can combine them into one as following and let combined polynomial as `F(x)`.

$$ L(x) + R(x) * X^{d+1} + O(x) * X^{2d+1} = F(x) $$

In polynomial `F(x)`, the coefficients of `0~d` degree expresses `L(x)` coefficients, `d+1~2d` is `R(x)` and `2d+1~3d` is `O(x)` as well. The polynomial `F(x)` degree is `3d` and when we denote coefficients as k, it would be following.

$$ F(x) = k_0 + k_1X + k_2X^2 + ... + k_{3d}X^{3d} $$

## Verification

The verification processes are following.

1. Bob choses random `α, (a_0,...,a_{3d}) ∈ F` and compute `(b_0,...,b_{3d}) = α(a_0,...,a_{3d})`.
2. Bob sends Alice to `(a_0,...,a_{3d})` and `(b_0,...,b_{3d})`.
3. Alice computes following.
$$ (\acute a_0,...,\acute a_{3d}, \acute b_0,...,\acute b_{3d}) = (a_0 * k_0,...,a_{3d} * k_{3d}, b_0 * k_0,...,b_{3d} * k_{3d}) $$
4. Bob checks following.
$$ (\acute a_0,...,\acute a_{3d}) = α(\acute b_0,...,\acute b_{3d}) $$

If Alice don't know the coefficients, she couldn't do step `3`. With using this step, we can know that the prover know polynomials `L(x), R(x), O(x)` and these comes from valid input.

## Next
In this section, we understood how to check the prover polynomials comming from valid input but these information would be known by verifier. To hide these information from verifier, we are using homomorphic encryption.
