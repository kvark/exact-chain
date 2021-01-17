# exact-chain

[![crates](https://img.shields.io/crates/v/exact-chain.svg)](https://crates.io/crates/exact-chain)
[![docs](https://img.shields.io/badge/docs.rs-exact--chain)](https://docs.rs/exact-chain)
[![Build Status](https://github.com/kvark/exact-chain/workflows/Check/badge.svg)](https://github.com/kvark/exact-chain)

The chained iterator that implements `ExactSizeIterator`, unlike the `std::iter::Chain`.
It panics if the exact size of the chain exceeds the `usize` bounds.
