use luadec_sys::{
    DecompileResult, luadec_decompile_buffer, luadec_free_result, 
    luadec_get_result, luadec_get_error
};
use std::ffi::CStr;
use crate::error::{DecompileError, Result};

/// Safe wrapper around the C decompilation function
pub fn decompile_bytecode_raw(bytecode: &[u8]) -> Result<String> {
    if bytecode.is_empty() {
        return Err(DecompileError::InvalidBytecode("Empty bytecode".to_string()));
    }
    
    let result_ptr = unsafe {
        luadec_decompile_buffer(
            bytecode.as_ptr() as *const i8,
            bytecode.len(),
        )
    };
    
    if result_ptr.is_null() {
        return Err(DecompileError::InternalError("Failed to get result".to_string()));
    }
    
    // Ensure we always free the result, even if we return early
    let _guard = ResultGuard(result_ptr);
    
    // Check for errors first
    let error_ptr = unsafe { luadec_get_error(result_ptr) };
    if !error_ptr.is_null() {
        let error_cstr = unsafe { CStr::from_ptr(error_ptr) };
        let error_msg = error_cstr.to_string_lossy().to_string();
        return Err(DecompileError::DecompilationFailed(error_msg));
    }
    
    // Get the result
    let result_cstr_ptr = unsafe { luadec_get_result(result_ptr) };
    if result_cstr_ptr.is_null() {
        return Err(DecompileError::InternalError("No result returned".to_string()));
    }
    
    let result_cstr = unsafe { CStr::from_ptr(result_cstr_ptr) };
    let result_string = result_cstr.to_string_lossy().to_string();
    
    Ok(result_string)
}

/// RAII guard to ensure C memory is freed
struct ResultGuard(*mut DecompileResult);

impl Drop for ResultGuard {
    fn drop(&mut self) {
        unsafe {
            luadec_free_result(self.0);
        }
    }
}