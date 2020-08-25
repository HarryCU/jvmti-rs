use cesu8::{from_java_cesu8, Cesu8DecodingError};
use log::error;
use std::borrow::Cow;
use std::os::raw::c_char;
use std::ffi::CStr;

pub fn to_modified_utf8<'a>(input: *const c_char) -> Cow<'a, str> {
    unsafe {
        let bytes = CStr::from_ptr(input).to_bytes();
        match from_java_cesu8(bytes) {
            Ok(s) => s,
            Err(e) => {
                error!("error decoding java cesu8: {:#?}", e);
                String::from_utf8_lossy(bytes)
            }
        }
    }
}