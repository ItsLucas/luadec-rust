# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust crate that provides safe bindings for LuaDec, a Lua 5.1 bytecode decompiler. The project consists of two main components:

- **luadec-sys**: Low-level FFI bindings to the C LuaDec library (unsafe)
- **luadec**: High-level safe Rust API wrapper

The crate allows decompiling Lua 5.1 bytecode files (.luac) back into readable Lua source code.

## Build System

### Basic Commands
```bash
# Build the project (includes C compilation of Lua 5.1 and LuaDec)
cargo build

# Run tests
cargo test

# Build with release optimizations
cargo build --release

# Run examples
cargo run --example basic_decompile
cargo run --example file_decompile

# Check code without building
cargo check

# Generate documentation
cargo doc --open
```

### Build Dependencies
The build process requires:
- C compiler (gcc/clang)
- make
- Platform-specific tools for Lua 5.1 compilation

The build.rs script automatically:
1. Builds Lua 5.1 from vendor/lua-5.1 using appropriate platform targets (macosx/linux/generic)
2. Builds LuaDec from vendor/luadec
3. Creates C wrapper bindings
4. Links everything together as static libraries

## Architecture

### Directory Structure
- **src/**: Main library code
  - `lib.rs`: Public API with `decompile()` functions and `Decompiler` struct
  - `ffi.rs`: Safe wrappers around raw FFI calls
  - `error.rs`: Error types and handling
- **luadec-sys/**: Low-level FFI bindings
  - `src/lib.rs`: Raw FFI function declarations
  - `src/wrapper.c`: C wrapper code for the LuaDec library
  - `build.rs`: Build script for C compilation
- **examples/**: Usage examples
- **vendor/**: External C dependencies (Lua 5.1 and LuaDec source)

### Key Components

1. **Public API** (src/lib.rs):
   - `decompile(&[u8]) -> Result<String>`: Decompile bytecode from memory
   - `decompile_file<P: AsRef<Path>>(P) -> Result<String>`: Decompile from file
   - `Decompiler` struct: Reusable decompiler instance

2. **FFI Layer** (luadec-sys):
   - Raw bindings to LuaDec C functions
   - Platform-specific build configuration
   - Memory safety handled at higher level

3. **Error Handling**:
   - `DecompileError` enum covering I/O, invalid bytecode, and FFI errors
   - Proper error propagation from C code to Rust

## Development Notes

### Platform Support
- Primary support: Linux and macOS
- Build script detects platform and uses appropriate Lua 5.1 build targets
- Links platform-specific system libraries (libm, libdl on Linux; libc on macOS)

### Testing
- Unit tests focus on error handling and edge cases
- No integration tests with actual bytecode (would require test fixtures)
- Examples serve as functional tests

### Safety Considerations
- All unsafe FFI operations are wrapped in safe Rust APIs
- C library state is isolated and doesn't leak across function calls
- Memory management handled properly between Rust and C boundaries

### Vendor Dependencies
The project includes C source code in vendor/ directories that must be compiled during build. This requires:
- Consistent C toolchain across development environments
- Proper handling of cross-compilation scenarios
- Build script maintenance for new platforms