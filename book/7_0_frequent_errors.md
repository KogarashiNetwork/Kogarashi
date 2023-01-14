# Frequent Errors
The main errors happen during development of runtime pallet are followings.

- `error: duplicate lang item in crate`
- `error: the wasm32-unknown-unknown target is not supported by default, you may need to enable the "js" feature`
- `error[E0603]: module "group" is private`
- `error[E0512]: cannot transmute between types of different sizes, or dependently-sized types`
- `error[E0432]: unresolved import sp_core::to_substrate_wasm_fn_return_value`

Explaining causes and remedies.

## `error: duplicate lang item in crate`
This error happens when we use different version crate but same crate on one crate.
The error says the dependencies duplication so we can query the crate name as following.

```
$ cargo tree -e features -i {crate}
```

If we find the duplication of crate that we use same crate different version multiple times, we should align the version.

## `error: the wasm32-unknown-unknown target is not supported by default, you may need to enable the "js" feature`
This error happens `getrandom` crate dependency on `std`.  
We need to disable `std` feature of `getrandom`.  

Firstly, checking which libraries depend on `getrandom` depending on `std` to execute following command.

```
$ cargo tree -e features
```

`cargo tree` command displays the dependencies tree.  
The libraries with `(*)` doesn't depend on `std` but if there is `getrandom` not marked as `(*)`, it would cause error.

Secondly, independing from `std` library by followings.

- Add `default-features = false` to crate in `Cargo.toml` which is not marked as `(*)`
- Add `#![cfg_attr(not(feature = "std"), no_std)]` if imported crate is made by self.

And run `cargo tree` and check whether `getrandom` is marked as `(*)`

You can also use `cargo nono check` to check dependency on `std`.

```
$ cargo nono check
```

## `error[E0603]: module "group" is private`
This error happens `syn` crate because its interface was change.
We need to indicate exact version of `syn` as using expected behavior.

```
$ cargo update -p syn --precise 1.0.96
```

## `error[E0512]: cannot transmute between types of different sizes, or dependently-sized types`
This error happens on [`runtime-interface`](https://github.com/paritytech/substrate/blob/master/primitives/runtime-interface/src/impls.rs#L44) and both macro available when `#[cfg(all(not(feature = "std"), not(feature = "disable_target_static_assertions")))]` so we need to specify `std` as following.

```toml
[features]
default = ["std"]
std = [
    "crate/std"
]
```

## `error[E0432]: unresolved import sp_core::to_substrate_wasm_fn_return_value`
This error happens the crate which has `sp_api` dependency. And to clarify every crate which imported as `default-features = false` is described as `crate/std` in `[features]`.

```toml
[features]
default = ["std"]
std = [
    "crate/std"
]
```
