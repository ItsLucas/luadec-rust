use luadec::{decompile, decompile_file, Decompiler, DecompileError};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("LuaDec Rust Bindings - Basic Example\n");

    // Example 1: Decompiling from memory
    println!("=== Example 1: Decompiling from memory ===");
    
    // First, let's create some simple bytecode
    let simple_lua = b"print('Hello from decompiled Lua!')";
    
    // For this example, we'll use a pre-compiled bytecode
    // In practice, you'd read this from a .luac file
    match create_sample_bytecode() {
        Ok(bytecode) => {
            match decompile(&bytecode) {
                Ok(source) => {
                    println!("Decompiled source:");
                    println!("{}", source);
                }
                Err(e) => {
                    println!("Decompilation failed: {}", e);
                }
            }
        }
        Err(e) => {
            println!("Failed to create sample bytecode: {}", e);
        }
    }

    println!("\n=== Example 2: Using the Decompiler struct ===");
    
    let decompiler = Decompiler::new();
    
    // Try to decompile a file if it exists
    if let Ok(bytecode) = create_sample_bytecode() {
        match decompiler.decompile(&bytecode) {
            Ok(source) => {
                println!("Decompiled with Decompiler struct:");
                println!("{}", source);
            }
            Err(e) => {
                println!("Decompilation failed: {}", e);
            }
        }
    }

    println!("\n=== Example 3: Error handling ===");
    
    // Try to decompile invalid bytecode
    let invalid_data = b"This is not Lua bytecode";
    match decompile(invalid_data) {
        Ok(_) => println!("Unexpectedly succeeded!"),
        Err(e) => println!("Expected error for invalid bytecode: {}", e),
    }

    Ok(())
}

fn create_sample_bytecode() -> Result<Vec<u8>, std::io::Error> {
    // Try to read a test file, or create minimal bytecode
    if let Ok(data) = fs::read("/tmp/hello.luac") {
        Ok(data)
    } else {
        // Return some minimal "bytecode" (this won't actually work for decompilation)
        // In a real example, you'd have actual Lua bytecode
        println!("No test bytecode file found, using placeholder");
        Ok(vec![0x1b, 0x4c, 0x75, 0x61]) // Lua signature
    }
}