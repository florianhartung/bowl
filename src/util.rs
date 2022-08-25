use std::ffi::CString;
use std::os::raw::c_char;
use std::vec;

pub(crate) fn string_to_c_string(string: String) -> CString {
    let mut bytes = vec![0; string.len() + 1];
    bytes[..string.len()].copy_from_slice(string.as_bytes());

    unsafe { CString::from_raw(bytes.as_mut_ptr() as *mut c_char) }
}