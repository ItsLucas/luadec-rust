# Examples

This directory contains examples demonstrating how to use the luadec crate.

## Running Examples

To run an example:

```bash
cargo run --example basic_decompile
```

For the file decompile example, you need a Lua bytecode file:

```bash
# Create a test bytecode file
echo 'print("Hello World")' | luac -o test.luac -

# Decompile it
cargo run --example file_decompile test.luac
```

## Examples

- `basic_decompile.rs` - Basic usage demonstrating memory-based decompilation
- `file_decompile.rs` - Command-line utility to decompile .luac files

## Creating Test Bytecode

You can create Lua bytecode files using the `luac` compiler:

```bash
# From a file
luac -o output.luac input.lua

# From stdin
echo 'your lua code here' | luac -o output.luac -
```

The bytecode files can then be decompiled using this library.