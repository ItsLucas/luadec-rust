# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2024-08-04

### Added
- Initial release of luadec Rust bindings
- Safe Rust API for LuaDec C library
- Support for decompiling Lua 5.1 bytecode
- Cross-platform support (Linux, macOS)
- Comprehensive error handling with `DecompileError` enum
- Memory-safe FFI bindings with automatic cleanup
- `decompile()` function for bytecode from memory
- `decompile_file()` function for bytecode from files
- `Decompiler` struct for advanced usage
- Basic examples and documentation
- Split architecture with `luadec-sys` for raw FFI bindings

### Features
- Zero-copy bytecode handling
- Automatic C library compilation via build script
- RAII memory management
- Ergonomic error handling
- Complete API documentation