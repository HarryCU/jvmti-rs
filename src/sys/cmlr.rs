use std::os::raw::c_char;
use std::ffi::c_void;
use crate::sys::{jint, jmethodID};

pub const JVMTI_CMLR_MAJOR_VERSION_1: i32 = 1;
pub const JVMTI_CMLR_MINOR_VERSION_0: i32 = 0;

pub const JVMTI_CMLR_MAJOR_VERSION: i32 = 1;
pub const JVMTI_CMLR_MINOR_VERSION: i32 = 0;

pub type jvmtiCMLRKind = u32;

pub const JVMTI_CMLR_DUMMY: jvmtiCMLRKind = 1;
pub const JVMTI_CMLR_INLINE_INFO: jvmtiCMLRKind = 2;

pub type jvmtiCompiledMethodLoadRecordHeader = _jvmtiCompiledMethodLoadRecordHeader;
pub type jvmtiCompiledMethodLoadInlineRecord = _jvmtiCompiledMethodLoadInlineRecord;
pub type jvmtiCompiledMethodLoadDummyRecord = _jvmtiCompiledMethodLoadDummyRecord;
pub type PCStackInfo = _PCStackInfo;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _jvmtiCompiledMethodLoadRecordHeader {
    pub kind: jvmtiCMLRKind,
    pub majorinfoversion: jint,
    pub minorinfoversion: jint,
    pub next: *mut _jvmtiCompiledMethodLoadRecordHeader,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _PCStackInfo {
    pub pc: *mut c_void,
    pub numstackframes: jint,
    pub methods: *mut jmethodID,
    pub bcis: *mut jint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _jvmtiCompiledMethodLoadInlineRecord {
    pub header: jvmtiCompiledMethodLoadRecordHeader,
    pub numpcs: jint,
    pub pcinfo: *mut PCStackInfo,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _jvmtiCompiledMethodLoadDummyRecord {
    pub header: jvmtiCompiledMethodLoadRecordHeader,
    pub message: [c_char; 50usize],
}