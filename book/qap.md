# QAP (Quadratic Arithmetic Programs)

## Abstract
The `QAP` is the technology which converts `computation` to `polynomial groups`. With this, we can check whether the `computation` was executed correctly just factor the polynomial without execute `computation` again.

## Details
Let's take a look at details. I give a example. Let's prove that following computation was executed correctly.

$$a^2 \cdot b^2 = c.$$

Assume that `c` is public input. `a` and `b` are private input. Prove that knowledge of `a` and `b` satisfying above equation.

### Flattening
First of all, let's disassemble the `computation` to minimum form using multicative.

$$ 1: a * a = a^2 $$
$$ 2: b * b = b^2 $$
$$ 3: a^2 * b^2 = c $$

Now the computation was disassembled to three gate.

### R1Cs
As described, we have three multicative computation and want to check whether each steps are executed correctly to set constraint. Before that, we permute above characters as following.

$$ [a, a^2, b, b^2, c] -> [v, w, x, y, z] $$

And now, our computation can be expressed as following table. Left, Right and Output.

Gate | L | R | O
:------------ | :------------- | :------------- | :-------------
1 | v | v | w
2 | x | x | y
3 | y | w | z

### QAP
Let's express above table as polynomail groups. As example, express `v` polynomial on `L` column. `x` cordinate is `Gate` number and `y` cordinate is if that variable is used, it's going to be 1 and oserwise 0. In `L` column, `v` is only used `Gate` 1 so express as `(1, 1) (2, 0) (3, 0)`. Find polynomial using [`Lagrange interpolation formula`](https://math.iitm.ac.in/public_html/sryedida/caimna/interpolation/lagrange.html) for each variables.

*L column polynomial*

Variable | Cordinate | Polynomial | Name
:------------ | :------------- | :------------- | :-------------
v | (1, 1) (2, 0) (3, 0) | $$ \frac{x^2}{2} - \frac{5x}{2} + 3 \\ $$ | Lv
w | (1, 0) (2, 0) (3, 0) | 0 | Lw
x | (1, 0) (2, 1) (3, 0) | $$ -x^2 + 4x - 3 $$ | Lx
y | (1, 0) (2, 0) (3, 1) | $$ \frac{x^2}{2} - \frac{3x}{2} + 1 \\ $$ | Ly
z | (1, 0) (2, 0) (3, 0) | 0 | Lz

Above polynomial expresses the gate that variable uses. When we pass gate number to polynomial `v` $$ \frac{x^2}{2} - \frac{5x}{2} + 3 \\ $$, we can know which gate the `v` is used. For example, we pass `1` to polynomial `v`, it returns `1` so the variable `v` is used on gate `1` but it returns `0` when we pass `2` and `3` so it's not used on these gate. When we add all polynomial `Lv + Lw + Lx + Ly + Lz = L(x)`, it returns `1` when we pass `1`, `2` and `3`. We do the same operation for each column and get polynomials as well.

- L(x)  
`Lv + Lw + Lx + Ly + Lz`  
- R(x)  
`Rv + Rw + Rx + Ry + Rz`  
- O(x)  
`Ov + Ow + Ox + Oy + Oz`  

We can intruduce above polynomials when we decide the `computation`.

### Proof
From now on, we are going to prove the state ment. In here, we use `a = 2`, `b = 3` and `c = 36`. We can get actual value as following.

$$ [v, w, x, y, z] -> [2, 4, 3, 9, 36] $$

And we multiply above variables by for each polynomial. For example, `L(x)` is following.

$$ L(x) = v * Lv + w * Lw + x * Lx + y * Ly + z * Lz $$

When we pass the `1` to `L(x)`, we can get `v` because only `Lv` returns `1` and others return `0`. `R(x)` as well and `O(x)` returns `w` so following equation holds.

$$ L(1) * R(1) - O(1) = v * v - w = 2 * 2 - 4 = 0 $$

It corresponds the table we saw in `R1Cs` and above also holds the case `x = 2` and `x = 3`. When we want to prove the statement, we are going to make above polynomial with secret `[v, w, x, y, z] -> [2, 4, 3, 9, 36]` so polynomial would be integrated as one as following.

$$ L(x) = 2 * Lv + 4 * Lw + 3 * Lx + 9 * Ly + 36 * Lz $$
$$ R(x) = 2 * Rv + 4 * Rw + 3 * Rx + 9 * Ry + 36 * Rz $$
$$ O(x) = 2 * Ov + 4 * Ow + 3 * Ox + 9 * Oy + 36 * Oz $$
$$ L(x) * R(x) - O(x) = P(x) $$

We make the `P(x)` to prove the statement.

### Verification
We can know whether `computation` was executed correctly to devide `P(x)` with `(x - 1) * (x - 2) * (x - 3)`. If it's devided as following prover knows the secret `a` and `b` leading `c`.

$$ P(x) = (x - 1) * (x - 2) * (x - 3) * T(x) $$

### Next
In this section, we understood how to convert `computation` to `polynomial groups` but there are some possibility that `P(x)` was made without using secret. In addition to this, we can know the secret to factor the polynomial. Zk SNARKs addresses the former problem with `Polynomial Commitment` and latter problem with `homomorphic encryption`.
