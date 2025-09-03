# LCM Bindings for Rust

[![crates.io](http://meritbadge.herokuapp.com/lcm)](https://crates.io/crates/lcm)
[![Build Status](https://travis-ci.org/adeschamps/lcm.svg?branch=rust)](https://travis-ci.org/adeschamps/lcm)

This crate provides Rust bindings for [LCM](http://lcm-proj.github.io) with a **statically linked LCM C library**.

**New in v0.3.0**: The LCM C library is now compiled and statically linked during the build process. You no longer need to install `liblcm` separately on your system.

## Features

- Self-contained: No external LCM library installation required
- Cross-platform: Automatically handles platform-specific build configurations
- Static linking: The LCM C library is built and linked statically

## Requirements

- A C compiler (gcc, clang, or MSVC)
- For Unix-like systems: GLib 2.0 development headers (`libglib2.0-dev` on Debian/Ubuntu, `glib2-devel` on RHEL/CentOS)
- For Windows: No additional dependencies required

## Usage

To generate Rust code for your LCM message definitions, you must use [this fork] of `lcm-gen`.
See also the [lcm_gen](https://crates.io/crates/lcm_gen) crate to integrate this into a Cargo build.
