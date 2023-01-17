# Jubjub Curve [![crates.io badge](https://img.shields.io/crates/v/zero-jubjub.svg)](https://crates.io/crates/zero-jubjub)
This crate provides jubjub curve arithmetic and also supports fully `no_std` and [`parity-scale-codec`](https://github.com/paritytech/parity-scale-codec).

**This crate uses [https://github.com/zkcrypto/jubjub](https://github.com/zkcrypto/jubjub) algorithm designed by @str4d and @ebfull.**
We replace field and curve implementation with `zero-crypto` to make this compatible with `Substrate`.

# Specification
The Jubjub curve is one of twisted edwards curve.

- Twisted Edwards Curve
$$
ax^2 + y^2 = 1 + dx^2y^2
$$

- Addition Law
$$
(x_3 = \frac{x_1y_1 + y_1x_1}{1 + dx_1x_1y_1y_1}, y_3 = \frac{y_1y_1 + ax_1x_1}{1 - dx_1x_1y_1y_1})
$$
