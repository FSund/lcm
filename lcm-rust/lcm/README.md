# LCM Bindings for Rust

[![crates.io](http://meritbadge.herokuapp.com/lcm)](https://crates.io/crates/lcm)
[![Build Status](https://travis-ci.org/adeschamps/lcm.svg?branch=rust)](https://travis-ci.org/adeschamps/lcm)

This crate provides Rust bindings for [LCM](http://lcm-proj.github.io).

## Requirements

- A C compiler (gcc, clang, or MSVC)
- [LCM](http://lcm-proj.github.io) library installed on your system (version 1.3.1 or later recommended)

## Usage

To generate Rust code for your LCM message definitions, you must use [this fork] of `lcm-gen`.
See also the [lcm_gen](https://crates.io/crates/lcm_gen) crate to integrate this into a Cargo build.

### `lcm-gen`

To just build `lcm-gen` from this repository, run:

```sh
mkdir build
cd build
cmake -DCMAKE_INSTALL_PREFIX=/usr -DLCM_ENABLE_EXAMPLES=OFF -DLCM_ENABLE_PYTHON=OFF -DLCM_ENABLE_JAVA=OFF -DLCM_ENABLE_LUA=OFF -DLCM_ENABLE_GO=OFF ..
make -j4
```

Then you can run `./lcmgen/lcm-gen` on your `.lcm` files to generate Rust code.
