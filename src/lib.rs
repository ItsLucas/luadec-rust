//! # LuaDec - Lua 5.1 Bytecode Decompiler
//!
//! This crate provides Rust bindings for the LuaDec library, which decompiles
//! Lua 5.1 bytecode back into readable Lua source code.
//!
//! ## Example
//!
//! ```rust,no_run
//! use luadec::{decompile, DecompileError};
//! use std::fs;
//!
//! fn main() -> Result<(), DecompileError> {
//!     // Read compiled Lua bytecode
//!     let bytecode = fs::read("example.luac")?;
//!     
//!     // Decompile to Lua source code
//!     let source = decompile(&bytecode)?;
//!     
//!     println!("Decompiled source:\n{}", source);
//!     Ok(())
//! }
//! ```

pub mod error;
pub mod ffi;

pub use error::{DecompileError, Result};

use std::fs;
use std::path::Path;

/// Decompile Lua 5.1 bytecode from a byte slice
/// 
/// # Arguments
/// 
/// * `bytecode` - The compiled Lua bytecode as a byte slice
/// 
/// # Returns
/// 
/// Returns the decompiled Lua source code as a String, or a DecompileError if
/// decompilation fails.
/// 
/// # Example
/// 
/// ```rust,no_run
/// use luadec::decompile;
/// use std::fs;
/// 
/// let bytecode = fs::read("hello.luac")?;
/// let source = decompile(&bytecode)?;
/// println!("{}", source);
/// # Ok::<(), luadec::DecompileError>(())
/// ```
pub fn decompile(bytecode: &[u8]) -> Result<String> {
    ffi::decompile_bytecode_raw(bytecode)
}

/// Decompile Lua 5.1 bytecode from a file
/// 
/// # Arguments
/// 
/// * `path` - Path to the compiled Lua bytecode file (.luac)
/// 
/// # Returns
/// 
/// Returns the decompiled Lua source code as a String, or a DecompileError if
/// reading the file or decompilation fails.
/// 
/// # Example
/// 
/// ```rust,no_run
/// use luadec::decompile_file;
/// 
/// let source = decompile_file("hello.luac")?;
/// println!("{}", source);
/// # Ok::<(), luadec::DecompileError>(())
/// ```
pub fn decompile_file<P: AsRef<Path>>(path: P) -> Result<String> {
    let bytecode = fs::read(path)?;
    decompile(&bytecode)
}

/// A high-level decompiler interface for more advanced usage
pub struct Decompiler {
    // Future: could add configuration options here
    // For now, we use the default behavior
}

impl Decompiler {
    /// Create a new decompiler instance
    pub fn new() -> Self {
        Self {}
    }
    
    /// Decompile bytecode with this decompiler instance
    pub fn decompile(&self, bytecode: &[u8]) -> Result<String> {
        decompile(bytecode)
    }
    
    /// Decompile a file with this decompiler instance
    pub fn decompile_file<P: AsRef<Path>>(&self, path: P) -> Result<String> {
        decompile_file(path)
    }
}

impl Default for Decompiler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_empty_bytecode() {
        let result = decompile(&[]);
        assert!(result.is_err());
        match result.unwrap_err() {
            DecompileError::InvalidBytecode(_) => {},
            other => panic!("Expected InvalidBytecode error, got: {:?}", other),
        }
    }
    
    #[test]
    fn test_invalid_bytecode() {
        let invalid_bytecode = b"not lua bytecode";
        let result = decompile(invalid_bytecode);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_decompiler_struct() {
        let decompiler = Decompiler::new();
        let result = decompiler.decompile(&[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_concurrent_decompile() {
        use std::thread;
        use std::sync::Arc;
        
        let invalid_bytecode = Arc::new(vec![1, 2, 3, 4, 5]);
        let mut handles = vec![];
        
        // Spawn multiple threads that try to decompile simultaneously
        for _ in 0..10 {
            let bytecode = invalid_bytecode.clone();
            let handle = thread::spawn(move || {
                let result = decompile(&bytecode);
                // All should fail with invalid bytecode, but shouldn't crash
                assert!(result.is_err());
            });
            handles.push(handle);
        }
        
        // Wait for all threads to complete
        for handle in handles {
            handle.join().expect("Thread panicked");
        }
    }

}