use thiserror::Error;
use std::os::raw::{c_char, c_uchar};
use crate::wrapper::*;
use std::{str, ptr};
use std::ffi::CStr;
use std::string::FromUtf8Error;
use jni_sys::jlong;
use std::marker::PhantomData;
use std::borrow::{Borrow, Cow};
use crate::sys::{JNIString, jmemory, JNIStr};
use cesu8::from_java_cesu8;
use log::{debug, error};

pub struct JString<'a> {
    ptr: *mut c_char,
    env: &'a JVMTIEnv<'a>,
}

impl<'a> JString<'a> {
    pub fn with_pointer<'b: 'a>(ptr: *mut c_char, env: &'b JVMTIEnv<'a>) -> JString<'a> {
        JString {
            ptr,
            env,
        }
    }

    pub fn as_ptr(&mut self) -> *mut c_char {
        self.ptr
    }
}


impl<'a> From<&JString<'a>> for Cow<'a, str> {
    fn from(other: &JString) -> Cow<'a, str> {
        to_modified_utf8(other.ptr as *const c_char)
    }
}

impl<'a> From<&JString<'a>> for String {
    fn from(other: &JString) -> String {
        let cow: Cow<str> = other.into();
        cow.into()
    }
}

impl<'a> From<JString<'a>> for Cow<'a, str> {
    fn from(other: JString) -> Cow<'a, str> {
        to_modified_utf8(other.ptr as *const c_char)
    }
}

impl<'a> From<JString<'a>> for String {
    fn from(other: JString) -> String {
        let cow: Cow<str> = other.into();
        cow.into()
    }
}

impl<'a> Drop for JString<'a> {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            self.env.deallocate(self).unwrap()
        }
    }
}

impl<'a> JDeallocate<'a> for JString<'a> {
    fn as_ptr(&self) -> jmemory {
        self.ptr as jmemory
    }
}