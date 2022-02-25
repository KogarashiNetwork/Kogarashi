#![cfg_attr(not(feature = "std"), no_std)]
#![feature(asm)]
#![allow(dead_code)]

mod arithmetic;
mod error;
mod fr;
mod operation;

pub use fr::Fr;
