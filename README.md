# luadec-rust

[![Crates.io](https://img.shields.io/crates/v/luadec-rust.svg)](https://crates.io/crates/luadec-rust)
[![Documentation](https://docs.rs/luadec-rust/badge.svg)](https://docs.rs/luadec-rust)

Safe Rust bindings for [LuaDec](https://github.com/viruscamp/luadec), a Lua 5.1 bytecode decompiler.

This crate provides a safe, high-level API for decompiling Lua 5.1 bytecode. It wraps the unsafe FFI bindings to LuaDec in a memory-safe interface.

## Requirements

- C compiler (gcc/clang)
- make
- Lua 5.1 source code (included as git submodule)

## Platform Support

- Linux (tested)
- macOS (tested)
- Other Unix-like systems (should work)

## Safety

This crate provides a safe interface to LuaDec by wrapping the unsafe FFI calls in memory-safe abstractions. All public APIs are designed to prevent undefined behavior and memory safety issues.

## License

This project follows the same license as the original LuaDec project. Lua itself is licensed under the MIT license.