use core::slice;
use crate::sys;
use std::ptr;
use log::error;
use crate::wrapper::to_modified_utf8;
use crate::sys::jboolean;
use std::os::raw::c_char;

pub fn to_bool(jbool: jboolean) -> bool {
    if jbool > 0 { true } else { false }
}

pub fn to_jboolean(value: bool) -> jboolean {
    if value { 1 } else { 0 }
}

pub fn slice_raw<'a, T>(data: *const T, len: sys::jint) -> &'a [T] {
    unsafe {
        if len == 0 || data.is_null() {
            return &[];
        }
        return slice::from_raw_parts(data, len as usize);
    }
}

pub fn stringify(input: *const c_char) -> String {
    to_modified_utf8(input).into()
}
