# Twisted Edwards Curve
[![crates.io badge](https://img.shields.io/crates/v/zero-jubjub.svg)](https://crates.io/crates/zero-jubjub)  
The [`Twisted Edwards Curve`](https://eprint.iacr.org/2008/013.pdf) implementation. The arithmetics are optimized by the `assembly`.

## Algorithm Driven Development 💩

### Abstruct

The cryptographic libraries need efficiency because it directly affect the usability. If it's not efficient, we would be necessary to pay expensive gas cost that we are dried up and wait thousand days to transfer money. The efficiency depends on algorithm, parallelize and language specification from largest to smallest. It's the most important to use proper algorithm but we don't know which one is the best now. We should architect so that we can change the algorithm. This architecture focuses on how we can change the algorithm easily without any extra changes.

### Layer

There mainly three layers `interface`, `algorithm` and `arithmetic`. The `interface` defines the behavior and the things that entity achieves. The `algorithm` describes the logic how to achieve the `interface` behavior. The `arithmetic` performs the basic arithmetic which is necessary for `algorithm`. It's okay to add some `helper` if you want.

- Interface(entity): Curve, Field and Group, PairingEngine etc...
- Algorithm(algorithm): Jacobian, Montgomery Reduction and Projective, Affine etc...
- Arithmetic(arithmetic): Multiplication, Inversion and Addition, Subtraction etc...
- Basic Macro(dress): Basic Arithmetic and etc...

## Feature

| Name | Description | Usage |
| ---- | ---- | ---- |
| std | Feature for client | Generate proof with parallelize |
| no_std | Feature for Substrate runtime | Implement runtime module |
| asm | Feature for intel x86 | Replace arithmetic with assembly |
| parity | Feature for pallet | Derive struct as SCALE |
