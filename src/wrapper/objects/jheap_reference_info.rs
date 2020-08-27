use jni_sys::jint;
use std::marker::PhantomData;
use crate::sys::{jlong, jlocation, jvmtiHeapReferenceInfoReserved, jvmtiHeapReferenceInfoJniLocal, jvmtiHeapReferenceInfoStackLocal};
use crate::wrapper::objects::JMethodID;

#[derive(Debug)]
pub struct JHeapReferenceInfoStackLocal<'a> {
    internal: jvmtiHeapReferenceInfoStackLocal,

    pub thread_tag: jlong,
    pub thread_id: jlong,
    pub depth: jint,
    pub method: JMethodID<'a>,
    pub location: jlocation,
    pub slot: jint,
}

impl<'a> From<jvmtiHeapReferenceInfoStackLocal> for JHeapReferenceInfoStackLocal<'a> {
    fn from(info: jvmtiHeapReferenceInfoStackLocal) -> Self {
        return JHeapReferenceInfoStackLocal {
            internal: info,

            thread_tag: info.thread_tag,
            thread_id: info.thread_id,
            depth: info.depth,
            method: info.method.into(),
            location: info.location,
            slot: info.slot,
        };
    }
}

impl<'a> ::std::ops::Deref for JHeapReferenceInfoStackLocal<'a> {
    type Target = jvmtiHeapReferenceInfoStackLocal;

    fn deref(&self) -> &Self::Target {
        &self.internal
    }
}

#[derive(Debug)]
pub struct JHeapReferenceInfoJniLocal<'a> {
    internal: jvmtiHeapReferenceInfoJniLocal,

    pub thread_tag: jlong,
    pub thread_id: jlong,
    pub depth: jint,
    pub method: JMethodID<'a>,
}

impl<'a> From<jvmtiHeapReferenceInfoJniLocal> for JHeapReferenceInfoJniLocal<'a> {
    fn from(info: jvmtiHeapReferenceInfoJniLocal) -> Self {
        return JHeapReferenceInfoJniLocal {
            internal: info,

            thread_tag: info.thread_tag,
            thread_id: info.thread_id,
            depth: info.depth,
            method: info.method.into(),
        };
    }
}

impl<'a> ::std::ops::Deref for JHeapReferenceInfoJniLocal<'a> {
    type Target = jvmtiHeapReferenceInfoJniLocal;

    fn deref(&self) -> &Self::Target {
        &self.internal
    }
}

#[derive(Debug)]
pub struct JHeapReferenceInfoReserved<'a> {
    internal: jvmtiHeapReferenceInfoReserved,
    lifetime: PhantomData<&'a ()>,

    pub reserved1: jlong,
    pub reserved2: jlong,
    pub reserved3: jlong,
    pub reserved4: jlong,
    pub reserved5: jlong,
    pub reserved6: jlong,
    pub reserved7: jlong,
    pub reserved8: jlong,
}

impl<'a> From<jvmtiHeapReferenceInfoReserved> for JHeapReferenceInfoReserved<'a> {
    fn from(info: jvmtiHeapReferenceInfoReserved) -> Self {
        return JHeapReferenceInfoReserved {
            internal: info,
            lifetime: PhantomData,

            reserved1: info.reserved1,
            reserved2: info.reserved2,
            reserved3: info.reserved3,
            reserved4: info.reserved4,
            reserved5: info.reserved5,
            reserved6: info.reserved6,
            reserved7: info.reserved7,
            reserved8: info.reserved8,
        };
    }
}

impl<'a> ::std::ops::Deref for JHeapReferenceInfoReserved<'a> {
    type Target = jvmtiHeapReferenceInfoReserved;

    fn deref(&self) -> &Self::Target {
        &self.internal
    }
}