#![crate_type = "dylib"]

use libc::{c_char, c_void};
use quizdown::{process_questions_str, Question};
use std::error::Error;
use std::ffi::{CStr, CString};
use std::ptr;

#[macro_use]
extern crate serde_derive;

/// This is a JSON-API, not a C-API, really.
#[derive(Serialize, Deserialize)]
struct ErrorMessage {
    error: String,
    context: String,
}

#[repr(C)]
pub struct FFIResult {
    /// Non-null if there's an error.
    pub error_message: *const c_void,
    /// Non-null if we succeeded.
    pub success: *const c_void,
}

impl Default for FFIResult {
    fn default() -> Self {
        FFIResult {
            error_message: ptr::null(),
            success: ptr::null(),
        }
    }
}

/// Accept a string parameter!
pub(crate) fn accept_str(name: &str, input: *const c_void) -> Result<&str, Box<dyn Error>> {
    if input.is_null() {
        Err(format!("NULL pointer: {}", name))?;
    }
    let input: &CStr = unsafe { CStr::from_ptr(input as *const c_char) };
    Ok(input
        .to_str()
        .map_err(|_| format!("Could not parse {} pointer as UTF-8 string!", name))?)
}

/// Internal helper: convert string reference to pointer to be passed to Python/C. Heap allocation.
pub(crate) fn return_string(output: &str) -> *const c_void {
    let c_output: CString = CString::new(output).expect("Conversion to CString should succeed!");
    CString::into_raw(c_output) as *const c_void
}

pub(crate) fn result_to_ffi<T>(rust_result: Result<T, Box<dyn Error>>) -> *const FFIResult
where
    T: serde::Serialize,
{
    let mut c_result = Box::new(FFIResult::default());
    match rust_result {
        Ok(item) => {
            c_result.success = return_string(&serde_json::to_string(&item).unwrap());
        }
        Err(e) => {
            let error_message = serde_json::to_string(&ErrorMessage {
                error: "error".to_string(),
                context: format!("{:?}", e),
            })
            .unwrap();
            c_result.error_message = return_string(&error_message);
        }
    };
    Box::into_raw(c_result)
}

/// This is our main interface to the library.
#[no_mangle]
pub extern "C" fn parse_quizdown(text: *const c_void) -> *const FFIResult {
    result_to_ffi(try_parse_quizdown(text))
}

fn try_parse_quizdown(text: *const c_void) -> Result<Vec<Question>, Box<dyn Error>> {
    let text = accept_str("text-to-parse", text)?;
    let result = process_questions_str(text)?;
    Ok(result)
}

/// Returns true if it received a non-null string to free.
#[no_mangle]
pub extern "C" fn free_str(originally_from_rust: *mut c_void) -> bool {
    if originally_from_rust.is_null() {
        return false;
    }
    let _will_drop: CString = unsafe { CString::from_raw(originally_from_rust as *mut c_char) };
    true
}

/// Note: not-recursive. Free Error Message or Result Manually!
#[no_mangle]
pub extern "C" fn free_ffi_result(originally_from_rust: *mut FFIResult) {
    let _will_drop: Box<FFIResult> = unsafe { Box::from_raw(originally_from_rust) };
}
