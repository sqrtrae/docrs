# docrs
A command line utility for opening rust crate documentation on https://docs.rs using your default web browser. Currently only supports opening the latest version of a crate.

This utility is intentionally simple and lacking in features, as my main motivation for creating it was to have a simple crate I could use to learn the steps of uploading a crate to https://crates.io.

# Installation

`docrs` can be installed via cargo:
```shell
cargo install docrs
```
The MSRV is currently `1.74.0`.

# Usage

Simply type in `docrs` followed by a crate to open that crates documentation in your default web browser. Example using `clap`:

```shell
docrs clap
```
