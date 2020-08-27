use std::os::raw::c_char;
use crate::*;
use std::str;
use std::borrow::Cow;
use crate::sys::jmemory;
use crate::objects::JDeallocate;

pub struct JvmtiString<'a> {
    ptr: *mut c_char,
    env: &'a JVMTIEnv<'a>,
}

impl<'a> JvmtiString<'a> {
    pub fn with_pointer<'b: 'a>(ptr: *mut c_char, env: &'b JVMTIEnv<'a>) -> JvmtiString<'a> {
        JvmtiString {
            ptr,
            env,
        }
    }

    pub fn as_ptr(&mut self) -> *mut c_char {
        self.ptr
    }

    pub fn deallocate(&self) {
        if !self.ptr.is_null() {
            self.env.deallocate(self).unwrap()
        }
    }
}


impl<'a> From<&JvmtiString<'a>> for Cow<'a, str> {
    fn from(other: &JvmtiString) -> Cow<'a, str> {
        if other.ptr.is_null() {
            return "".into();
        }
        to_modified_utf8(other.ptr as *const c_char)
    }
}

impl<'a> From<&JvmtiString<'a>> for String {
    fn from(other: &JvmtiString) -> String {
        let cow: Cow<str> = other.into();
        cow.into()
    }
}

impl<'a> From<JvmtiString<'a>> for Cow<'a, str> {
    fn from(other: JvmtiString) -> Cow<'a, str> {
        if other.ptr.is_null() {
            return "".into();
        }
        to_modified_utf8(other.ptr as *const c_char)
    }
}

impl<'a> From<JvmtiString<'a>> for String {
    fn from(other: JvmtiString) -> String {
        let cow: Cow<str> = other.into();
        cow.into()
    }
}

impl<'a> Drop for JvmtiString<'a> {
    fn drop(&mut self) {
        self.deallocate()
    }
}

impl<'a> JDeallocate<'a> for JvmtiString<'a> {
    fn as_deallocate_ptr(&self) -> jmemory {
        self.ptr as jmemory
    }
}