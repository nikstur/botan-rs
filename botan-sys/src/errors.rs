use cty::{c_char, c_int};

#[allow(clippy::upper_case_acronyms)]

pub type BOTAN_FFI_ERROR = c_int;

pub const BOTAN_FFI_SUCCESS: BOTAN_FFI_ERROR = 0;
pub const BOTAN_FFI_INVALID_VERIFIER: BOTAN_FFI_ERROR = 1;
pub const BOTAN_FFI_ERROR_INVALID_INPUT: BOTAN_FFI_ERROR = -1;
pub const BOTAN_FFI_ERROR_BAD_MAC: BOTAN_FFI_ERROR = -2;
pub const BOTAN_FFI_ERROR_INSUFFICIENT_BUFFER_SPACE: BOTAN_FFI_ERROR = -10;
pub const BOTAN_FFI_ERROR_EXCEPTION_THROWN: BOTAN_FFI_ERROR = -20;
pub const BOTAN_FFI_ERROR_OUT_OF_MEMORY: BOTAN_FFI_ERROR = -21;
pub const BOTAN_FFI_ERROR_SYSTEM_ERROR: BOTAN_FFI_ERROR = -22;
pub const BOTAN_FFI_ERROR_INTERNAL_ERROR: BOTAN_FFI_ERROR = -23;
pub const BOTAN_FFI_ERROR_BAD_FLAG: BOTAN_FFI_ERROR = -30;
pub const BOTAN_FFI_ERROR_NULL_POINTER: BOTAN_FFI_ERROR = -31;
pub const BOTAN_FFI_ERROR_BAD_PARAMETER: BOTAN_FFI_ERROR = -32;
pub const BOTAN_FFI_ERROR_KEY_NOT_SET: BOTAN_FFI_ERROR = -33;
pub const BOTAN_FFI_ERROR_INVALID_KEY_LENGTH: BOTAN_FFI_ERROR = -34;
pub const BOTAN_FFI_ERROR_INVALID_OBJECT_STATE: BOTAN_FFI_ERROR = -35;
pub const BOTAN_FFI_ERROR_NOT_IMPLEMENTED: BOTAN_FFI_ERROR = -40;
pub const BOTAN_FFI_ERROR_INVALID_OBJECT: BOTAN_FFI_ERROR = -50;
pub const BOTAN_FFI_ERROR_TLS_ERROR: BOTAN_FFI_ERROR = -75;
pub const BOTAN_FFI_ERROR_HTTP_ERROR: BOTAN_FFI_ERROR = -76;
pub const BOTAN_FFI_ERROR_UNKNOWN_ERROR: BOTAN_FFI_ERROR = -100;

extern "C" {

    pub fn botan_error_description(err: BOTAN_FFI_ERROR) -> *const c_char;

    #[cfg(feature = "botan3")]
    pub fn botan_error_last_exception_message() -> *const c_char;

}
