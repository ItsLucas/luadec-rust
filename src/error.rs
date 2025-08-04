use thiserror::Error;
use std::io;

/// Errors that can occur during Lua bytecode decompilation
#[derive(Error, Debug)]
pub enum DecompileError {
    #[error("Invalid bytecode: {0}")]
    InvalidBytecode(String),
    
    #[error("Failed to create Lua state")]
    LuaStateCreationFailed,
    
    #[error("Failed to load bytecode: {0}")]
    LoadFailed(String),
    
    #[error("Decompilation failed: {0}")]
    DecompilationFailed(String),
    
    #[error("Internal error: {0}")]
    InternalError(String),
    
    #[error("Null pointer error")]
    NullPointer,
    
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
}

pub type Result<T> = std::result::Result<T, DecompileError>;