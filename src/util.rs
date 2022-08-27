use std::ffi::CString;
use std::vec;

pub(crate) fn string_to_c_string(string: &str) -> CString {
    let mut bytes = vec![0; string.len() + 1];
    bytes[..string.len()].copy_from_slice(string.as_bytes());

    unsafe { CString::from_vec_with_nul_unchecked(bytes) }
}