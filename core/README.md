# cfgloader-rs-core

> Core functionality for CFGLoader

This crate provides the core types, traits, and utilities for the CFGLoader configuration loading library. It is typically not used directly - instead, use the main [`cfgloader-rs`](https://crates.io/crates/cfgloader-rs) crate which re-exports everything you need.

## What's in this crate

- `FromEnv` trait for types that can be loaded from environment variables
- `CfgError` enum for configuration loading errors  
- Utility functions for parsing and loading environment variables
- `.env` file loading support via `dotenvy`

## Usage

This is an internal crate. For usage examples and documentation, please see the main [`cfgloader-rs`](https://crates.io/crates/cfgloader-rs) crate.

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.
