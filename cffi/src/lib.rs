#![crate_type = "dylib"]

use libc::{c_char, c_void};
use quizdown_lib as qd;
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

pub(crate) fn result_to_ffi(rust_result: Result<String, Box<dyn Error>>) -> *const FFIResult {
    let mut c_result = Box::new(FFIResult::default());
    match rust_result {
        Ok(item) => {
            c_result.success = return_string(&item);
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

#[no_mangle]
pub extern "C" fn available_themes() -> *const c_void {
    let mut output = String::new();
    for theme in qd::list_themes() {
        if output.len() > 0 {
            output.push('\t');
        }
        output.push_str(&theme);
    }
    return_string(&output)
}

#[no_mangle]
pub extern "C" fn default_config() -> *const c_void {
    return_string(&serde_json::to_string(&qd::Config::default()).unwrap())
}

/// This is our main interface to the library.
#[no_mangle]
pub extern "C" fn parse_quizdown(
    text: *const c_void,
    name: *const c_void,
    format: *const c_void,
    config: *const c_void,
) -> *const FFIResult {
    result_to_ffi(try_parse_quizdown(text, name, format, config))
}

fn try_parse_quizdown(
    text: *const c_void,
    name: *const c_void,
    format: *const c_void,
    config: *const c_void,
) -> Result<String, Box<dyn Error>> {
    let text = accept_str("text-to-parse", text)?;
    let name = accept_str("quiz_name", name)?;
    let format: qd::OutputFormat = serde_json::from_str(accept_str("format", format)?)?;
    let config: qd::Config = serde_json::from_str(accept_str("config", config)?)?;

    let parsed = qd::process_questions_str(text, Some(config))
        .map_err(|e| format!("Parsing Error: {}", e))?;
    Ok(format
        .render(name, &parsed)
        .map_err(|e| format!("Rendering Error: {}", e))?)
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
